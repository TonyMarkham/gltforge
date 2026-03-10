pub mod camera_type;
pub mod orthographic;
pub mod perspective;

// -------------------------------------------------------------------------- //

pub use camera_type::CameraType;
pub use orthographic::Orthographic as CameraOrthographic;
pub use perspective::Perspective as CameraPerspective;

// -------------------------------------------------------------------------- //

use crate::schema::{Extensions, Extras};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Camera";

/// A camera's projection. A node may reference a camera to place it in the scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Camera {
    /// Specifies if the camera uses perspective or orthographic projection.
    #[serde(rename = "type")]
    pub camera_type: CameraType,

    /// An orthographic camera. MUST NOT be defined when `perspective` is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orthographic: Option<CameraOrthographic>,

    /// A perspective camera. MUST NOT be defined when `orthographic` is defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perspective: Option<CameraPerspective>,

    /// The user-defined name of this object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
