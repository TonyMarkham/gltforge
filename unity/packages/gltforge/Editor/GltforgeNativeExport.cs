using System;
using System.Runtime.InteropServices;

namespace Gltforge.Editor
{
    internal static class GltforgeNativeExport
    {
        const string Lib = "gltforge_unity_export";

        // ---- lifecycle ------------------------------------------------------

        [DllImport(Lib)] public static extern IntPtr gltforge_export_begin();
        [DllImport(Lib)] public static extern void   gltforge_export_free(IntPtr ctx);

        // ---- nodes ----------------------------------------------------------

        /// <summary>
        /// Add a node. <paramref name="namePtr"/> points to <paramref name="nameLen"/> UTF-8 bytes
        /// (not null-terminated); pass <c>IntPtr.Zero</c> / 0 to omit the name.
        /// <paramref name="parentIdx"/> is -1 for root nodes.
        /// <paramref name="pos"/>, <paramref name="rot"/>, <paramref name="scale"/> point to 3, 4,
        /// and 3 contiguous <c>float</c> values respectively; pass <c>IntPtr.Zero</c> to omit.
        /// </summary>
        [DllImport(Lib)]
        public static extern uint gltforge_export_add_node(
            IntPtr ctx,
            IntPtr namePtr, uint nameLen,
            int    parentIdx,
            IntPtr pos, IntPtr rot, IntPtr scale);

        [DllImport(Lib)] public static extern void gltforge_export_node_set_mesh(IntPtr ctx, uint nodeIdx, uint meshIdx);

        // ---- meshes ---------------------------------------------------------

        [DllImport(Lib)] public static extern uint gltforge_export_add_mesh(IntPtr ctx, IntPtr namePtr, uint nameLen);

        /// <summary>Tightly packed [x, y, z] floats. <paramref name="f32Count"/> must be a multiple of 3.</summary>
        [DllImport(Lib)] public static extern void gltforge_export_mesh_set_positions(IntPtr ctx, uint meshIdx, IntPtr ptr, uint f32Count);

        /// <summary>Tightly packed [x, y, z] floats. <paramref name="f32Count"/> must be a multiple of 3.</summary>
        [DllImport(Lib)] public static extern void gltforge_export_mesh_set_normals(IntPtr ctx, uint meshIdx, IntPtr ptr, uint f32Count);

        /// <summary>Tightly packed [u, v] floats. <paramref name="f32Count"/> must be a multiple of 2.</summary>
        [DllImport(Lib)] public static extern void gltforge_export_mesh_set_uvs(IntPtr ctx, uint meshIdx, uint channel, IntPtr ptr, uint f32Count);

        /// <summary>Triangle indices. <paramref name="indexCount"/> must be a multiple of 3.</summary>
        [DllImport(Lib)] public static extern void gltforge_export_mesh_add_submesh_u16(IntPtr ctx, uint meshIdx, IntPtr ptr, uint indexCount);

        /// <summary>Triangle indices. <paramref name="indexCount"/> must be a multiple of 3.</summary>
        [DllImport(Lib)] public static extern void gltforge_export_mesh_add_submesh_u32(IntPtr ctx, uint meshIdx, IntPtr ptr, uint indexCount);

        // ---- finish ---------------------------------------------------------

        /// <summary>
        /// Write the .gltf and .bin files. Consumes and frees the context in all cases.
        /// Returns 1 on success, 0 on failure.
        /// </summary>
        [DllImport(Lib)] public static extern byte gltforge_export_finish(IntPtr ctx, string path);

        /// <summary>
        /// Write a single .glb file (binary glTF). Consumes and frees the context in all cases.
        /// Returns 1 on success, 0 on failure.
        /// </summary>
        [DllImport(Lib)] public static extern byte gltforge_export_finish_glb(IntPtr ctx, string path);
    }
}
