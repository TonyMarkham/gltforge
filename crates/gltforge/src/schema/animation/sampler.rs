use crate::schema::{AnimationInterpolation, Extensions, Extras, GltfId};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "AnimationSampler";

/// An animation sampler combining timestamps with output values and an interpolation algorithm.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sampler {
    /// The index of an accessor containing keyframe timestamps (scalar floats, strictly increasing).
    pub input: GltfId,

    /// The index of an accessor containing keyframe output values.
    pub output: GltfId,

    /// Interpolation algorithm. Default: `LINEAR`.
    #[serde(default)]
    pub interpolation: AnimationInterpolation,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
