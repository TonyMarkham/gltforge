using System;
using System.Runtime.InteropServices;

namespace Gltforge
{
    public static class GltforgeNative
    {
        const string Lib = "gltforge_unity";

        // ---- load / retain / release ----------------------------------------

        [DllImport(Lib)] public static extern IntPtr gltforge_load(string path);
        [DllImport(Lib)] public static extern void   gltforge_retain(IntPtr handle);
        [DllImport(Lib)] public static extern void   gltforge_release(IntPtr handle);

        // ---- scene ----------------------------------------------------------

        [DllImport(Lib)] public static extern IntPtr gltforge_scene_name(IntPtr handle, out uint len);
        [DllImport(Lib)] public static extern uint   gltforge_root_node_count(IntPtr handle);
        [DllImport(Lib)] public static extern uint   gltforge_root_node_index(IntPtr handle, uint slot);

        // ---- nodes ----------------------------------------------------------

        [DllImport(Lib)] public static extern uint   gltforge_node_count(IntPtr handle);
        [DllImport(Lib)] public static extern IntPtr gltforge_node_name(IntPtr handle, uint nodeIdx, out uint len);
        [DllImport(Lib)] public static extern uint   gltforge_node_child_count(IntPtr handle, uint nodeIdx);
        [DllImport(Lib)] public static extern uint   gltforge_node_child(IntPtr handle, uint nodeIdx, uint slot);
        [DllImport(Lib)] public static extern uint   gltforge_node_mesh_count(IntPtr handle, uint nodeIdx);
        [DllImport(Lib)] public static extern uint   gltforge_node_mesh_index(IntPtr handle, uint nodeIdx, uint slot);
        [DllImport(Lib)] public static extern void   gltforge_node_transform(IntPtr handle, uint nodeIdx, IntPtr transform);

        // ---- meshes ---------------------------------------------------------

        [DllImport(Lib)] public static extern uint   gltforge_mesh_count(IntPtr handle);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_name(IntPtr handle, uint meshIdx, out uint len);
        [DllImport(Lib)] public static extern uint   gltforge_mesh_vertex_count(IntPtr handle, uint meshIdx);
        [DllImport(Lib)] public static extern uint   gltforge_mesh_index_format(IntPtr handle, uint meshIdx);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_positions(IntPtr handle, uint meshIdx, out uint len);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_normals(IntPtr handle, uint meshIdx, out uint len);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_tangents(IntPtr handle, uint meshIdx, out uint len);
        [DllImport(Lib)] public static extern uint   gltforge_mesh_uv_channel_count(IntPtr handle, uint meshIdx);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_uvs(IntPtr handle, uint meshIdx, uint channel, out uint len);

        // ---- sub-meshes -----------------------------------------------------

        [DllImport(Lib)] public static extern uint  gltforge_mesh_submesh_count(IntPtr handle, uint meshIdx);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_submesh_indices_u16(IntPtr handle, uint meshIdx, uint submesh, out uint len);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_submesh_indices_u32(IntPtr handle, uint meshIdx, uint submesh, out uint len);
        [DllImport(Lib)] public static extern int   gltforge_mesh_submesh_material(IntPtr handle, uint meshIdx, uint submesh);

        // ---- images ---------------------------------------------------------

        [DllImport(Lib)] public static extern uint   gltforge_image_count(IntPtr handle);
        [DllImport(Lib)] public static extern IntPtr gltforge_image_name(IntPtr handle, uint imageIdx, out uint len);
        [DllImport(Lib)] public static extern IntPtr gltforge_image_uri(IntPtr handle, uint imageIdx, out uint len);
        /// <summary>Raw encoded bytes (PNG, JPEG, …) for buffer-view-embedded images. Null for URI-based images.</summary>
        [DllImport(Lib)] public static extern IntPtr gltforge_image_bytes(IntPtr handle, uint imageIdx, out uint len);

        // ---- GLTF/PbrMetallicRoughness materials ----------------------------

        [DllImport(Lib)] public static extern uint   gltforge_pbr_metallic_roughness_count(IntPtr handle);
        [DllImport(Lib)] public static extern IntPtr gltforge_pbr_metallic_roughness_name(IntPtr handle, uint matIdx, out uint len);

        // Texture slots — image index, or -1 if absent.
        [DllImport(Lib)] public static extern int gltforge_pbr_metallic_roughness_base_color_texture(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern int gltforge_pbr_metallic_roughness_metallic_roughness_texture(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern int gltforge_pbr_metallic_roughness_normal_texture(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern int gltforge_pbr_metallic_roughness_occlusion_texture(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern int gltforge_pbr_metallic_roughness_emissive_texture(IntPtr handle, uint matIdx);

        // Scalar factors.
        [DllImport(Lib)] public static extern void  gltforge_pbr_metallic_roughness_base_color_factor(IntPtr handle, uint matIdx, IntPtr outFloats);
        [DllImport(Lib)] public static extern float gltforge_pbr_metallic_roughness_metallic_factor(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern float gltforge_pbr_metallic_roughness_roughness_factor(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern float gltforge_pbr_metallic_roughness_normal_scale(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern float gltforge_pbr_metallic_roughness_occlusion_strength(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern void  gltforge_pbr_metallic_roughness_emissive_factor(IntPtr handle, uint matIdx, IntPtr outFloats);

        // Rendering mode.
        [DllImport(Lib)] public static extern float gltforge_pbr_metallic_roughness_alpha_cutoff(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern uint  gltforge_pbr_metallic_roughness_alpha_mode(IntPtr handle, uint matIdx);
        [DllImport(Lib)] public static extern uint  gltforge_pbr_metallic_roughness_cull(IntPtr handle, uint matIdx);

        // ---- helpers --------------------------------------------------------

        /// <summary>
        /// Marshals a UTF-8 name pointer returned by the native library.
        /// Returns null if ptr is zero (name was absent in the glTF).
        /// </summary>
        public static string ReadName(IntPtr ptr, uint len)
        {
            if (ptr == IntPtr.Zero) return null;
            return Marshal.PtrToStringUTF8(ptr, (int)len);
        }
    }
}
