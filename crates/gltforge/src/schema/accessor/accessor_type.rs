use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "AccessorType";

/// The dimensionality of an accessor's components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SCALAR")]
    Scalar,
    #[serde(rename = "VEC2")]
    Vec2,
    #[serde(rename = "VEC3")]
    Vec3,
    #[serde(rename = "VEC4")]
    Vec4,
    #[serde(rename = "MAT2")]
    Mat2,
    #[serde(rename = "MAT3")]
    Mat3,
    #[serde(rename = "MAT4")]
    Mat4,
}
