pub mod channel;
pub mod interpolation;
pub mod path;
pub mod sampler;

// -------------------------------------------------------------------------- //

pub use channel::{Channel as AnimationChannel, Target as AnimationChannelTarget};
pub use interpolation::Interpolation as AnimationInterpolation;
pub use path::Path as AnimationPath;
pub use sampler::Sampler as AnimationSampler;

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Animation";

/// A keyframe animation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    /// An array of animation channels. Different channels MUST NOT target the same property.
    pub channels: Vec<AnimationChannel>,

    /// An array of animation samplers combining timestamps with output values.
    pub samplers: Vec<AnimationSampler>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
