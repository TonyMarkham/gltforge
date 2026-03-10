use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "TextureInfo";

/// Reference to a texture and its texture coordinate set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The index of the texture.
    pub index: GltfId,

    /// The TEXCOORD set index used for texture coordinate mapping. Default: 0.
    #[serde(default)]
    pub tex_coord: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
