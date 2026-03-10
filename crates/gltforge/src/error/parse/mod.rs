use error_location::ErrorLocation;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("JSON: {source} {location}")]
    Json {
        #[source]
        source: serde_json::Error,
        location: ErrorLocation,
    },

    #[error("I/O: {source} {location}")]
    Io {
        #[source]
        source: std::io::Error,
        location: ErrorLocation,
    },

    #[error("buffer index {index} out of range {location}")]
    BufferIndexOutOfRange {
        index: usize,
        location: ErrorLocation,
    },

    #[error("buffer view index {index} out of range {location}")]
    BufferViewIndexOutOfRange {
        index: usize,
        location: ErrorLocation,
    },

    #[error("accessor has no buffer view {location}")]
    AccessorNoBufferView { location: ErrorLocation },

    #[error("accessor byte range [{start}, {end}) exceeds buffer length {len} {location}")]
    AccessorOutOfBounds {
        start: usize,
        end: usize,
        len: usize,
        location: ErrorLocation,
    },

    #[error("GLB-embedded buffers (no URI) are not yet supported {location}")]
    GlbBufferNotSupported { location: ErrorLocation },

    #[error("data URI buffers are not yet supported {location}")]
    DataUriNotSupported { location: ErrorLocation },
}

pub type Result<T> = std::result::Result<T, ParseError>;
