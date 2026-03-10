use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Node";

/// A node in the glTF scene hierarchy.
///
/// The transform is defined by either a `matrix` or the TRS properties
/// (`translation`, `rotation`, `scale`). These are mutually exclusive.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// The indices of this node's children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<GltfId>>,

    /// The index of the mesh in this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mesh: Option<GltfId>,

    /// The index of the skin referenced by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skin: Option<GltfId>,

    /// The index of the camera referenced by this node.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub camera: Option<GltfId>,

    /// A 4×4 column-major transformation matrix. Mutually exclusive with TRS properties.
    /// Default is the identity matrix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matrix: Option<[f32; 16]>,

    /// The node's translation along the x, y, and z axes. Default: `[0.0, 0.0, 0.0]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<[f32; 3]>,

    /// The node's rotation as a unit quaternion `[x, y, z, w]`. Default: `[0.0, 0.0, 0.0, 1.0]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<[f32; 4]>,

    /// The node's scale along the x, y, and z axes. Default: `[1.0, 1.0, 1.0]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f32; 3]>,

    /// The weights of the instantiated morph target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
