pub mod mag_filter;
pub mod min_filter;
pub mod wrap_mode;

// -------------------------------------------------------------------------- //

pub use mag_filter::MagFilter as SamplerMagFilter;
pub use min_filter::MinFilter as SamplerMinFilter;
pub use wrap_mode::WrapMode as SamplerWrapMode;

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Sampler";

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sampler {
    /// Magnification filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mag_filter: Option<SamplerMagFilter>,

    /// Minification filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_filter: Option<SamplerMinFilter>,

    /// S (U) wrapping mode. Default: `REPEAT`.
    #[serde(default, rename = "wrapS")]
    pub wrap_s: SamplerWrapMode,

    /// T (V) wrapping mode. Default: `REPEAT`.
    #[serde(default, rename = "wrapT")]
    pub wrap_t: SamplerWrapMode,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
