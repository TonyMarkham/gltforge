use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Skin";

/// Joints and matrices defining a skin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    /// Indices of skeleton nodes used as joints in this skin.
    pub joints: Vec<GltfId>,

    /// The index of the accessor containing the floating-point 4×4 inverse-bind matrices.
    /// When undefined, identity matrices are assumed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse_bind_matrices: Option<GltfId>,

    /// The index of the node used as the skeleton root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeleton: Option<GltfId>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
