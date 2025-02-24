use crate::{traits::HintWriterClient, PipeHandle};
use alloc::vec;
use anyhow::Result;

/// A [HintWriter] is a high-level interface to the hint pipe. It provides a way to write hints to
/// the host.
#[derive(Debug, Clone, Copy)]
pub struct HintWriter {
    pipe_handle: PipeHandle,
}

impl HintWriter {
    /// Create a new [HintWriter] from a [PipeHandle].
    pub fn new(pipe_handle: PipeHandle) -> Self {
        Self { pipe_handle }
    }
}

impl HintWriterClient for HintWriter {
    /// Write a hint to the host. This will overwrite any existing hint in the pipe, and block until
    /// all data has been written.
    fn write(&self, hint: &str) -> Result<()> {
        // Form the hint into a byte buffer. The format is a 4-byte big-endian length prefix
        // followed by the hint string.
        let mut hint_bytes = vec![0u8; hint.len() + 4];
        hint_bytes[0..4].copy_from_slice(u32::to_be_bytes(hint.len() as u32).as_ref());
        hint_bytes[4..].copy_from_slice(hint.as_bytes());

        // Write the hint to the host.
        self.pipe_handle.write(&hint_bytes)?;

        // Read the hint acknowledgement from the host.
        let mut hint_ack = [0u8; 1];
        self.pipe_handle.read_exact(&mut hint_ack)?;

        Ok(())
    }
}
