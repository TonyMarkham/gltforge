use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "AnimationInterpolation";

/// Interpolation algorithm used by an animation sampler.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Interpolation {
    /// Linearly interpolated between keyframes. Uses slerp for rotations. Default.
    #[default]
    #[serde(rename = "LINEAR")]
    Linear,
    /// Constant value held until the next keyframe.
    #[serde(rename = "STEP")]
    Step,
    /// Cubic spline with specified tangents. Output count MUST be three times the input count.
    #[serde(rename = "CUBICSPLINE")]
    CubicSpline,
}
