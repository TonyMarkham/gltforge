pub mod primitive;
pub mod primitive_mode;

// -------------------------------------------------------------------------- //

pub use primitive::Primitive as MeshPrimitive;
pub use primitive_mode::PrimitiveMode as MeshPrimitiveMode;

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Mesh";

/// A set of primitives to be rendered with a global transform defined by a referencing node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    /// An array of primitives, each defining geometry to be rendered.
    pub primitives: Vec<MeshPrimitive>,

    /// Array of weights to apply to morph targets.
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
