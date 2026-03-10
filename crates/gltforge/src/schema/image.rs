use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Image";

/// Image data used to create a texture.
///
/// Exactly one of `uri` or `buffer_view` MUST be defined.
/// When `buffer_view` is defined, `mime_type` is required.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    /// The URI (or IRI) of the image. Mutually exclusive with `buffer_view`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// The image's MIME type. Required when `buffer_view` is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,

    /// The index of the buffer view containing the image data.
    /// Mutually exclusive with `uri`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<GltfId>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
