/// Resolved data for a single glTF primitive, in Unity's left-handed coordinate system.
/// Used internally during mesh conversion before primitives are merged into a [`crate::unity_mesh::UnityMesh`].
pub(crate) struct GltfPrimitiveData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub wound: Vec<u32>,
}
