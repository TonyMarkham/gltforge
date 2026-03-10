pub mod accessor_type;
pub mod component_type;
pub mod sparse;

// -------------------------------------------------------------------------- //

pub use accessor_type::Type as AccessorType;
pub use component_type::ComponentType as AccessorComponentType;
pub use sparse::{AccessorSparseIndices, AccessorSparseValues, Sparse};

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Accessor";

/// A typed view into a buffer view containing raw binary data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accessor {
    /// The data type of the accessor's components.
    pub component_type: AccessorComponentType,

    /// The number of elements referenced by this accessor.
    pub count: u32,

    /// Specifies whether the element is a scalar, vector, or matrix.
    #[serde(rename = "type")]
    pub accessor_type: AccessorType,

    /// The index of the buffer view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<GltfId>,

    /// The offset relative to the start of the buffer view in bytes.
    /// Only valid when `buffer_view` is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_offset: Option<u32>,

    /// Specifies whether integer data values are normalized on access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<bool>,

    /// Maximum value of each component in this accessor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<Vec<f64>>,

    /// Minimum value of each component in this accessor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<Vec<f64>>,

    /// Sparse storage of elements that deviate from their initialization value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse: Option<Sparse>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
