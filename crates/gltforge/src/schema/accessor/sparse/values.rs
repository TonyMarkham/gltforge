use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "AccessorSparseValues";

/// An object pointing to a buffer view containing the deviating accessor values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Values {
    /// The index of the buffer view with sparse values.
    pub buffer_view: GltfId,

    /// The offset relative to the start of the buffer view in bytes.
    #[serde(default)]
    pub byte_offset: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
