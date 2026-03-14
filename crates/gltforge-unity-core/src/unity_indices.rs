/// Index buffer format, selected per primitive based on vertex count.
pub enum UnityIndices {
    /// Used when `vertex_count <= 65535`. Maps to Unity's `IndexFormat.UInt16`.
    U16(Vec<u16>),
    /// Used when `vertex_count > 65535`. Maps to Unity's `IndexFormat.UInt32`.
    U32(Vec<u32>),
}
