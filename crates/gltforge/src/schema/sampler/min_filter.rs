use crate::error::{SchemaError, SchemaResult};

use error_location::ErrorLocation;
use serde::{Deserialize, Serialize};
use std::panic::Location;

pub const TYPE_NAME: &str = "SamplerMinFilter";

/// Minification filter applied when a texel maps to more than one pixel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub enum MinFilter {
    /// `NEAREST` (9728)
    Nearest,
    /// `LINEAR` (9729)
    Linear,
    /// `NEAREST_MIPMAP_NEAREST` (9984)
    NearestMipmapNearest,
    /// `LINEAR_MIPMAP_NEAREST` (9985)
    LinearMipmapNearest,
    /// `NEAREST_MIPMAP_LINEAR` (9986)
    NearestMipmapLinear,
    /// `LINEAR_MIPMAP_LINEAR` (9987)
    LinearMipmapLinear,
}

impl TryFrom<u32> for MinFilter {
    type Error = SchemaError;

    #[track_caller]
    fn try_from(v: u32) -> SchemaResult<Self> {
        match v {
            9728 => Ok(Self::Nearest),
            9729 => Ok(Self::Linear),
            9984 => Ok(Self::NearestMipmapNearest),
            9985 => Ok(Self::LinearMipmapNearest),
            9986 => Ok(Self::NearestMipmapLinear),
            9987 => Ok(Self::LinearMipmapLinear),
            _ => Err(SchemaError::InvalidValue {
                type_name: TYPE_NAME,
                value: v,
                location: ErrorLocation::from(Location::caller()),
            }),
        }
    }
}

impl From<MinFilter> for u32 {
    fn from(v: MinFilter) -> u32 {
        match v {
            MinFilter::Nearest => 9728,
            MinFilter::Linear => 9729,
            MinFilter::NearestMipmapNearest => 9984,
            MinFilter::LinearMipmapNearest => 9985,
            MinFilter::NearestMipmapLinear => 9986,
            MinFilter::LinearMipmapLinear => 9987,
        }
    }
}
