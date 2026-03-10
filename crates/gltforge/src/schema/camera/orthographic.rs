use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "CameraOrthographic";

/// An orthographic camera containing properties to create an orthographic projection matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orthographic {
    /// The floating-point horizontal magnification of the view. MUST NOT be zero.
    pub xmag: f32,

    /// The floating-point vertical magnification of the view. MUST NOT be zero.
    pub ymag: f32,

    /// The floating-point distance to the far clipping plane. MUST be greater than `znear`.
    pub zfar: f32,

    /// The floating-point distance to the near clipping plane.
    pub znear: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
