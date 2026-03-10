use error_location::ErrorLocation;
use gltforge::schema::{AccessorComponentType, MeshPrimitiveMode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertError {
    #[error("no nodes in document {location}")]
    NoNodes { location: ErrorLocation },

    #[error("node index {index} out of range {location}")]
    NodeIndexOutOfRange {
        index: usize,
        location: ErrorLocation,
    },

    #[error("node {index} has no mesh {location}")]
    NodeHasNoMesh {
        index: usize,
        location: ErrorLocation,
    },

    #[error("no meshes in document {location}")]
    NoMeshes { location: ErrorLocation },

    #[error("mesh index {index} out of range {location}")]
    MeshIndexOutOfRange {
        index: usize,
        location: ErrorLocation,
    },

    #[error("unsupported primitive mode {mode:?} {location}")]
    UnsupportedPrimitiveMode {
        mode: MeshPrimitiveMode,
        location: ErrorLocation,
    },

    #[error("no POSITION attribute on primitive {location}")]
    NoPositionAttribute { location: ErrorLocation },

    #[error("POSITION accessor index out of range {location}")]
    PositionAccessorOutOfRange { location: ErrorLocation },

    #[error("POSITION accessor must be VEC3 FLOAT {location}")]
    InvalidPositionType { location: ErrorLocation },

    #[error("primitive has no index accessor {location}")]
    NoIndices { location: ErrorLocation },

    #[error("index accessor index out of range {location}")]
    IndexAccessorOutOfRange { location: ErrorLocation },

    #[error("unsupported index component type {component_type:?} {location}")]
    UnsupportedIndexComponentType {
        component_type: AccessorComponentType,
        location: ErrorLocation,
    },

    #[error("accessor resolution failed: {source} {location}")]
    Resolve {
        #[source]
        source: gltforge::error::ParseError,
        location: ErrorLocation,
    },
}

pub type ConvertResult<T> = std::result::Result<T, ConvertError>;
