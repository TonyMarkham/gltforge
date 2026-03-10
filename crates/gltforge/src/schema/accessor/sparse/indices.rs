use crate::schema::{AccessorComponentType, Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "AccessorSparseIndices";

/// An object pointing to a buffer view containing the indices of deviating accessor values.
/// Indices MUST strictly increase.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indices {
    /// The index of the buffer view with sparse indices.
    pub buffer_view: GltfId,

    /// The data type of the sparse indices.
    pub component_type: AccessorComponentType,

    /// The offset relative to the start of the buffer view in bytes.
    #[serde(default)]
    pub byte_offset: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
