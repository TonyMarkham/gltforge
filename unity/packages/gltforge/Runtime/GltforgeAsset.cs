using System;
using System.Collections.Generic;
using UnityEngine;

namespace Gltforge
{
    public class GltforgeAsset : ScriptableObject
    {
        [Serializable]
        public struct GameObjectEntry
        {
            /// <summary>glTF node index.</summary>
            public int nodeIndex;
            /// <summary>The GameObject created for this node.</summary>
            public GameObject gameObject;
        }

        [Serializable]
        public struct MeshEntry
        {
            /// <summary>glTF mesh index.</summary>
            public int meshIndex;
            /// <summary>
            /// The Unity Mesh built from this glTF mesh.
            /// Sub-mesh count matches the number of glTF primitives.
            /// </summary>
            public Mesh mesh;
        }

        [Serializable]
        public struct TextureEntry
        {
            /// <summary>glTF image index.</summary>
            public int imageIndex;
            /// <summary>The Texture2D loaded from the glTF image URI.</summary>
            public Texture2D texture;
        }

        [Serializable]
        public struct MaterialEntry
        {
            /// <summary>glTF material index.</summary>
            public int materialIndex;
            /// <summary>
            /// The Unity Material built for this glTF PBR metallic-roughness material,
            /// using the <c>GLTF/PbrMetallicRoughness</c> shader.
            /// </summary>
            public Material material;
        }

        /// <summary>One entry per glTF node, in traversal order.</summary>
        public List<GameObjectEntry> gameObjects;

        /// <summary>One entry per unique glTF mesh index referenced by the scene.</summary>
        public List<MeshEntry> meshes;

        /// <summary>One entry per glTF image, keyed by glTF image index.</summary>
        public List<TextureEntry> textures;

        /// <summary>One entry per glTF material, keyed by glTF material index.</summary>
        public List<MaterialEntry> materials;
    }
}
