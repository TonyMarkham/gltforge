/// A single sub-mesh triangle list in Unity's left-handed coordinate system.
/// Indices are stored as `u32` regardless of the source format; the build step
/// selects the output component type based on the parent mesh's total vertex count.
pub struct SubmeshData {
    /// Triangle indices in Unity's left-handed winding order.
    /// Winding is reversed to right-handed during the build step.
    pub indices: Vec<u32>,
}
