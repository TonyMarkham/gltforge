use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "CameraType";

/// Specifies whether the camera uses a perspective or orthographic projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CameraType {
    Perspective,
    Orthographic,
}
