/// A glTF mesh converted into Unity's left-handed coordinate system,
/// ready to be assigned to a `UnityEngine.Mesh`.
pub struct UnityMesh {
    /// Derived name for `UnityEngine.Mesh.name`.
    pub name: String,

    /// All vertex positions concatenated across every primitive (left-handed, X negated).
    /// Tightly packed `[x, y, z]` floats.
    pub positions: Vec<[f32; 3]>,

    /// One submesh per glTF primitive. All indices are absolute into `positions`.
    pub submeshes: Vec<UnitySubmesh>,
}

/// A single submesh, corresponding to one glTF primitive.
pub struct UnitySubmesh {
    /// Triangle indices with winding order reversed for Unity's left-handed convention.
    /// Format is shared across all submeshes and determined by total vertex count.
    pub indices: UnityIndices,
}

/// Index buffer format, selected once based on total vertex count across all primitives.
pub enum UnityIndices {
    /// Used when `total_vertex_count <= 65535`. Maps to Unity's `IndexFormat.UInt16`.
    U16(Vec<u16>),
    /// Used when `total_vertex_count > 65535`. Maps to Unity's `IndexFormat.UInt32`.
    U32(Vec<u32>),
}
