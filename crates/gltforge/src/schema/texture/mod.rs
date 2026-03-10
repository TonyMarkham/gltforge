pub mod info;

// -------------------------------------------------------------------------- //

pub use info::Info as TextureInfo;

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Texture";

/// A texture and its sampler.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Texture {
    /// The index of the sampler used by this texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<GltfId>,

    /// The index of the image used by this texture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<GltfId>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
