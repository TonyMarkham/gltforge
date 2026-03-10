using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using UnityEditor.AssetImporters;
using UnityEngine;
using UnityEngine.Rendering;

namespace Gltforge.Editor
{
    [ScriptedImporter(3, "gltf")]
    public class GltforgeImporter : ScriptedImporter
    {
        public override void OnImportAsset(AssetImportContext ctx)
        {
            string absolutePath = Path.GetFullPath(ctx.assetPath);

            IntPtr handle = GltforgeNative.gltforge_load(absolutePath);
            if (handle == IntPtr.Zero)
            {
                ctx.LogImportError($"[gltforge] Failed to load '{ctx.assetPath}'.");
                return;
            }

            try
            {
                var asset = ScriptableObject.CreateInstance<GltforgeAsset>();
                asset.name   = GltforgeNative.ReadName(GltforgeNative.gltforge_scene_name(handle, out uint sceneNameLen), sceneNameLen);
                asset.nodes  = new List<GltforgeAsset.NodeEntry>();
                asset.meshes = new List<GltforgeAsset.MeshEntry>();

                // Build all meshes up-front, deduplicated by glTF mesh index.
                var builtMeshes = BuildAllMeshes(handle, asset, ctx);

                // Root GameObject named after the scene.
                var root = new GameObject(asset.name);

                // Traverse the scene graph recursively.
                uint rootCount = GltforgeNative.gltforge_root_node_count(handle);
                for (uint i = 0; i < rootCount; i++)
                {
                    uint nodeIdx = GltforgeNative.gltforge_root_node_index(handle, i);
                    TraverseNode(handle, nodeIdx, root.transform, asset, builtMeshes, ctx);
                }

                ctx.AddObjectToAsset("asset", asset);
                ctx.AddObjectToAsset("root",  root);
                ctx.SetMainObject(root);
            }
            finally
            {
                GltforgeNative.gltforge_release(handle);
            }
        }

        // ---- mesh building --------------------------------------------------

        static Dictionary<uint, Mesh> BuildAllMeshes(
            IntPtr handle,
            GltforgeAsset asset,
            AssetImportContext ctx)
        {
            var builtMeshes = new Dictionary<uint, Mesh>();
            uint meshCount = GltforgeNative.gltforge_mesh_count(handle);

            for (uint meshIdx = 0; meshIdx < meshCount; meshIdx++)
            {
                Mesh mesh = BuildMesh(handle, meshIdx);
                builtMeshes[meshIdx] = mesh;

                asset.meshes.Add(new GltforgeAsset.MeshEntry
                {
                    meshIndex = (int)meshIdx,
                    mesh      = mesh,
                });

                ctx.AddObjectToAsset($"mesh_{meshIdx}", mesh);
            }

            return builtMeshes;
        }

        static Mesh BuildMesh(IntPtr handle, uint meshIdx)
        {
            string meshName = GltforgeNative.ReadName(
                GltforgeNative.gltforge_mesh_name(handle, meshIdx, out uint nameLen), nameLen);

            // ---- vertices ---------------------------------------------------

            IntPtr posPtr  = GltforgeNative.gltforge_mesh_positions(handle, meshIdx, out uint posFloatCount);
            float[] floats = new float[posFloatCount];
            Marshal.Copy(posPtr, floats, 0, (int)posFloatCount);

            var vertices = new Vector3[posFloatCount / 3];
            for (int i = 0; i < vertices.Length; i++)
                vertices[i] = new Vector3(floats[i * 3], floats[i * 3 + 1], floats[i * 3 + 2]);

            // ---- index format (uniform across all sub-meshes) ---------------

            uint fmt = GltforgeNative.gltforge_mesh_index_format(handle, meshIdx);

            // ---- normals (optional) -----------------------------------------

            IntPtr normPtr = GltforgeNative.gltforge_mesh_normals(handle, meshIdx, out uint normFloatCount);
            Vector3[] normals = null;
            if (normPtr != IntPtr.Zero && normFloatCount > 0)
            {
                float[] normFloats = new float[normFloatCount];
                Marshal.Copy(normPtr, normFloats, 0, (int)normFloatCount);
                normals = new Vector3[normFloatCount / 3];
                for (int i = 0; i < normals.Length; i++)
                    normals[i] = new Vector3(normFloats[i * 3], normFloats[i * 3 + 1], normFloats[i * 3 + 2]);
            }

            var mesh = new Mesh
            {
                name        = meshName,
                indexFormat = fmt == 32 ? IndexFormat.UInt32 : IndexFormat.UInt16,
                vertices    = vertices,
            };

            if (normals != null)
                mesh.normals = normals;

            // ---- tangents (optional) -----------------------------------------

            IntPtr tangPtr = GltforgeNative.gltforge_mesh_tangents(handle, meshIdx, out uint tangFloatCount);
            if (tangPtr != IntPtr.Zero && tangFloatCount > 0)
            {
                float[] tangFloats = new float[tangFloatCount];
                Marshal.Copy(tangPtr, tangFloats, 0, (int)tangFloatCount);
                var tangents = new Vector4[tangFloatCount / 4];
                for (int i = 0; i < tangents.Length; i++)
                    tangents[i] = new Vector4(tangFloats[i * 4], tangFloats[i * 4 + 1], tangFloats[i * 4 + 2], tangFloats[i * 4 + 3]);
                mesh.tangents = tangents;
            }

            // ---- UV channels (optional) -------------------------------------

            uint uvChannelCount = GltforgeNative.gltforge_mesh_uv_channel_count(handle, meshIdx);
            for (uint ch = 0; ch < uvChannelCount; ch++)
            {
                IntPtr uvPtr = GltforgeNative.gltforge_mesh_uvs(handle, meshIdx, ch, out uint uvFloatCount);
                if (uvPtr == IntPtr.Zero || uvFloatCount == 0) continue;

                float[] uvFloats = new float[uvFloatCount];
                Marshal.Copy(uvPtr, uvFloats, 0, (int)uvFloatCount);
                var uvs = new Vector2[uvFloatCount / 2];
                for (int i = 0; i < uvs.Length; i++)
                    uvs[i] = new Vector2(uvFloats[i * 2], uvFloats[i * 2 + 1]);
                mesh.SetUVs((int)ch, uvs);
            }

            // ---- sub-meshes -------------------------------------------------

            uint submeshCount = GltforgeNative.gltforge_mesh_submesh_count(handle, meshIdx);
            mesh.subMeshCount = (int)submeshCount;

            for (uint s = 0; s < submeshCount; s++)
            {
                int[] triangles;

                if (fmt == 16)
                {
                    IntPtr idxPtr   = GltforgeNative.gltforge_mesh_submesh_indices_u16(handle, meshIdx, s, out uint idxCount);
                    short[] shorts  = new short[idxCount];
                    Marshal.Copy(idxPtr, shorts, 0, (int)idxCount);
                    triangles = Array.ConvertAll(shorts, v => (int)(ushort)v);
                }
                else
                {
                    IntPtr idxPtr = GltforgeNative.gltforge_mesh_submesh_indices_u32(handle, meshIdx, s, out uint idxCount);
                    int[] ints    = new int[idxCount];
                    Marshal.Copy(idxPtr, ints, 0, (int)idxCount);
                    triangles = ints;
                }

                mesh.SetTriangles(triangles, (int)s);
            }

            mesh.RecalculateBounds();
            return mesh;
        }

        // ---- scene graph traversal ------------------------------------------

        static void TraverseNode(
            IntPtr handle,
            uint nodeIdx,
            Transform parent,
            GltforgeAsset asset,
            Dictionary<uint, Mesh> builtMeshes,
            AssetImportContext ctx)
        {
            string nodeName = GltforgeNative.ReadName(
                GltforgeNative.gltforge_node_name(handle, nodeIdx, out uint nameLen), nameLen);

            var go = new GameObject(nodeName);
            go.transform.SetParent(parent, worldPositionStays: false);

            float[] t = new float[10];
            try
            {
                var pin = GCHandle.Alloc(t, GCHandleType.Pinned);
                try   { GltforgeNative.gltforge_node_transform(handle, nodeIdx, pin.AddrOfPinnedObject()); }
                finally { pin.Free(); }
            }
            catch (Exception e)
            {
                ctx.LogImportWarning($"[gltforge] transform error on node {nodeIdx}: {e.Message}");
            }
            go.transform.localPosition = new Vector3(t[0], t[1], t[2]);
            go.transform.localRotation = new Quaternion(t[3], t[4], t[5], t[6]);
            go.transform.localScale    = new Vector3(t[7], t[8], t[9]);

            asset.nodes.Add(new GltforgeAsset.NodeEntry
            {
                nodeIndex  = (int)nodeIdx,
                gameObject = go,
            });

            // Attach meshes referenced by this node.
            uint meshRefCount = GltforgeNative.gltforge_node_mesh_count(handle, nodeIdx);
            for (uint slot = 0; slot < meshRefCount; slot++)
            {
                uint meshIdx = GltforgeNative.gltforge_node_mesh_index(handle, nodeIdx, slot);
                if (builtMeshes.TryGetValue(meshIdx, out Mesh mesh))
                {
                    go.AddComponent<MeshFilter>().sharedMesh = mesh;
                    go.AddComponent<MeshRenderer>();
                }
            }

            // Recurse into children.
            uint childCount = GltforgeNative.gltforge_node_child_count(handle, nodeIdx);
            for (uint i = 0; i < childCount; i++)
            {
                uint childIdx = GltforgeNative.gltforge_node_child(handle, nodeIdx, i);
                TraverseNode(handle, childIdx, go.transform, asset, builtMeshes, ctx);
            }
        }
    }
}
