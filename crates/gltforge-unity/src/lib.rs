pub mod convert;
pub mod error;
pub mod mesh;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::sync::Arc;

use mesh::{UnityIndices, UnityMesh};

// ---- load / retain / release ------------------------------------------------

/// Parse a glTF file and build a [`UnityMesh`] for the given node.
///
/// Loads all primitives of the node's mesh as submeshes. Returns an
/// `Arc<UnityMesh>` as an opaque pointer, or null on any error.
/// The caller must eventually pass the pointer to [`gltforge_mesh_release`].
///
/// # Safety
/// `path` must be a valid, null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_load_mesh(
    path: *const c_char,
    node_idx: u32,
) -> *const UnityMesh {
    let result = std::panic::catch_unwind(|| {
        let path_str = unsafe { CStr::from_ptr(path) }.to_str().ok()?;
        let path = Path::new(path_str);
        let base_dir = path.parent()?;

        let json = std::fs::read_to_string(path).ok()?;
        let gltf = gltforge::parser::parse(&json).ok()?;
        let buffers = gltforge::parser::load_buffers(&gltf, base_dir).ok()?;
        let unity_mesh = convert::build_unity_mesh(&gltf, &buffers, node_idx).ok()?;

        Some(Arc::into_raw(Arc::new(unity_mesh)))
    });

    result.ok().flatten().unwrap_or(std::ptr::null())
}

/// Increment the reference count of a [`UnityMesh`] handle.
///
/// # Safety
/// `ptr` must have been returned by [`gltforge_load_mesh`] and not yet fully released.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_retain(ptr: *const UnityMesh) {
    if !ptr.is_null() {
        unsafe { Arc::increment_strong_count(ptr) };
    }
}

/// Decrement the reference count of a [`UnityMesh`] handle.
/// Frees the underlying data when the count reaches zero.
///
/// # Safety
/// `ptr` must have been returned by [`gltforge_load_mesh`] or [`gltforge_mesh_retain`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_release(ptr: *const UnityMesh) {
    if !ptr.is_null() {
        unsafe { drop(Arc::from_raw(ptr)) };
    }
}

// ---- metadata ---------------------------------------------------------------

/// Return a pointer to the mesh name as UTF-8 bytes (not null-terminated).
/// `out_len` receives the byte length. Use `Marshal.PtrToStringUTF8(ptr, len)` in C#.
/// The pointer is valid as long as the handle is alive.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_name(ptr: *const UnityMesh, out_len: *mut u32) -> *const u8 {
    let name = &unsafe { &*ptr }.name;
    if !out_len.is_null() {
        unsafe { *out_len = name.len() as u32 };
    }
    name.as_ptr()
}

/// Return the total number of vertices across all submeshes.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_vertex_count(ptr: *const UnityMesh) -> u32 {
    unsafe { &*ptr }.positions.len() as u32
}

/// Return the number of submeshes (one per glTF primitive).
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_submesh_count(ptr: *const UnityMesh) -> u32 {
    unsafe { &*ptr }.submeshes.len() as u32
}

// ---- vertex data ------------------------------------------------------------

/// Return a pointer to the position data (tightly packed `[x, y, z]` floats, left-handed).
/// `out_len` receives the total number of `float` values (`vertex_count × 3`).
/// The pointer is valid as long as the handle is alive.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_positions(
    ptr: *const UnityMesh,
    out_len: *mut u32,
) -> *const f32 {
    let positions = unsafe { &*ptr }.positions.as_flattened();
    if !out_len.is_null() {
        unsafe { *out_len = positions.len() as u32 };
    }
    positions.as_ptr()
}

// ---- index data -------------------------------------------------------------

/// Returns `16` if all submesh index buffers are `u16`, `32` if they are `u32`.
/// The format is the same for every submesh and is determined by total vertex count.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_index_format(ptr: *const UnityMesh) -> u32 {
    match unsafe { &*ptr }.submeshes.first() {
        Some(sub) => match sub.indices {
            UnityIndices::U16(_) => 16,
            UnityIndices::U32(_) => 32,
        },
        None => 16,
    }
}

/// Return a pointer to the index data for `submesh_idx` as `u16`.
/// `out_len` receives the number of index values.
/// Returns null if the format is `u32` or the submesh index is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_submesh_indices_u16(
    ptr: *const UnityMesh,
    submesh_idx: u32,
    out_len: *mut u32,
) -> *const u16 {
    let mesh = unsafe { &*ptr };
    let Some(submesh) = mesh.submeshes.get(submesh_idx as usize) else {
        if !out_len.is_null() {
            unsafe { *out_len = 0 };
        }
        return std::ptr::null();
    };
    match &submesh.indices {
        UnityIndices::U16(v) => {
            if !out_len.is_null() {
                unsafe { *out_len = v.len() as u32 };
            }
            v.as_ptr()
        }
        UnityIndices::U32(_) => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}

/// Return a pointer to the index data for `submesh_idx` as `u32`.
/// `out_len` receives the number of index values.
/// Returns null if the format is `u16` or the submesh index is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_mesh_submesh_indices_u32(
    ptr: *const UnityMesh,
    submesh_idx: u32,
    out_len: *mut u32,
) -> *const u32 {
    let mesh = unsafe { &*ptr };
    let Some(submesh) = mesh.submeshes.get(submesh_idx as usize) else {
        if !out_len.is_null() {
            unsafe { *out_len = 0 };
        }
        return std::ptr::null();
    };
    match &submesh.indices {
        UnityIndices::U32(v) => {
            if !out_len.is_null() {
                unsafe { *out_len = v.len() as u32 };
            }
            v.as_ptr()
        }
        UnityIndices::U16(_) => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}
