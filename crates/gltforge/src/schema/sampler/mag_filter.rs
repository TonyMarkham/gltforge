use crate::error::{SchemaError, SchemaResult};

use error_location::ErrorLocation;
use serde::{Deserialize, Serialize};
use std::panic::Location;

pub const TYPE_NAME: &str = "SamplerMagFilter";

/// Magnification filter applied when a texel maps to less than one pixel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub enum MagFilter {
    /// `NEAREST` (9728) — returns the value of the nearest texel.
    Nearest,
    /// `LINEAR` (9729) — returns the weighted average of the four nearest texels.
    Linear,
}

impl TryFrom<u32> for MagFilter {
    type Error = SchemaError;

    #[track_caller]
    fn try_from(v: u32) -> SchemaResult<Self> {
        match v {
            9728 => Ok(Self::Nearest),
            9729 => Ok(Self::Linear),
            _ => Err(SchemaError::InvalidValue {
                type_name: TYPE_NAME,
                value: v,
                location: ErrorLocation::from(Location::caller()),
            }),
        }
    }
}

impl From<MagFilter> for u32 {
    fn from(v: MagFilter) -> u32 {
        match v {
            MagFilter::Nearest => 9728,
            MagFilter::Linear => 9729,
        }
    }
}
