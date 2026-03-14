/// A Unity GameObject to be exported as a glTF node.
/// Transform values are in Unity's left-handed coordinate system; the build step
/// inverts handedness to produce right-handed glTF output.
pub struct NodeData {
    /// Node name, or `None` to omit from the glTF output.
    pub name: Option<String>,

    /// Index of the parent node, or `None` for root nodes.
    pub parent: Option<u32>,

    /// Index into the export context's mesh list, if this node has a mesh.
    pub mesh_index: Option<u32>,

    /// Local position `[x, y, z]` in Unity space, or `None` to omit (glTF default: origin).
    pub translation: Option<[f32; 3]>,

    /// Local rotation as a unit quaternion `[x, y, z, w]` in Unity space,
    /// or `None` to omit (glTF default: identity).
    pub rotation: Option<[f32; 4]>,

    /// Local scale `[x, y, z]`, or `None` to omit (glTF default: ones).
    pub scale: Option<[f32; 3]>,
}
