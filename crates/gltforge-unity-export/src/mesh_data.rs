use crate::submesh_data::SubmeshData;

/// A Unity-shaped mesh ready for export.
/// All data is in Unity's left-handed coordinate system; the build step inverts
/// handedness to produce right-handed glTF output.
pub struct MeshData {
    /// Mesh name, or `None` to omit from the glTF output.
    pub name: Option<String>,

    /// Vertex positions. Tightly packed `[x, y, z]` floats (Unity space, X negated vs glTF).
    pub positions: Vec<[f32; 3]>,

    /// Vertex normals, same length as `positions`. Empty if the mesh has no normals.
    pub normals: Vec<[f32; 3]>,

    /// UV channels. `uvs[k]` holds all vertices for `TEXCOORD_k` (Unity bottom-left origin, V
    /// will be flipped during the build step).
    pub uvs: Vec<Vec<[f32; 2]>>,

    /// One sub-mesh per glTF primitive.
    pub submeshes: Vec<SubmeshData>,
}
