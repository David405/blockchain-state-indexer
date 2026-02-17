use std::io;
use std::path::Path;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("failed to read file '{path}': {source}")]
    Io {
        path: String,
        #[source]
        source: io::Error,
    },

    #[error("failed to parse JSON from '{path}': {source}")]
    Json {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

impl IndexerError {
    pub fn io(path: &Path, source: io::Error) -> Self {
        Self::Io {
            path: path.to_string_lossy().to_string(),
            source,
        }
    }

    pub fn json(path: &Path, source: serde_json::Error) -> Self {
        Self::Json {
            path: path.to_string_lossy().to_string(),
            source,
        }
    }
}
