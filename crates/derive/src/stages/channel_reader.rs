//! This module contains the `ChannelReader` struct.

use crate::{
    stages::BatchQueueProvider,
    traits::{LogLevel, OriginProvider, TelemetryProvider},
    types::{Batch, BlockInfo, StageError, StageResult},
};

use alloc::{boxed::Box, sync::Arc, vec::Vec};
use alloy_primitives::Bytes;
use async_trait::async_trait;
use core::fmt::Debug;
use miniz_oxide::inflate::decompress_to_vec_zlib;

/// The [ChannelReader] provider trait.
#[async_trait]
pub trait ChannelReaderProvider {
    /// Pulls the next piece of data from the channel bank. Note that it attempts to pull data out
    /// of the channel bank prior to loading data in (unlike most other stages). This is to
    /// ensure maintain consistency around channel bank pruning which depends upon the order
    /// of operations.
    async fn next_data(&mut self) -> StageResult<Option<Bytes>>;
}

/// [ChannelReader] is a stateful stage that does the following:
#[derive(Debug)]
pub struct ChannelReader<P, T>
where
    P: ChannelReaderProvider + OriginProvider + Debug,
    T: TelemetryProvider + Debug,
{
    /// The previous stage of the derivation pipeline.
    prev: P,
    /// Telemetry
    telemetry: Arc<T>,
    /// The batch reader.
    next_batch: Option<BatchReader>,
}

impl<P, T> ChannelReader<P, T>
where
    P: ChannelReaderProvider + OriginProvider + Debug,
    T: TelemetryProvider + Debug,
{
    /// Create a new [ChannelReader] stage.
    pub fn new(prev: P, telemetry: Arc<T>) -> Self {
        Self { prev, telemetry, next_batch: None }
    }

    /// Creates the batch reader from available channel data.
    async fn set_batch_reader(&mut self) -> StageResult<()> {
        if self.next_batch.is_none() {
            let channel = self.prev.next_data().await?.ok_or(StageError::NoChannel)?;
            self.next_batch = Some(BatchReader::from(&channel[..]));
        }
        Ok(())
    }

    /// Forces the read to continue with the next channel, resetting any
    /// decoding / decompression state to a fresh start.
    pub fn next_channel(&mut self) {
        self.next_batch = None;
    }
}

#[async_trait]
impl<P, T> BatchQueueProvider for ChannelReader<P, T>
where
    P: ChannelReaderProvider + OriginProvider + Send + Debug,
    T: TelemetryProvider + Send + Sync + Debug,
{
    async fn next_batch(&mut self) -> StageResult<Batch> {
        if let Err(e) = self.set_batch_reader().await {
            self.telemetry
                .write(alloc::format!("Failed to set batch reader: {:?}", e), LogLevel::Error);
            self.next_channel();
            return Err(e);
        }
        match self
            .next_batch
            .as_mut()
            .expect("Cannot be None")
            .next_batch()
            .ok_or(StageError::NotEnoughData)
        {
            Ok(batch) => Ok(batch),
            Err(e) => {
                self.next_channel();
                Err(e)
            }
        }
    }
}

impl<P, T> OriginProvider for ChannelReader<P, T>
where
    P: ChannelReaderProvider + OriginProvider + Debug,
    T: TelemetryProvider + Debug,
{
    fn origin(&self) -> Option<&BlockInfo> {
        self.prev.origin()
    }
}

/// Batch Reader provides a function that iteratively consumes batches from the reader.
/// The L1Inclusion block is also provided at creation time.
/// Warning: the batch reader can read every batch-type.
/// The caller of the batch-reader should filter the results.
#[derive(Debug)]
pub(crate) struct BatchReader {
    /// The raw data to decode.
    data: Option<Vec<u8>>,
    /// Decompressed data.
    decompressed: Vec<u8>,
    /// The current cursor in the `decompressed` data.
    cursor: usize,
}

impl BatchReader {
    /// Pulls out the next batch from the reader.
    pub(crate) fn next_batch(&mut self) -> Option<Batch> {
        // If the data is not already decompressed, decompress it.
        if let Some(data) = self.data.take() {
            let decompressed_data = decompress_to_vec_zlib(&data).ok()?;
            self.decompressed = decompressed_data;
        }

        // Decompress and RLP decode the batch data, before finally decoding the batch itself.
        let mut decompressed_reader = self.decompressed.as_slice();
        let batch = Batch::decode(&mut decompressed_reader).ok()?;

        // Advance the cursor on the reader.
        self.cursor += self.decompressed.len() - decompressed_reader.len();

        Some(batch)
    }
}

impl From<&[u8]> for BatchReader {
    fn from(data: &[u8]) -> Self {
        Self { data: Some(data.to_vec()), decompressed: Vec::new(), cursor: 0 }
    }
}

impl From<Vec<u8>> for BatchReader {
    fn from(data: Vec<u8>) -> Self {
        Self { data: Some(data), decompressed: Vec::new(), cursor: 0 }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        stages::test_utils::MockChannelReaderProvider, traits::test_utils::TestTelemetry,
        types::BatchType,
    };
    use alloc::vec;
    use miniz_oxide::deflate::compress_to_vec_zlib;

    fn new_compressed_batch_data() -> Bytes {
        let raw_data = include_bytes!("../../testdata/raw_batch.hex");
        let mut typed_data = vec![BatchType::Span as u8];
        typed_data.extend_from_slice(raw_data.as_slice());
        compress_to_vec_zlib(typed_data.as_slice(), 5).into()
    }

    #[tokio::test]
    async fn test_next_batch_batch_reader_set_fails() {
        let mock = MockChannelReaderProvider::new(vec![Err(StageError::Eof)]);
        let telemetry = Arc::new(TestTelemetry::new());
        let mut reader = ChannelReader::new(mock, telemetry);
        assert_eq!(reader.next_batch().await, Err(StageError::Eof));
        assert!(reader.next_batch.is_none());
    }

    #[tokio::test]
    async fn test_next_batch_batch_reader_no_data() {
        let mock = MockChannelReaderProvider::new(vec![Ok(None)]);
        let telemetry = Arc::new(TestTelemetry::new());
        let mut reader = ChannelReader::new(mock, telemetry);
        assert_eq!(reader.next_batch().await, Err(StageError::NoChannel));
        assert!(reader.next_batch.is_none());
    }

    #[tokio::test]
    async fn test_next_batch_not_enough_data() {
        let mock = MockChannelReaderProvider::new(vec![Ok(Some(Bytes::default()))]);
        let telemetry = Arc::new(TestTelemetry::new());
        let mut reader = ChannelReader::new(mock, telemetry);
        assert_eq!(reader.next_batch().await, Err(StageError::NotEnoughData));
        assert!(reader.next_batch.is_none());
    }

    #[tokio::test]
    async fn test_next_batch_succeeds() {
        let raw = new_compressed_batch_data();
        let mock = MockChannelReaderProvider::new(vec![Ok(Some(raw))]);
        let telemetry = Arc::new(TestTelemetry::new());
        let mut reader = ChannelReader::new(mock, telemetry);
        let res = reader.next_batch().await.unwrap();
        matches!(res, Batch::Span(_));
        assert!(reader.next_batch.is_some());
    }

    #[test]
    fn test_batch_reader() {
        let raw_data = include_bytes!("../../testdata/raw_batch.hex");
        let mut typed_data = vec![BatchType::Span as u8];
        typed_data.extend_from_slice(raw_data.as_slice());

        let compressed_raw_data = compress_to_vec_zlib(typed_data.as_slice(), 5);
        let mut reader = BatchReader::from(compressed_raw_data);
        reader.next_batch().unwrap();

        assert_eq!(reader.cursor, typed_data.len());
    }
}
