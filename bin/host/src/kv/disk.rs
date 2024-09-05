//! Contains a concrete implementation of the [KeyValueStore] trait that stores data on disk
//! using [rocksdb].

use super::KeyValueStore;
use anyhow::{anyhow, Result};
use rocksdb::{Options, DB};
use std::path::PathBuf;

/// A simple, synchronous key-value store that stores data on disk.
#[derive(Debug)]
pub struct DiskKeyValueStore {
    data_directory: PathBuf,
    db: DB,
}

impl DiskKeyValueStore {
    /// Create a new [DiskKeyValueStore] with the given data directory.
    pub fn new(data_directory: PathBuf) -> Self {
        let db = DB::open(&Self::get_db_options(), data_directory.as_path())
            .unwrap_or_else(|e| panic!("Failed to open database at {data_directory:?}: {e}"));

        Self { data_directory, db }
    }

    /// Gets the [Options] for the underlying RocksDB instance.
    fn get_db_options() -> Options {
        let mut options = Options::default();
        options.set_compression_type(rocksdb::DBCompressionType::Snappy);
        options.create_if_missing(true);
        options
    }
}

impl KeyValueStore for DiskKeyValueStore {
    fn get(&self, key: alloy_primitives::B256) -> Option<Vec<u8>> {
        self.db.get(*key).ok()?
    }

    fn set(&mut self, key: alloy_primitives::B256, value: Vec<u8>) -> Result<()> {
        self.db.put(*key, value).map_err(|e| anyhow!("Failed to set key-value pair: {}", e))
    }
}

impl Drop for DiskKeyValueStore {
    fn drop(&mut self) {
        let _ = DB::destroy(&Self::get_db_options(), self.data_directory.as_path());
    }
}
