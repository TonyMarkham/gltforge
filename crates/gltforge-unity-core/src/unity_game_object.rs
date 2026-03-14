use crate::unity_transform::UnityTransform;

/// A Unity-shaped glTF node. References children and meshes by index,
/// mirroring the glTF node structure.
pub struct UnityGameObject {
    /// The GameObject name. Falls back to the glTF node index if the source node is unnamed.
    pub name: String,

    /// Indices of child GameObjects.
    pub children: Vec<u32>,

    /// Indices of meshes referenced by this GameObject.
    /// In glTF a node references at most one mesh, but that mesh may have
    /// multiple primitives stored as separate entries.
    pub mesh_indices: Vec<u32>,

    /// Local transform in Unity's left-handed coordinate system.
    pub transform: UnityTransform,
}
