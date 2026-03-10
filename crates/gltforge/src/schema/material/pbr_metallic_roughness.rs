use crate::schema::{Extensions, Extras, TextureInfo};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "MaterialPbrMetallicRoughness";

/// PBR metallic-roughness material model parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    /// RGBA factors for the base color of the material. Default: `[1.0, 1.0, 1.0, 1.0]`.
    #[serde(default = "default_base_color_factor")]
    pub base_color_factor: [f32; 4],

    /// The base color texture. RGB components are sRGB-encoded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_color_texture: Option<TextureInfo>,

    /// The factor for the metalness of the material. Range [0.0, 1.0]. Default: 1.0.
    #[serde(default = "default_one_f32")]
    pub metallic_factor: f32,

    /// The factor for the roughness of the material. Range [0.0, 1.0]. Default: 1.0.
    #[serde(default = "default_one_f32")]
    pub roughness_factor: f32,

    /// The metallic-roughness texture. Metalness from B channel, roughness from G channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metallic_roughness_texture: Option<TextureInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}

pub fn default_base_color_factor() -> [f32; 4] {
    [1.0, 1.0, 1.0, 1.0]
}

pub fn default_one_f32() -> f32 {
    1.0
}
