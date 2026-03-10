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

        [DllImport(Lib)] public static extern uint   gltforge_mesh_submesh_count(IntPtr handle, uint meshIdx);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_submesh_indices_u16(IntPtr handle, uint meshIdx, uint submesh, out uint len);
        [DllImport(Lib)] public static extern IntPtr gltforge_mesh_submesh_indices_u32(IntPtr handle, uint meshIdx, uint submesh, out uint len);

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
