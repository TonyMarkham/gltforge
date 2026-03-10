pub mod accessor;
pub mod animation;
pub mod asset;
pub mod buffer;
pub mod buffer_view;
pub mod camera;
pub mod extensions;
pub mod extras;
pub mod gltf;
pub mod gltf_id;
pub mod image;
pub mod material;
pub mod mesh;
pub mod node;
pub mod sampler;
pub mod scene;
pub mod skin;
pub mod texture;

pub const TYPE_NAME: &str = "Schema";

// -------------------------------------------------------------------------- //

pub use accessor::{
    Accessor, AccessorComponentType, AccessorType,
    sparse::{AccessorSparseIndices, AccessorSparseValues, Sparse as AccessorSparse},
};
pub use animation::{
    Animation, AnimationChannel, AnimationChannelTarget, AnimationInterpolation, AnimationPath,
    AnimationSampler,
};
pub use asset::Asset;
pub use buffer::Buffer;
pub use buffer_view::{BufferView, BufferViewTarget};
pub use camera::{Camera, CameraOrthographic, CameraPerspective, CameraType};
pub use extensions::Extensions;
pub use extras::Extras;
pub use gltf::Gltf;
pub use gltf_id::GltfId;
pub use image::Image;
pub use material::{
    Material, MaterialAlphaMode, MaterialNormalTextureInfo, MaterialOcclusionTextureInfo,
    MaterialPbrMetallicRoughness,
};
pub use mesh::{Mesh, MeshPrimitive, MeshPrimitiveMode};
pub use node::Node;
pub use sampler::{Sampler, SamplerMagFilter, SamplerMinFilter, SamplerWrapMode};
pub use scene::Scene;
pub use skin::Skin;
pub use texture::{Texture, TextureInfo};
