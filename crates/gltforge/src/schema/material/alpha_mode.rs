use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "MaterialAlphaMode";

/// Specifies the alpha rendering mode of the material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AlphaMode {
    /// Alpha value is ignored; rendered fully opaque (default).
    #[default]
    Opaque,
    /// Rendered either fully opaque or transparent based on `alpha_cutoff`.
    Mask,
    /// Alpha value is used to blend with the background.
    Blend,
}

pub fn default_alpha_cutoff() -> f32 {
    0.5
}
