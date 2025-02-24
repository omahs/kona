//! Mock testing utilities for the [ChannelBank] stage.

use crate::{
    stages::ChannelBankProvider,
    traits::OriginProvider,
    types::{BlockInfo, Frame, StageError, StageResult},
};
use alloc::{boxed::Box, vec::Vec};
use async_trait::async_trait;

/// A mock [ChannelBankProvider] for testing the [ChannelBank] stage.
#[derive(Debug)]
pub struct MockChannelBankProvider {
    /// The data to return.
    pub data: Vec<StageResult<Frame>>,
    /// The block info
    pub block_info: Option<BlockInfo>,
}

impl MockChannelBankProvider {
    /// Creates a new [MockChannelBankProvider] with the given data.
    pub fn new(data: Vec<StageResult<Frame>>) -> Self {
        Self { data, block_info: Some(BlockInfo::default()) }
    }
}

impl OriginProvider for MockChannelBankProvider {
    fn origin(&self) -> Option<&BlockInfo> {
        self.block_info.as_ref()
    }
}

#[async_trait]
impl ChannelBankProvider for MockChannelBankProvider {
    async fn next_frame(&mut self) -> StageResult<Frame> {
        self.data.pop().unwrap_or(Err(StageError::Eof))
    }
}
