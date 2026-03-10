pub mod alpha_mode;
pub mod normal_texture_info;
pub mod occlusion_texture_info;
pub mod pbr_metallic_roughness;

// -------------------------------------------------------------------------- //

pub use alpha_mode::{AlphaMode as MaterialAlphaMode, default_alpha_cutoff};
pub use normal_texture_info::{
    NormalTextureInfo as MaterialNormalTextureInfo, default_normal_scale,
};
pub use occlusion_texture_info::{
    OcclusionTextureInfo as MaterialOcclusionTextureInfo, default_occlusion_strength,
};
pub use pbr_metallic_roughness::{
    PbrMetallicRoughness as MaterialPbrMetallicRoughness, default_base_color_factor,
    default_one_f32,
};

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras, TextureInfo};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Material";

/// The material appearance of a primitive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    /// PBR metallic-roughness material model parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pbr_metallic_roughness: Option<MaterialPbrMetallicRoughness>,

    /// A tangent-space normal map.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal_texture: Option<MaterialNormalTextureInfo>,

    /// An occlusion map texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occlusion_texture: Option<MaterialOcclusionTextureInfo>,

    /// An emissive map texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emissive_texture: Option<TextureInfo>,

    /// RGB emissive color factors. Default: `[0.0, 0.0, 0.0]`.
    #[serde(default)]
    pub emissive_factor: [f32; 3],

    /// The alpha rendering mode. Default: `OPAQUE`.
    #[serde(default)]
    pub alpha_mode: MaterialAlphaMode,

    /// The alpha cutoff value used in `MASK` mode. Default: 0.5.
    #[serde(default = "default_alpha_cutoff")]
    pub alpha_cutoff: f32,

    /// Whether the material is double-sided. Default: false.
    #[serde(default)]
    pub double_sided: bool,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
