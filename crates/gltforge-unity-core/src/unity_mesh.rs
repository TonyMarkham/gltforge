use crate::unity_submesh::UnitySubMesh;

/// A Unity-shaped glTF mesh: one shared vertex array with N sub-meshes (one per glTF primitive).
/// Maps directly to a `UnityEngine.Mesh` with `subMeshCount` sub-meshes.
pub struct UnityMesh {
    /// Mesh name. Falls back to the glTF mesh index if the source mesh is unnamed.
    pub name: String,

    /// All vertex positions across all primitives, concatenated.
    /// Left-handed coordinate system (X negated relative to glTF).
    /// Tightly packed `[x, y, z]` floats — maps to `mesh.vertices`.
    pub vertices: Vec<[f32; 3]>,

    /// Vertex normals, same length as `vertices`. Empty if the source mesh has no normals.
    /// Left-handed coordinate system (X negated relative to glTF).
    /// Tightly packed `[x, y, z]` floats — maps to `mesh.normals`.
    pub normals: Vec<[f32; 3]>,

    /// Vertex tangents, same length as `vertices`. Empty if the source mesh has no tangents.
    /// Left-handed coordinate system (X and W negated relative to glTF).
    /// Tightly packed `[x, y, z, w]` floats — maps to `mesh.tangents`.
    pub tangents: Vec<[f32; 4]>,

    /// UV channels, densely packed from channel 0.
    /// `uvs[k]` holds all vertices for `TEXCOORD_k` (V-flipped for Unity's bottom-left origin).
    /// Only channels present on every primitive are included; the vec stops at the first absent channel.
    /// Maps to `mesh.SetUVs(k, uvs[k])`.
    pub uvs: Vec<Vec<[f32; 2]>>,

    /// One sub-mesh per glTF primitive — maps to `mesh.SetTriangles(tris, submeshIndex)`.
    pub sub_meshes: Vec<UnitySubMesh>,
}
