use crate::{unity_gltf::UnityGltf, unity_indices::UnityIndices, unity_submesh::UnitySubMesh};

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

    /// One sub-mesh per glTF primitive — maps to `mesh.SetTriangles(tris, submeshIndex)`.
    pub sub_meshes: Vec<UnitySubMesh>,
}

/// Return the number of meshes in the document.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_count(ptr: *const UnityGltf) -> u32 {
    unsafe { &*ptr }.meshes.len() as u32
}

/// Return the name of mesh `mesh_idx` as UTF-8 bytes (not null-terminated).
/// Always non-null — unnamed meshes use their index as the name.
/// `out_len` receives the byte length.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_name(
    ptr: *const UnityGltf,
    mesh_idx: u32,
    out_len: *mut u32,
) -> *const u8 {
    let gltf = unsafe { &*ptr };
    match gltf.meshes.get(&mesh_idx) {
        Some(m) => {
            if !out_len.is_null() {
                unsafe { *out_len = m.name.len() as u32 };
            }
            m.name.as_ptr()
        }
        None => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}

/// Return the total vertex count for mesh `mesh_idx` (across all sub-meshes).
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_vertex_count(ptr: *const UnityGltf, mesh_idx: u32) -> u32 {
    unsafe { &*ptr }
        .meshes
        .get(&mesh_idx)
        .map(|m| m.vertices.len() as u32)
        .unwrap_or(0)
}

/// Return `16` or `32` indicating the index format for mesh `mesh_idx`.
/// The format is uniform across all sub-meshes and is determined by total vertex count.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_index_format(ptr: *const UnityGltf, mesh_idx: u32) -> u32 {
    let gltf = unsafe { &*ptr };
    match gltf
        .meshes
        .get(&mesh_idx)
        .and_then(|m| m.sub_meshes.first())
        .map(|s| &s.indices)
    {
        Some(UnityIndices::U32(_)) => 32,
        _ => 16,
    }
}

/// Return a pointer to the normal data for mesh `mesh_idx`.
/// `out_len` receives the total number of `f32` values (`vertex_count × 3`).
/// Returns null with `out_len = 0` if the mesh has no normals.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_normals(
    ptr: *const UnityGltf,
    mesh_idx: u32,
    out_len: *mut u32,
) -> *const f32 {
    let gltf = unsafe { &*ptr };
    match gltf.meshes.get(&mesh_idx) {
        Some(m) if !m.normals.is_empty() => {
            let flat = m.normals.as_flattened();
            if !out_len.is_null() {
                unsafe { *out_len = flat.len() as u32 };
            }
            flat.as_ptr()
        }
        _ => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}

/// Return a pointer to the position data for mesh `mesh_idx`.
/// `out_len` receives the total number of `f32` values (`vertex_count × 3`).
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_positions(
    ptr: *const UnityGltf,
    mesh_idx: u32,
    out_len: *mut u32,
) -> *const f32 {
    let gltf = unsafe { &*ptr };
    match gltf.meshes.get(&mesh_idx) {
        Some(m) => {
            let flat = m.vertices.as_flattened();
            if !out_len.is_null() {
                unsafe { *out_len = flat.len() as u32 };
            }
            flat.as_ptr()
        }
        None => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}
