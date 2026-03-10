use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Buffer";

/// A buffer pointing to binary geometry, animation, or skins.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Buffer {
    /// The length of the buffer in bytes.
    pub byte_length: u32,

    /// The URI (or IRI) of the buffer. Omitted for the GLB binary chunk.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
