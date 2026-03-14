using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using UnityEditor;
using UnityEngine;
using UnityEngine.Rendering;

namespace Gltforge.Editor
{
    public static class GltforgeExport
    {
        private const string ExportGltfMenuName = "Assets/Gltforge/Export (glTF)";
        private const string ExportGlbMenuName  = "Assets/Gltforge/Export (GLB)";

        [MenuItem(ExportGltfMenuName, false, 100)]
        private static void ExportGltf()
        {
            var go = (GameObject)Selection.activeObject;
            string path = EditorUtility.SaveFilePanel("Export glTF", "", go.name, "gltf");
            if (string.IsNullOrEmpty(path)) return;
            RunExport(go, path, glb: false);
        }

        [MenuItem(ExportGltfMenuName, true, 100)]
        private static bool ExportGltfValidate() => Selection.activeObject is GameObject;

        [MenuItem(ExportGlbMenuName, false, 101)]
        private static void ExportGlb()
        {
            var go = (GameObject)Selection.activeObject;
            string path = EditorUtility.SaveFilePanel("Export GLB", "", go.name, "glb");
            if (string.IsNullOrEmpty(path)) return;
            RunExport(go, path, glb: true);
        }

        [MenuItem(ExportGlbMenuName, true, 101)]
        private static bool ExportGlbValidate() => Selection.activeObject is GameObject;

        static void RunExport(GameObject go, string path, bool glb)
        {
            IntPtr ctx = GltforgeNativeExport.gltforge_export_begin();
            if (ctx == IntPtr.Zero)
            {
                Debug.LogError("[gltforge] Failed to create export context.");
                return;
            }

            try
            {
                var meshMap = new Dictionary<int, uint>(); // instanceID → export mesh index
                PushGameObject(ctx, go, -1, meshMap);
            }
            catch
            {
                GltforgeNativeExport.gltforge_export_free(ctx);
                throw;
            }

            // finish always consumes ctx, even on failure — do not free after this point.
            byte ok = glb
                ? GltforgeNativeExport.gltforge_export_finish_glb(ctx, path)
                : GltforgeNativeExport.gltforge_export_finish(ctx, path);

            if (ok != 0)
                Debug.Log($"[gltforge] Exported '{go.name}' to {path}");
            else
                Debug.LogError($"[gltforge] {(glb ? "GLB" : "glTF")} export failed for '{go.name}'.");
        }

        // ---- traversal ------------------------------------------------------

        static void PushGameObject(IntPtr ctx, GameObject go, int parentIdx, Dictionary<int, uint> meshMap)
        {
            uint nodeIdx = PushNode(ctx, go, parentIdx);

            var meshFilter = go.GetComponent<MeshFilter>();
            if (meshFilter != null && meshFilter.sharedMesh != null)
            {
                uint meshIdx = GetOrPushMesh(ctx, meshFilter.sharedMesh, meshMap);
                GltforgeNativeExport.gltforge_export_node_set_mesh(ctx, nodeIdx, meshIdx);
            }

            for (int i = 0; i < go.transform.childCount; i++)
                PushGameObject(ctx, go.transform.GetChild(i).gameObject, (int)nodeIdx, meshMap);
        }

        static uint PushNode(IntPtr ctx, GameObject go, int parentIdx)
        {
            var t = go.transform;
            var pos   = new float[] { t.localPosition.x, t.localPosition.y, t.localPosition.z };
            var rot   = new float[] { t.localRotation.x, t.localRotation.y, t.localRotation.z, t.localRotation.w };
            var scale = new float[] { t.localScale.x,    t.localScale.y,    t.localScale.z };

            var posPin   = GCHandle.Alloc(pos,   GCHandleType.Pinned);
            var rotPin   = GCHandle.Alloc(rot,   GCHandleType.Pinned);
            var scalePin = GCHandle.Alloc(scale, GCHandleType.Pinned);
            try
            {
                return WithName(go.name, (namePtr, nameLen) =>
                    GltforgeNativeExport.gltforge_export_add_node(
                        ctx, namePtr, nameLen, parentIdx,
                        posPin.AddrOfPinnedObject(),
                        rotPin.AddrOfPinnedObject(),
                        scalePin.AddrOfPinnedObject()));
            }
            finally
            {
                posPin.Free();
                rotPin.Free();
                scalePin.Free();
            }
        }

        static uint GetOrPushMesh(IntPtr ctx, Mesh mesh, Dictionary<int, uint> meshMap)
        {
            if (meshMap.TryGetValue(mesh.GetInstanceID(), out uint existing))
                return existing;

            uint meshIdx = PushMesh(ctx, mesh);
            meshMap[mesh.GetInstanceID()] = meshIdx;
            return meshIdx;
        }

        static uint PushMesh(IntPtr ctx, Mesh mesh)
        {
            uint meshIdx = WithName(mesh.name, (namePtr, nameLen) =>
                GltforgeNativeExport.gltforge_export_add_mesh(ctx, namePtr, nameLen));

            // Positions.
            PushVec3Channel(mesh.vertices, floats =>
                GltforgeNativeExport.gltforge_export_mesh_set_positions(ctx, meshIdx, floats, (uint)(mesh.vertexCount * 3)));

            // Normals (optional).
            if (mesh.normals is { Length: > 0 })
                PushVec3Channel(mesh.normals, floats =>
                    GltforgeNativeExport.gltforge_export_mesh_set_normals(ctx, meshIdx, floats, (uint)(mesh.normals.Length * 3)));

            // UV channels — stop at the first absent channel.
            var uvList = new List<Vector2>();
            for (int ch = 0; ch < 8; ch++)
            {
                uvList.Clear();
                mesh.GetUVs(ch, uvList);
                if (uvList.Count == 0) break;

                int capturedCh = ch;
                PushVec2Channel(uvList, floats =>
                    GltforgeNativeExport.gltforge_export_mesh_set_uvs(ctx, meshIdx, (uint)capturedCh, floats, (uint)(uvList.Count * 2)));
            }

            // Sub-meshes.
            bool use16 = mesh.indexFormat == IndexFormat.UInt16;
            for (int s = 0; s < mesh.subMeshCount; s++)
            {
                int[] tris = mesh.GetTriangles(s);
                if (use16)
                {
                    var indices = new ushort[tris.Length];
                    for (int i = 0; i < tris.Length; i++) indices[i] = (ushort)tris[i];
                    Pinned(indices, ptr =>
                        GltforgeNativeExport.gltforge_export_mesh_add_submesh_u16(ctx, meshIdx, ptr, (uint)indices.Length));
                }
                else
                {
                    var indices = new uint[tris.Length];
                    for (int i = 0; i < tris.Length; i++) indices[i] = (uint)tris[i];
                    Pinned(indices, ptr =>
                        GltforgeNativeExport.gltforge_export_mesh_add_submesh_u32(ctx, meshIdx, ptr, (uint)indices.Length));
                }
            }

            return meshIdx;
        }

        // ---- marshaling helpers ---------------------------------------------

        static T WithName<T>(string name, Func<IntPtr, uint, T> action)
        {
            if (string.IsNullOrEmpty(name))
                return action(IntPtr.Zero, 0);

            byte[] bytes = Encoding.UTF8.GetBytes(name);
            var pin = GCHandle.Alloc(bytes, GCHandleType.Pinned);
            try   { return action(pin.AddrOfPinnedObject(), (uint)bytes.Length); }
            finally { pin.Free(); }
        }

        static void PushVec3Channel(Vector3[] verts, Action<IntPtr> action)
        {
            var floats = new float[verts.Length * 3];
            for (int i = 0; i < verts.Length; i++)
            {
                floats[i * 3]     = verts[i].x;
                floats[i * 3 + 1] = verts[i].y;
                floats[i * 3 + 2] = verts[i].z;
            }
            Pinned(floats, action);
        }

        static void PushVec2Channel(List<Vector2> uvs, Action<IntPtr> action)
        {
            var floats = new float[uvs.Count * 2];
            for (int i = 0; i < uvs.Count; i++)
            {
                floats[i * 2]     = uvs[i].x;
                floats[i * 2 + 1] = uvs[i].y;
            }
            Pinned(floats, action);
        }

        static void Pinned<T>(T[] array, Action<IntPtr> action) where T : struct
        {
            var pin = GCHandle.Alloc(array, GCHandleType.Pinned);
            try   { action(pin.AddrOfPinnedObject()); }
            finally { pin.Free(); }
        }
    }
}
