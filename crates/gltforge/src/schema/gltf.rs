use crate::schema::{
    Accessor, Animation, Asset, Buffer, BufferView, Camera, Extensions, Extras, GltfId, Image,
    Material, Mesh, Node, Sampler, Scene, Skin, Texture,
};

use serde::{Deserialize, Serialize};

pub const TYPE_NAME: &str = "Gltf";

/// The root object for a glTF 2.0 asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gltf {
    /// Metadata about the glTF asset. Required.
    pub asset: Asset,

    /// The index of the default scene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<GltfId>,

    /// An array of accessors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessors: Option<Vec<Accessor>>,

    /// An array of keyframe animations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations: Option<Vec<Animation>>,

    /// An array of buffers pointing to binary geometry, animation, or skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<Vec<Buffer>>,

    /// An array of buffer views.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_views: Option<Vec<BufferView>>,

    /// An array of cameras.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cameras: Option<Vec<Camera>>,

    /// An array of images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,

    /// An array of materials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<Vec<Material>>,

    /// An array of meshes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meshes: Option<Vec<Mesh>>,

    /// An array of nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,

    /// An array of texture samplers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samplers: Option<Vec<Sampler>>,

    /// An array of scenes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<Vec<Scene>>,

    /// An array of skins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins: Option<Vec<Skin>>,

    /// An array of textures.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<Vec<Texture>>,

    /// Names of glTF extensions used in this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_used: Option<Vec<String>>,

    /// Names of glTF extensions required to correctly load and render this asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions_required: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Extras>,
}
