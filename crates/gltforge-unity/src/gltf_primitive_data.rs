/// Resolved data for a single glTF primitive, in Unity's left-handed coordinate system.
/// Used internally during mesh conversion before primitives are merged into a [`crate::unity_mesh::UnityMesh`].
pub(crate) struct GltfPrimitiveData {
    pub positions: Vec<[f32; 3]>,
    pub normals: Option<Vec<[f32; 3]>>,
    pub tangents: Option<Vec<[f32; 4]>>,
    /// One entry per UV channel (`TEXCOORD_0`, `TEXCOORD_1`, …).
    /// `uvs[k]` is `None` if that channel is absent on this primitive.
    pub uvs: Vec<Option<Vec<[f32; 2]>>>,
    pub wound: Vec<u32>,
}
