using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using UnityEditor;
using UnityEditor.AssetImporters;
using UnityEngine;
using UnityEngine.Rendering;

namespace Gltforge.Editor
{
    public enum NamingMode
    {
        /// <summary>
        /// Uses the glTF name exactly as provided. Objects with no glTF name get an empty name,
        /// which Unity renders as the default ("GameObject", "Mesh", etc.).
        /// </summary>
        Strict,

        /// <summary>
        /// Applies fallback names when a glTF object has no name: scene container uses the scene
        /// name or file stem; meshes and materials fall back to their glTF index.
        /// </summary>
        Auto,
    }

    public enum SceneRootMode
    {
        /// <summary>
        /// One glTF root node → use it directly as the prefab root, falling back to the scene
        /// name then the file stem when the node has no name.
        /// Multiple glTF root nodes → wrap them in a scene-named container GameObject.
        /// </summary>
        Auto,

        /// <summary>
        /// Always create a root container GameObject named after the scene or file,
        /// regardless of how many glTF root nodes exist.
        /// </summary>
        AlwaysWrap,
    }

    [ScriptedImporter(4, new[] { "gltf", "glb" })]
    public class GltforgeImporter : ScriptedImporter
    {
        [SerializeField] NamingMode    _namingMode    = NamingMode.Strict; // ordinal 0
        [SerializeField] SceneRootMode _sceneRootMode = SceneRootMode.Auto;

        static Shader _pbrShader;
        static Shader _pbrBlendShader;
        static Shader PbrShader      => _pbrShader      ??= Shader.Find("Shader Graphs/glTF PBR Metallic Roughness");
        static Shader PbrBlendShader => _pbrBlendShader ??= Shader.Find("Shader Graphs/glTF PBR Metallic Roughness Blend");

        public override void OnImportAsset(AssetImportContext ctx)
        {
            string absolutePath = Path.GetFullPath(ctx.assetPath);
            string gltfDir      = Path.GetDirectoryName(absolutePath);

            IntPtr handle = GltforgeNative.gltforge_load(absolutePath);
            if (handle == IntPtr.Zero)
            {
                ctx.LogImportError($"[gltforge] Failed to load '{ctx.assetPath}'.");
                return;
            }

            try
            {
                string fileStem   = Path.GetFileNameWithoutExtension(ctx.assetPath);
                string sceneName  = GltforgeNative.ReadName(GltforgeNative.gltforge_scene_name(handle, out uint sceneNameLen), sceneNameLen);
                string rootName   = _namingMode == NamingMode.Auto ? (sceneName ?? fileStem) : sceneName;

                var asset = ScriptableObject.CreateInstance<GltforgeAsset>();
                asset.name        = rootName;
                asset.gameObjects = new List<GltforgeAsset.GameObjectEntry>();
                asset.meshes      = new List<GltforgeAsset.MeshEntry>();
                asset.textures    = new List<GltforgeAsset.TextureEntry>();
                asset.materials   = new List<GltforgeAsset.MaterialEntry>();

                // Build textures, then materials (materials reference textures by index).
                var normalMapIndices = CollectNormalMapIndices(handle);
                var builtTextures    = BuildAllTextures(handle, gltfDir, normalMapIndices, asset, ctx);
                var builtMaterials   = BuildAllMaterials(handle, builtTextures, asset, ctx, _namingMode);

                // Build all meshes up-front, deduplicated by glTF mesh index.
                var builtMeshes = BuildAllMeshes(handle, asset, ctx, _namingMode);

                // Build the root GameObject according to the configured SceneRootMode.
                uint rootCount = GltforgeNative.gltforge_root_game_object_count(handle);

                GameObject root;
                if (_sceneRootMode == SceneRootMode.Auto && rootCount == 1)
                {
                    // Single glTF root node — use it directly as the prefab root.
                    uint goIdx = GltforgeNative.gltforge_root_game_object_index(handle, 0);
                    root = TraverseGameObject(handle, goIdx, null, asset, builtMeshes, builtMaterials, ctx);
                    if (_namingMode == NamingMode.Auto && string.IsNullOrEmpty(root.name))
                        root.name = rootName;
                }
                else
                {
                    // Multiple root nodes (or AlwaysWrap) — wrap everything in a named container.
                    root = new GameObject(rootName);
                    for (uint i = 0; i < rootCount; i++)
                    {
                        uint goIdx = GltforgeNative.gltforge_root_game_object_index(handle, i);
                        TraverseGameObject(handle, goIdx, root.transform, asset, builtMeshes, builtMaterials, ctx);
                    }
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

        // ---- textures -------------------------------------------------------

        /// <summary>
        /// Returns the set of glTF image indices used as normal maps across all PBR materials.
        /// These textures must be imported as <c>TextureImporterType.NormalMap</c>.
        /// </summary>
        static HashSet<uint> CollectNormalMapIndices(IntPtr handle)
        {
            var result = new HashSet<uint>();
            uint matCount = GltforgeNative.gltforge_pbr_metallic_roughness_count(handle);
            for (uint matIdx = 0; matIdx < matCount; matIdx++)
            {
                int idx = GltforgeNative.gltforge_pbr_metallic_roughness_normal_texture(handle, matIdx);
                if (idx >= 0) result.Add((uint)idx);
            }
            return result;
        }

        static Dictionary<uint, Texture2D> BuildAllTextures(
            IntPtr handle,
            string gltfDir,
            HashSet<uint> normalMapIndices,
            GltforgeAsset asset,
            AssetImportContext ctx)
        {
            var built = new Dictionary<uint, Texture2D>();
            uint imageCount = GltforgeNative.gltforge_image_count(handle);

            for (uint imageIdx = 0; imageIdx < imageCount; imageIdx++)
            {
                bool isNormalMap = normalMapIndices.Contains(imageIdx);

                IntPtr uriPtr = GltforgeNative.gltforge_image_uri(handle, imageIdx, out uint uriLen);
                if (uriPtr == IntPtr.Zero)
                {
                    // Embedded image — decode raw bytes directly.
                    IntPtr bytesPtr = GltforgeNative.gltforge_image_bytes(handle, imageIdx, out uint byteLen);
                    if (bytesPtr == IntPtr.Zero)
                    {
                        ctx.LogImportWarning($"[gltforge] Image {imageIdx} has no URI and no embedded bytes; skipping.");
                        continue;
                    }

                    byte[] rawBytes = new byte[byteLen];
                    Marshal.Copy(bytesPtr, rawBytes, 0, (int)byteLen);

                    // Use linear color space for normal maps; sRGB for everything else.
                    var tex = new Texture2D(2, 2, TextureFormat.RGBA32, mipChain: false, linear: isNormalMap);
                    if (!ImageConversion.LoadImage(tex, rawBytes))
                    {
                        ctx.LogImportWarning($"[gltforge] Failed to decode embedded image {imageIdx}; skipping.");
                        UnityEngine.Object.DestroyImmediate(tex);
                        continue;
                    }

                    string embName = GltforgeNative.ReadName(
                        GltforgeNative.gltforge_image_name(handle, imageIdx, out uint embNameLen), embNameLen);
                    tex.name = embName ?? imageIdx.ToString();

                    built[imageIdx] = tex;
                    ctx.AddObjectToAsset($"tex_{imageIdx}", tex);

                    asset.textures.Add(new GltforgeAsset.TextureEntry
                    {
                        imageIndex = (int)imageIdx,
                        texture    = tex,
                    });
                    continue;
                }

                string uri          = GltforgeNative.ReadName(uriPtr, uriLen);
                string absolutePath = Path.Combine(gltfDir, uri);
                string assetPath    = AbsoluteToAssetPath(absolutePath);

                // Ensure normal maps are imported with the correct texture type so Unity
                // applies the right channel swizzle and compression.
                EnsureTextureType(assetPath, isNormalMap
                    ? TextureImporterType.NormalMap
                    : TextureImporterType.Default);

                Texture2D uriTex = AssetDatabase.LoadAssetAtPath<Texture2D>(assetPath);
                if (uriTex == null)
                {
                    ctx.LogImportWarning($"[gltforge] Could not load texture '{assetPath}' (image {imageIdx}).");
                    continue;
                }

                ctx.DependsOnArtifact(AssetDatabase.GUIDFromAssetPath(assetPath));
                built[imageIdx] = uriTex;

                asset.textures.Add(new GltforgeAsset.TextureEntry
                {
                    imageIndex = (int)imageIdx,
                    texture    = uriTex,
                });
            }

            return built;
        }

        /// <summary>
        /// Sets the <see cref="TextureImporter"/> type for <paramref name="assetPath"/> if it
        /// doesn't already match <paramref name="type"/>.<br/>
        /// When called during an AssetDatabase refresh (<see cref="EditorApplication.isUpdating"/>),
        /// the settings are written and the reimport is deferred via
        /// <see cref="EditorApplication.delayCall"/> so we don't load a stale in-flight asset.
        /// <see cref="AssetImportContext.DependsOnArtifact"/> will then cascade the .gltf reimport
        /// automatically once the texture finishes.
        /// </summary>
        static void EnsureTextureType(string assetPath, TextureImporterType type)
        {
            var ti = AssetImporter.GetAtPath(assetPath) as TextureImporter;
            if (ti == null || ti.textureType == type) return;
            ti.textureType = type;
            if (type == TextureImporterType.NormalMap)
            {
                ti.convertToNormalmap = false; // glTF normals are already encoded, not height maps.
                ti.sRGBTexture        = false;
            }
            if (EditorApplication.isUpdating)
            {
                AssetDatabase.WriteImportSettingsIfDirty(assetPath);
                EditorApplication.delayCall += () => AssetDatabase.ImportAsset(assetPath);
            }
            else
            {
                ti.SaveAndReimport();
            }
        }

        // ---- materials ------------------------------------------------------

        static Dictionary<uint, Material> BuildAllMaterials(
            IntPtr handle,
            Dictionary<uint, Texture2D> builtTextures,
            GltforgeAsset asset,
            AssetImportContext ctx,
            NamingMode namingMode)
        {
            var built = new Dictionary<uint, Material>();
            uint matCount = GltforgeNative.gltforge_pbr_metallic_roughness_count(handle);

            for (uint matIdx = 0; matIdx < matCount; matIdx++)
            {
                Material mat = BuildPbrMaterial(handle, matIdx, builtTextures, namingMode);
                built[matIdx] = mat;

                asset.materials.Add(new GltforgeAsset.MaterialEntry
                {
                    materialIndex = (int)matIdx,
                    material      = mat,
                });

                ctx.AddObjectToAsset($"mat_{matIdx}", mat);
            }

            return built;
        }

        static Material BuildPbrMaterial(
            IntPtr handle,
            uint matIdx,
            Dictionary<uint, Texture2D> builtTextures,
            NamingMode namingMode)
        {
            string rawName = GltforgeNative.ReadName(
                GltforgeNative.gltforge_pbr_metallic_roughness_name(handle, matIdx, out uint nameLen), nameLen);
            string name = namingMode == NamingMode.Auto ? (rawName ?? matIdx.ToString()) : (rawName ?? string.Empty);

            uint alphaMode = GltforgeNative.gltforge_pbr_metallic_roughness_alpha_mode(handle, matIdx);
            var shader = alphaMode == 2 ? PbrBlendShader : PbrShader;
            var mat = new Material(shader) { name = name };

            // ── Textures ─────────────────────────────────────────────────────

            Texture2D TryGetTex(int imageIdx) =>
                imageIdx >= 0 && builtTextures.TryGetValue((uint)imageIdx, out var t) ? t : null;

            var baseColorTex = TryGetTex(GltforgeNative.gltforge_pbr_metallic_roughness_base_color_texture(handle, matIdx));
            if (baseColorTex != null)
                mat.SetTexture("_BaseMap", baseColorTex);

            var metallicRoughTex = TryGetTex(GltforgeNative.gltforge_pbr_metallic_roughness_metallic_roughness_texture(handle, matIdx));
            if (metallicRoughTex != null)
                mat.SetTexture("_MetallicRoughnessMap", metallicRoughTex);

            var normalTex = TryGetTex(GltforgeNative.gltforge_pbr_metallic_roughness_normal_texture(handle, matIdx));
            if (normalTex != null)
                mat.SetTexture("_BumpMap", normalTex);

            var occlusionTex = TryGetTex(GltforgeNative.gltforge_pbr_metallic_roughness_occlusion_texture(handle, matIdx));
            if (occlusionTex != null)
                mat.SetTexture("_OcclusionMap", occlusionTex);

            var emissiveTex = TryGetTex(GltforgeNative.gltforge_pbr_metallic_roughness_emissive_texture(handle, matIdx));
            if (emissiveTex != null)
                mat.SetTexture("_EmissionMap", emissiveTex);

            // ── Scalar factors ────────────────────────────────────────────────

            var baseColorFactor = new float[4];
            var pin = GCHandle.Alloc(baseColorFactor, GCHandleType.Pinned);
            try   { GltforgeNative.gltforge_pbr_metallic_roughness_base_color_factor(handle, matIdx, pin.AddrOfPinnedObject()); }
            finally { pin.Free(); }
            mat.SetColor("_BaseColor", new Color(baseColorFactor[0], baseColorFactor[1], baseColorFactor[2], baseColorFactor[3]));

            mat.SetFloat("_Metallic",           GltforgeNative.gltforge_pbr_metallic_roughness_metallic_factor(handle, matIdx));
            mat.SetFloat("_Roughness",          GltforgeNative.gltforge_pbr_metallic_roughness_roughness_factor(handle, matIdx));
            mat.SetFloat("_BumpScale",          GltforgeNative.gltforge_pbr_metallic_roughness_normal_scale(handle, matIdx));
            mat.SetFloat("_OcclusionStrength",  GltforgeNative.gltforge_pbr_metallic_roughness_occlusion_strength(handle, matIdx));

            var emissiveFactor = new float[3];
            pin = GCHandle.Alloc(emissiveFactor, GCHandleType.Pinned);
            try   { GltforgeNative.gltforge_pbr_metallic_roughness_emissive_factor(handle, matIdx, pin.AddrOfPinnedObject()); }
            finally { pin.Free(); }
            var emissiveColor = new Color(emissiveFactor[0], emissiveFactor[1], emissiveFactor[2]);
            mat.SetColor("_EmissionColor", emissiveColor);

            mat.SetFloat("_Cutoff", GltforgeNative.gltforge_pbr_metallic_roughness_alpha_cutoff(handle, matIdx));

            return mat;
        }

        // ---- mesh building --------------------------------------------------

        static Dictionary<uint, Mesh> BuildAllMeshes(
            IntPtr handle,
            GltforgeAsset asset,
            AssetImportContext ctx,
            NamingMode namingMode)
        {
            var builtMeshes = new Dictionary<uint, Mesh>();
            uint meshCount = GltforgeNative.gltforge_mesh_count(handle);

            for (uint meshIdx = 0; meshIdx < meshCount; meshIdx++)
            {
                Mesh mesh = BuildMesh(handle, meshIdx, namingMode);
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

        static Mesh BuildMesh(IntPtr handle, uint meshIdx, NamingMode namingMode)
        {
            string rawName  = GltforgeNative.ReadName(GltforgeNative.gltforge_mesh_name(handle, meshIdx, out uint nameLen), nameLen);
            string meshName = namingMode == NamingMode.Auto ? (rawName ?? meshIdx.ToString()) : (rawName ?? string.Empty);

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
                    IntPtr idxPtr  = GltforgeNative.gltforge_mesh_submesh_indices_u16(handle, meshIdx, s, out uint idxCount);
                    short[] shorts = new short[idxCount];
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

        static GameObject TraverseGameObject(
            IntPtr handle,
            uint goIdx,
            Transform parent,
            GltforgeAsset asset,
            Dictionary<uint, Mesh> builtMeshes,
            Dictionary<uint, Material> builtMaterials,
            AssetImportContext ctx)
        {
            string goName = GltforgeNative.ReadName(
                GltforgeNative.gltforge_game_object_name(handle, goIdx, out uint nameLen), nameLen);

            var go = new GameObject(goName);
            if (parent != null)
                go.transform.SetParent(parent, worldPositionStays: false);

            float[] t = new float[10];
            try
            {
                var pin = GCHandle.Alloc(t, GCHandleType.Pinned);
                try   { GltforgeNative.gltforge_game_object_transform(handle, goIdx, pin.AddrOfPinnedObject()); }
                finally { pin.Free(); }
            }
            catch (Exception e)
            {
                ctx.LogImportWarning($"[gltforge] transform error on game object {goIdx}: {e.Message}");
            }
            go.transform.localPosition = new Vector3(t[0], t[1], t[2]);
            go.transform.localRotation = new Quaternion(t[3], t[4], t[5], t[6]);
            go.transform.localScale    = new Vector3(t[7], t[8], t[9]);

            asset.gameObjects.Add(new GltforgeAsset.GameObjectEntry
            {
                nodeIndex  = (int)goIdx,
                gameObject = go,
            });

            // Attach meshes referenced by this GameObject.
            uint meshRefCount = GltforgeNative.gltforge_game_object_mesh_count(handle, goIdx);
            for (uint slot = 0; slot < meshRefCount; slot++)
            {
                uint meshIdx = GltforgeNative.gltforge_game_object_mesh_index(handle, goIdx, slot);
                if (!builtMeshes.TryGetValue(meshIdx, out Mesh mesh))
                    continue;

                go.AddComponent<MeshFilter>().sharedMesh = mesh;

                uint submeshCount = GltforgeNative.gltforge_mesh_submesh_count(handle, meshIdx);
                var mats = new Material[submeshCount];
                for (uint s = 0; s < submeshCount; s++)
                {
                    int matIdx = GltforgeNative.gltforge_mesh_submesh_material(handle, meshIdx, s);
                    if (matIdx >= 0 && builtMaterials.TryGetValue((uint)matIdx, out Material mat))
                        mats[s] = mat;
                }

                go.AddComponent<MeshRenderer>().sharedMaterials = mats;
            }

            // Recurse into children.
            uint childCount = GltforgeNative.gltforge_game_object_child_count(handle, goIdx);
            for (uint i = 0; i < childCount; i++)
            {
                uint childIdx = GltforgeNative.gltforge_game_object_child(handle, goIdx, i);
                TraverseGameObject(handle, childIdx, go.transform, asset, builtMeshes, builtMaterials, ctx);
            }

            return go;
        }

        // ---- utilities ------------------------------------------------------

        /// <summary>
        /// Converts an absolute file-system path to a Unity asset path (<c>Assets/…</c>).
        /// </summary>
        static string AbsoluteToAssetPath(string absolutePath)
        {
            string full    = Path.GetFullPath(absolutePath).Replace('\\', '/');
            string dataDir = Path.GetFullPath(Application.dataPath).Replace('\\', '/');

            if (full.StartsWith(dataDir, StringComparison.OrdinalIgnoreCase))
                return "Assets" + full.Substring(dataDir.Length);

            return full; // Outside the Assets folder — AssetDatabase.LoadAssetAtPath will return null.
        }
    }
}
