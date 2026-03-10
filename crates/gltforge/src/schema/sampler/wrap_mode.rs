use crate::error::{SchemaError, SchemaResult};

use error_location::ErrorLocation;
use serde::{Deserialize, Serialize};
use std::panic::Location;

pub const TYPE_NAME: &str = "SamplerWrapMode";

/// Texture coordinate wrapping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub enum WrapMode {
    /// `CLAMP_TO_EDGE` (33071)
    ClampToEdge,
    /// `MIRRORED_REPEAT` (33648)
    MirroredRepeat,
    /// `REPEAT` (10497, default)
    #[default]
    Repeat,
}

impl TryFrom<u32> for WrapMode {
    type Error = SchemaError;

    #[track_caller]
    fn try_from(v: u32) -> SchemaResult<Self> {
        match v {
            33071 => Ok(Self::ClampToEdge),
            33648 => Ok(Self::MirroredRepeat),
            10497 => Ok(Self::Repeat),
            _ => Err(SchemaError::InvalidValue {
                type_name: TYPE_NAME,
                value: v,
                location: ErrorLocation::from(Location::caller()),
            }),
        }
    }
}

impl From<WrapMode> for u32 {
    fn from(v: WrapMode) -> u32 {
        match v {
            WrapMode::ClampToEdge => 33071,
            WrapMode::MirroredRepeat => 33648,
            WrapMode::Repeat => 10497,
        }
    }
}
