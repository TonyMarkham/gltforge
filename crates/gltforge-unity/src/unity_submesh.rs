use crate::{unity_gltf::UnityGltf, unity_indices::UnityIndices};

/// A single glTF primitive converted to a Unity sub-mesh.
/// Indices are pre-offset to reference the shared vertex array on [`crate::unity_mesh::UnityMesh`].
pub struct UnitySubMesh {
    /// Triangle indices with winding order reversed for Unity's left-handed convention.
    /// Absolute offsets into the parent [`UnityMesh::vertices`] array.
    pub indices: UnityIndices,
}

/// Return the number of sub-meshes in mesh `mesh_idx`.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_submesh_count(ptr: *const UnityGltf, mesh_idx: u32) -> u32 {
    unsafe { &*ptr }
        .meshes
        .get(&mesh_idx)
        .map(|m| m.sub_meshes.len() as u32)
        .unwrap_or(0)
}

/// Return a pointer to the `u16` index data for sub-mesh `submesh` of mesh `mesh_idx`.
/// Returns null if the mesh uses `u32` indices or the indices are out of range.
/// `out_len` receives the number of index values (not bytes).
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_submesh_indices_u16(
    ptr: *const UnityGltf,
    mesh_idx: u32,
    submesh: u32,
    out_len: *mut u32,
) -> *const u16 {
    let gltf = unsafe { &*ptr };
    match gltf
        .meshes
        .get(&mesh_idx)
        .and_then(|m| m.sub_meshes.get(submesh as usize))
        .map(|s| &s.indices)
    {
        Some(UnityIndices::U16(v)) => {
            if !out_len.is_null() {
                unsafe { *out_len = v.len() as u32 };
            }
            v.as_ptr()
        }
        _ => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}

/// Return a pointer to the `u32` index data for sub-mesh `submesh` of mesh `mesh_idx`.
/// Returns null if the mesh uses `u16` indices or the indices are out of range.
/// `out_len` receives the number of index values (not bytes).
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_submesh_indices_u32(
    ptr: *const UnityGltf,
    mesh_idx: u32,
    submesh: u32,
    out_len: *mut u32,
) -> *const u32 {
    let gltf = unsafe { &*ptr };
    match gltf
        .meshes
        .get(&mesh_idx)
        .and_then(|m| m.sub_meshes.get(submesh as usize))
        .map(|s| &s.indices)
    {
        Some(UnityIndices::U32(v)) => {
            if !out_len.is_null() {
                unsafe { *out_len = v.len() as u32 };
            }
            v.as_ptr()
        }
        _ => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}
