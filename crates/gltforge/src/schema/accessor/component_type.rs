use crate::error::{SchemaError, SchemaResult};

use error_location::ErrorLocation;
use serde::{Deserialize, Serialize};
use std::panic::Location;

const TYPE_NAME: &str = "AccessorComponentType";

/// The data type of an accessor's components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub enum ComponentType {
    /// `BYTE` (5120)
    Byte,
    /// `UNSIGNED_BYTE` (5121)
    UnsignedByte,
    /// `SHORT` (5122)
    Short,
    /// `UNSIGNED_SHORT` (5123)
    UnsignedShort,
    /// `UNSIGNED_INT` (5125)
    UnsignedInt,
    /// `FLOAT` (5126)
    Float,
}

impl TryFrom<u32> for ComponentType {
    type Error = SchemaError;

    #[track_caller]
    fn try_from(v: u32) -> SchemaResult<Self> {
        match v {
            5120 => Ok(Self::Byte),
            5121 => Ok(Self::UnsignedByte),
            5122 => Ok(Self::Short),
            5123 => Ok(Self::UnsignedShort),
            5125 => Ok(Self::UnsignedInt),
            5126 => Ok(Self::Float),
            _ => Err(SchemaError::InvalidValue {
                type_name: TYPE_NAME,
                value: v,
                location: ErrorLocation::from(Location::caller()),
            }),
        }
    }
}

impl From<ComponentType> for u32 {
    fn from(v: ComponentType) -> u32 {
        match v {
            ComponentType::Byte => 5120,
            ComponentType::UnsignedByte => 5121,
            ComponentType::Short => 5122,
            ComponentType::UnsignedShort => 5123,
            ComponentType::UnsignedInt => 5125,
            ComponentType::Float => 5126,
        }
    }
}
