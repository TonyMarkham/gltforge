use crate::error::{SchemaError, SchemaResult};

use error_location::ErrorLocation;
use serde::{Deserialize, Serialize};
use std::panic::Location;

pub const TYPE_NAME: &str = "BufferViewTarget";

/// The intended GPU buffer type to use with a buffer view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub enum Target {
    /// Vertex attribute data — `ARRAY_BUFFER` (34962).
    ArrayBuffer,
    /// Vertex index data — `ELEMENT_ARRAY_BUFFER` (34963).
    ElementArrayBuffer,
}

impl TryFrom<u32> for Target {
    type Error = SchemaError;

    #[track_caller]
    fn try_from(v: u32) -> SchemaResult<Self> {
        match v {
            34962 => Ok(Self::ArrayBuffer),
            34963 => Ok(Self::ElementArrayBuffer),
            _ => Err(SchemaError::InvalidValue {
                type_name: TYPE_NAME,
                value: v,
                location: ErrorLocation::from(Location::caller()),
            }),
        }
    }
}

impl From<Target> for u32 {
    fn from(v: Target) -> u32 {
        match v {
            Target::ArrayBuffer => 34962,
            Target::ElementArrayBuffer => 34963,
        }
    }
}
