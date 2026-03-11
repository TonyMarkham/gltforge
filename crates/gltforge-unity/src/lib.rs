pub mod convert;
pub mod error;
pub(crate) mod gltf_primitive_data;
pub mod unity_gltf;
pub mod unity_image;
pub mod unity_indices;
pub mod unity_mesh;
pub mod unity_node;
pub mod unity_node_transform;
pub mod unity_pbr_metallic_roughness;
pub mod unity_scene;
pub mod unity_submesh;

// -------------------------------------------------------------------------- //

pub use unity_gltf::{UnityGltf, gltforge_load, gltforge_release, gltforge_retain};
pub use unity_image::{
    UnityImage, gltforge_image_bytes, gltforge_image_count, gltforge_image_name, gltforge_image_uri,
};
pub use unity_indices::UnityIndices;
pub use unity_mesh::{
    UnityMesh, gltforge_mesh_count, gltforge_mesh_index_format, gltforge_mesh_name,
    gltforge_mesh_normals, gltforge_mesh_positions, gltforge_mesh_tangents,
    gltforge_mesh_uv_channel_count, gltforge_mesh_uvs, gltforge_mesh_vertex_count,
};
pub use unity_node::{
    UnityNode, gltforge_node_child, gltforge_node_child_count, gltforge_node_count,
    gltforge_node_mesh_count, gltforge_node_mesh_index, gltforge_node_name,
    gltforge_node_transform,
};
pub use unity_node_transform::UnityNodeTransform;
pub use unity_pbr_metallic_roughness::{
    ALPHA_MODE_BLEND, ALPHA_MODE_MASK, ALPHA_MODE_OPAQUE, UnityPbrMetallicRoughness,
    gltforge_pbr_metallic_roughness_alpha_cutoff, gltforge_pbr_metallic_roughness_alpha_mode,
    gltforge_pbr_metallic_roughness_base_color_factor,
    gltforge_pbr_metallic_roughness_base_color_texture, gltforge_pbr_metallic_roughness_count,
    gltforge_pbr_metallic_roughness_cull, gltforge_pbr_metallic_roughness_emissive_factor,
    gltforge_pbr_metallic_roughness_emissive_texture,
    gltforge_pbr_metallic_roughness_metallic_factor,
    gltforge_pbr_metallic_roughness_metallic_roughness_texture,
    gltforge_pbr_metallic_roughness_name, gltforge_pbr_metallic_roughness_normal_scale,
    gltforge_pbr_metallic_roughness_normal_texture,
    gltforge_pbr_metallic_roughness_occlusion_strength,
    gltforge_pbr_metallic_roughness_occlusion_texture,
    gltforge_pbr_metallic_roughness_roughness_factor,
};
pub use unity_scene::{gltforge_root_node_count, gltforge_root_node_index, gltforge_scene_name};
pub use unity_submesh::{
    UnitySubMesh, gltforge_mesh_submesh_count, gltforge_mesh_submesh_indices_u16,
    gltforge_mesh_submesh_indices_u32, gltforge_mesh_submesh_material,
};

// -------------------------------------------------------------------------- //

/// Returns null + sets `out_len = 0` when a name is absent,
/// otherwise returns a pointer to the UTF-8 bytes and sets `out_len`.
unsafe fn write_name(name: Option<&String>, out_len: *mut u32) -> *const u8 {
    match name {
        Some(s) => {
            if !out_len.is_null() {
                unsafe { *out_len = s.len() as u32 };
            }
            s.as_ptr()
        }
        None => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}
