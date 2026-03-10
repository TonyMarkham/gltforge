use crate::schema::{AnimationPath, Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "AnimationChannelTarget";

/// The descriptor of the animated property.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Target {
    /// The name of the node's TRS property to animate, or `"weights"` for morph targets.
    pub path: AnimationPath,

    /// The index of the node to animate. When undefined, defined by an extension.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<GltfId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
