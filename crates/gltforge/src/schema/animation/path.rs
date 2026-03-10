use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "AnimationPath";

/// The TRS property or morph weights targeted by an animation channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Path {
    Translation,
    Rotation,
    Scale,
    Weights,
}
