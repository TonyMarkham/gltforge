use crate::unity_indices::UnityIndices;

/// A single glTF primitive converted to a Unity sub-mesh.
/// Indices are pre-offset to reference the shared vertex array on [`crate::unity_mesh::UnityMesh`].
pub struct UnitySubMesh {
    /// Triangle indices with winding order reversed for Unity's left-handed convention.
    /// Absolute offsets into the parent [`UnityMesh::vertices`] array.
    pub indices: UnityIndices,

    /// The glTF material index for this sub-mesh, if the primitive referenced one.
    pub material_index: Option<u32>,
}
