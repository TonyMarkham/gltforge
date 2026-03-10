using System;
using System.Collections.Generic;
using UnityEngine;

namespace Gltforge
{
    public class GltforgeAsset : ScriptableObject
    {
        [Serializable]
        public struct NodeEntry
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

        /// <summary>One entry per glTF node, in traversal order.</summary>
        public List<NodeEntry> nodes;

        /// <summary>One entry per unique glTF mesh index referenced by the scene.</summary>
        public List<MeshEntry> meshes;
    }
}
