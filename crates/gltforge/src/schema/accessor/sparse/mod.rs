pub mod indices;
pub mod values;

// -------------------------------------------------------------------------- //

pub use indices::Indices as AccessorSparseIndices;
pub use values::Values as AccessorSparseValues;

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, extras::Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "AccessorSparse";

/// Sparse storage of accessor values that deviate from their initialization value.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sparse {
    /// Number of deviating accessor values stored in the sparse array.
    pub count: u32,

    /// Buffer view containing the indices of deviating accessor values.
    pub indices: AccessorSparseIndices,

    /// Buffer view containing the deviating accessor values.
    pub values: AccessorSparseValues,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
