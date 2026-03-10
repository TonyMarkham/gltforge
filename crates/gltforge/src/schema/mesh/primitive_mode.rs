use crate::error::{SchemaError, SchemaResult};

use error_location::ErrorLocation;
use serde::{Deserialize, Serialize};
use std::panic::Location;

pub const TYPE_NAME: &str = "MeshPrimitiveMode";

/// The topology type of primitives to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub enum PrimitiveMode {
    /// Individual point clouds (0).
    Points,
    /// Individual line segments (1).
    Lines,
    /// Line loop (2).
    LineLoop,
    /// Line strip (3).
    LineStrip,
    /// Individual triangles (4, default).
    #[default]
    Triangles,
    /// Triangle strip (5).
    TriangleStrip,
    /// Triangle fan (6).
    TriangleFan,
}

impl TryFrom<u32> for PrimitiveMode {
    type Error = SchemaError;

    #[track_caller]
    fn try_from(v: u32) -> SchemaResult<Self> {
        match v {
            0 => Ok(Self::Points),
            1 => Ok(Self::Lines),
            2 => Ok(Self::LineLoop),
            3 => Ok(Self::LineStrip),
            4 => Ok(Self::Triangles),
            5 => Ok(Self::TriangleStrip),
            6 => Ok(Self::TriangleFan),
            _ => Err(SchemaError::InvalidValue {
                type_name: TYPE_NAME,
                value: v,
                location: ErrorLocation::from(Location::caller()),
            }),
        }
    }
}

impl From<PrimitiveMode> for u32 {
    fn from(v: PrimitiveMode) -> u32 {
        match v {
            PrimitiveMode::Points => 0,
            PrimitiveMode::Lines => 1,
            PrimitiveMode::LineLoop => 2,
            PrimitiveMode::LineStrip => 3,
            PrimitiveMode::Triangles => 4,
            PrimitiveMode::TriangleStrip => 5,
            PrimitiveMode::TriangleFan => 6,
        }
    }
}
