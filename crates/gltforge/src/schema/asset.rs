use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Asset";

/// Metadata about the glTF asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// The glTF version this asset targets (e.g., `"2.0"`).
    pub version: String,

    /// A copyright message suitable for display to credit the content creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    /// Tool that generated this glTF model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,

    /// The minimum glTF version this asset targets. MUST NOT be greater than `version`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
