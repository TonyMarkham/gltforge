use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "CameraPerspective";

/// A perspective camera containing properties to create a perspective projection matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Perspective {
    /// The floating-point vertical field of view in radians. SHOULD be less than π.
    pub yfov: f32,

    /// The floating-point distance to the near clipping plane.
    pub znear: f32,

    /// The floating-point aspect ratio of the field of view. When undefined, use the viewport ratio.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f32>,

    /// The floating-point distance to the far clipping plane. When undefined, use infinite projection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zfar: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
