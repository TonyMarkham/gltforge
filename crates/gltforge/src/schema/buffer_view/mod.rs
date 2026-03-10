pub mod target;

// -------------------------------------------------------------------------- //

pub use target::Target as BufferViewTarget;

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "BufferView";

/// A view into a buffer, generally representing a subset of the buffer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferView {
    /// The index of the buffer.
    pub buffer: GltfId,

    /// The length of the buffer view in bytes.
    pub byte_length: u32,

    /// The offset into the buffer in bytes.
    #[serde(default)]
    pub byte_offset: u32,

    /// The stride in bytes between vertex attributes. Must be a multiple of 4, range [4, 252].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_stride: Option<u32>,

    /// The intended GPU buffer type for this buffer view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<BufferViewTarget>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
