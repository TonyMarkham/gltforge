use error_location::ErrorLocation;
use std::result::Result as StdResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("I/O error writing '{path}': {source} {location}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
        location: ErrorLocation,
    },

    #[error("JSON serialization error: {source} {location}")]
    Json {
        #[source]
        source: serde_json::Error,
        location: ErrorLocation,
    },
}

pub type ExportResult<T> = StdResult<T, ExportError>;
