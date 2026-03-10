use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "MaterialOcclusionTextureInfo";

/// Material occlusion texture reference with an additional strength parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcclusionTextureInfo {
    /// The index of the texture.
    pub index: u32,

    /// The TEXCOORD set index used for texture coordinate mapping. Default: 0.
    #[serde(default)]
    pub tex_coord: u32,

    /// Scalar multiplier controlling the amount of occlusion applied. Range [0.0, 1.0]. Default: 1.0.
    #[serde(default = "default_occlusion_strength")]
    pub strength: f32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}

pub fn default_occlusion_strength() -> f32 {
    1.0
}
