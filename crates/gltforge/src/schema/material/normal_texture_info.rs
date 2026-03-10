use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "MaterialNormalTextureInfo";

/// Material normal texture reference with an additional scale parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NormalTextureInfo {
    /// The index of the texture.
    pub index: u32,

    /// The TEXCOORD set index used for texture coordinate mapping. Default: 0.
    #[serde(default)]
    pub tex_coord: u32,

    /// Scalar multiplier applied to each normal vector of the texture. Default: 1.0.
    #[serde(default = "default_normal_scale")]
    pub scale: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}

pub fn default_normal_scale() -> f32 {
    1.0
}
