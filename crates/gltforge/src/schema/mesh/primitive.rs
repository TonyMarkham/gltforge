use crate::schema::{Extensions, Extras, GltfId, MeshPrimitiveMode};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const TYPE_NAME: &str = "MeshPrimitive";

/// Geometry to be rendered with the given material.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Primitive {
    /// Maps attribute semantic names (e.g., `"POSITION"`) to accessor indices.
    pub attributes: HashMap<String, GltfId>,

    /// The index of the accessor containing vertex indices. When absent, non-indexed geometry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<GltfId>,

    /// The index of the material to apply when rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<GltfId>,

    /// The topology type of primitives to render. Default: `TRIANGLES`.
    #[serde(default)]
    pub mode: MeshPrimitiveMode,

    /// An array of morph targets, each mapping attribute semantics to accessor indices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<HashMap<String, GltfId>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
