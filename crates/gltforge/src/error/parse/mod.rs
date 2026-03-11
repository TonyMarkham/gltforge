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

    #[error(
        "GLB-embedded buffer used without GLB context — call parse_glb instead of parse + load_buffers {location}"
    )]
    GlbBufferNotSupported { location: ErrorLocation },

    #[error("data URI buffers are not yet supported {location}")]
    DataUriNotSupported { location: ErrorLocation },

    #[error("file is too short to be a valid GLB container {location}")]
    GlbTooShort { location: ErrorLocation },

    #[error("not a GLB file: magic bytes do not match 'glTF' {location}")]
    GlbBadMagic { location: ErrorLocation },

    #[error("unsupported GLB version {version}; only version 2 is supported {location}")]
    GlbUnsupportedVersion {
        version: u32,
        location: ErrorLocation,
    },

    #[error("GLB JSON chunk is missing {location}")]
    GlbMissingJsonChunk { location: ErrorLocation },

    #[error("GLB chunk at offset {offset} extends past end of file {location}")]
    GlbChunkOutOfBounds {
        offset: usize,
        location: ErrorLocation,
    },

    #[error("GLB JSON chunk is not valid UTF-8 {location}")]
    GlbJsonInvalidUtf8 { location: ErrorLocation },
}

pub type Result<T> = std::result::Result<T, ParseError>;
