pub mod convert;
pub mod error;
pub(crate) mod gltf_primitive_data;
pub mod unity_game_object;
pub mod unity_gltf;
pub mod unity_image;
pub mod unity_mesh;
pub mod unity_pbr_metallic_roughness;
pub mod unity_scene;
pub mod unity_submesh;

// Re-export the unity_indices module path for code that imports via the old module path.
pub mod unity_indices {
    pub use gltforge_unity_core::UnityIndices;
}

// -------------------------------------------------------------------------- //

pub use gltforge_unity_core::{
    ALPHA_MODE_BLEND, ALPHA_MODE_MASK, ALPHA_MODE_OPAQUE, IDENTITY, UnityGameObject, UnityImage,
    UnityIndices, UnityMesh, UnityPbrMetallicRoughness, UnitySubMesh, UnityTransform,
};
pub use unity_game_object::{
    gltforge_game_object_child, gltforge_game_object_child_count, gltforge_game_object_count,
    gltforge_game_object_mesh_count, gltforge_game_object_mesh_index, gltforge_game_object_name,
    gltforge_game_object_transform,
};
pub use unity_gltf::{UnityGltf, gltforge_load, gltforge_release, gltforge_retain};
pub use unity_image::{
    gltforge_image_bytes, gltforge_image_count, gltforge_image_name, gltforge_image_uri,
};
pub use unity_mesh::{
    gltforge_mesh_count, gltforge_mesh_index_format, gltforge_mesh_name, gltforge_mesh_normals,
    gltforge_mesh_positions, gltforge_mesh_tangents, gltforge_mesh_uv_channel_count,
    gltforge_mesh_uvs, gltforge_mesh_vertex_count,
};
pub use unity_pbr_metallic_roughness::{
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
pub use unity_scene::{
    gltforge_root_game_object_count, gltforge_root_game_object_index, gltforge_scene_name,
};
pub use unity_submesh::{
    gltforge_mesh_submesh_count, gltforge_mesh_submesh_indices_u16,
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
