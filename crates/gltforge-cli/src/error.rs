use std::path::PathBuf;

use error_location::ErrorLocation;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("could not read '{path}': {source} {location}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
        location: ErrorLocation,
    },

    #[error("parse failed: {0}")]
    Parse(#[from] gltforge::error::ParseError),
}

pub type CliResult<T> = std::result::Result<T, CliError>;
