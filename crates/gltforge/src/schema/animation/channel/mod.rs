pub mod target;

// -------------------------------------------------------------------------- //

pub use target::Target;

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Channel";

/// An animation channel combining an animation sampler with a target property.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    /// The index of a sampler in this animation used to compute the value for the target.
    pub sampler: GltfId,

    /// The descriptor of the animated property.
    pub target: Target,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
