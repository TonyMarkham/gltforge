pub mod build;
pub mod error;
pub mod export_context;
pub mod mesh_data;
pub mod node_data;
pub mod submesh_data;

// -------------------------------------------------------------------------- //

use export_context::ExportContext;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

// -------------------------------------------------------------------------- //

/// Create a new export context.
///
/// Must be passed to [`gltforge_export_finish`] or [`gltforge_export_free`] to avoid a memory
/// leak.
#[unsafe(no_mangle)]
pub extern "C" fn gltforge_export_begin() -> *mut ExportContext {
    Box::into_raw(Box::new(ExportContext::new()))
}

/// Free an export context without writing any files.
///
/// # Safety
/// `ctx` must be a valid pointer returned by [`gltforge_export_begin`] that has not yet been
/// consumed by [`gltforge_export_finish`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_free(ctx: *mut ExportContext) {
    if !ctx.is_null() {
        unsafe { drop(Box::from_raw(ctx)) };
    }
}

/// Add a node (GameObject) to the export context. Returns the node index.
///
/// `parent_idx` is `-1` for root nodes, otherwise the index returned by a prior call.
/// `pos`, `rot`, `scale` point to 3, 4, and 3 contiguous `f32` values respectively (Unity space).
/// Pass null for any component to omit it from the output (glTF defaults apply).
///
/// # Safety
/// `ctx` must be valid. `name_ptr` must point to `name_len` valid UTF-8 bytes, or be null.
/// Each non-null transform pointer must point to the required number of `f32` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_add_node(
    ctx: *mut ExportContext,
    name_ptr: *const u8,
    name_len: u32,
    parent_idx: i32,
    pos: *const f32,
    rot: *const f32,
    scale: *const f32,
) -> u32 {
    let ctx = unsafe { &mut *ctx };

    let name = unsafe { read_name(name_ptr, name_len) };
    let parent = if parent_idx < 0 {
        None
    } else {
        Some(parent_idx as u32)
    };
    let translation = unsafe { read_f32s::<3>(pos) };
    let rotation = unsafe { read_f32s::<4>(rot) };
    let scale = unsafe { read_f32s::<3>(scale) };

    ctx.add_node(name, parent, translation, rotation, scale)
}

/// Add a mesh to the export context. Returns the mesh index.
///
/// # Safety
/// `ctx` must be valid. `name_ptr` must point to `name_len` valid UTF-8 bytes, or be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_add_mesh(
    ctx: *mut ExportContext,
    name_ptr: *const u8,
    name_len: u32,
) -> u32 {
    let ctx = unsafe { &mut *ctx };
    ctx.add_mesh(unsafe { read_name(name_ptr, name_len) })
}

/// Set vertex positions for mesh `mesh_idx`.
/// `ptr` points to `f32_count` floats as tightly packed `[x, y, z]` triples (Unity space).
///
/// # Safety
/// `ctx` and `ptr` must be valid. `f32_count` must be a multiple of 3.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_mesh_set_positions(
    ctx: *mut ExportContext,
    mesh_idx: u32,
    ptr: *const f32,
    f32_count: u32,
) {
    let ctx = unsafe { &mut *ctx };
    let floats = unsafe { std::slice::from_raw_parts(ptr, f32_count as usize) };
    let positions = floats.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
    ctx.set_positions(mesh_idx, positions);
}

/// Set vertex normals for mesh `mesh_idx`.
/// `ptr` points to `f32_count` floats as tightly packed `[x, y, z]` triples (Unity space).
///
/// # Safety
/// `ctx` and `ptr` must be valid. `f32_count` must be a multiple of 3.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_mesh_set_normals(
    ctx: *mut ExportContext,
    mesh_idx: u32,
    ptr: *const f32,
    f32_count: u32,
) {
    let ctx = unsafe { &mut *ctx };
    let floats = unsafe { std::slice::from_raw_parts(ptr, f32_count as usize) };
    let normals = floats.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect();
    ctx.set_normals(mesh_idx, normals);
}

/// Set a UV channel for mesh `mesh_idx`.
/// `channel` is 0-based. `ptr` points to `f32_count` floats as `[u, v]` pairs (Unity space).
///
/// # Safety
/// `ctx` and `ptr` must be valid. `f32_count` must be a multiple of 2.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_mesh_set_uvs(
    ctx: *mut ExportContext,
    mesh_idx: u32,
    channel: u32,
    ptr: *const f32,
    f32_count: u32,
) {
    let ctx = unsafe { &mut *ctx };
    let floats = unsafe { std::slice::from_raw_parts(ptr, f32_count as usize) };
    let uvs = floats.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
    ctx.set_uvs(mesh_idx, channel, uvs);
}

/// Add a sub-mesh with 16-bit indices to mesh `mesh_idx`.
///
/// # Safety
/// `ctx` and `ptr` must be valid. `index_count` must be a multiple of 3.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_mesh_add_submesh_u16(
    ctx: *mut ExportContext,
    mesh_idx: u32,
    ptr: *const u16,
    index_count: u32,
) {
    let ctx = unsafe { &mut *ctx };
    let slice = unsafe { std::slice::from_raw_parts(ptr, index_count as usize) };
    let indices = slice.iter().map(|&i| i as u32).collect();
    ctx.add_submesh(mesh_idx, indices);
}

/// Add a sub-mesh with 32-bit indices to mesh `mesh_idx`.
///
/// # Safety
/// `ctx` and `ptr` must be valid. `index_count` must be a multiple of 3.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_mesh_add_submesh_u32(
    ctx: *mut ExportContext,
    mesh_idx: u32,
    ptr: *const u32,
    index_count: u32,
) {
    let ctx = unsafe { &mut *ctx };
    let slice = unsafe { std::slice::from_raw_parts(ptr, index_count as usize) };
    ctx.add_submesh(mesh_idx, slice.to_vec());
}

/// Attach mesh `mesh_idx` to node `node_idx`.
///
/// # Safety
/// `ctx` must be valid.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_node_set_mesh(
    ctx: *mut ExportContext,
    node_idx: u32,
    mesh_idx: u32,
) {
    let ctx = unsafe { &mut *ctx };
    ctx.set_node_mesh(node_idx, mesh_idx);
}

/// Build and write the `.gltf` and `.bin` files.
/// `path` is the output `.gltf` file path (null-terminated UTF-8).
/// Returns `1` on success, `0` on failure. Consumes and frees the context in both cases.
///
/// # Safety
/// `ctx` must be a valid pointer returned by [`gltforge_export_begin`].
/// `path` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_finish(
    ctx: *mut ExportContext,
    path: *const c_char,
) -> u8 {
    let result = std::panic::catch_unwind(|| {
        let ctx = unsafe { Box::from_raw(ctx) };
        let path_str = unsafe { CStr::from_ptr(path) }.to_str().ok()?;
        build::write(*ctx, Path::new(path_str)).ok()
    });
    result.ok().flatten().map(|_| 1u8).unwrap_or(0)
}

/// Build and write a single `.glb` file (binary glTF).
/// `path` is the output `.glb` file path (null-terminated UTF-8).
/// Returns `1` on success, `0` on failure. Consumes and frees the context in both cases.
///
/// # Safety
/// `ctx` must be a valid pointer returned by [`gltforge_export_begin`].
/// `path` must be a valid null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_export_finish_glb(
    ctx: *mut ExportContext,
    path: *const c_char,
) -> u8 {
    let result = std::panic::catch_unwind(|| {
        let ctx = unsafe { Box::from_raw(ctx) };
        let path_str = unsafe { CStr::from_ptr(path) }.to_str().ok()?;
        build::write_glb(*ctx, Path::new(path_str)).ok()
    });
    result.ok().flatten().map(|_| 1u8).unwrap_or(0)
}

// -------------------------------------------------------------------------- //

unsafe fn read_name(ptr: *const u8, len: u32) -> Option<String> {
    if ptr.is_null() || len == 0 {
        return None;
    }
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    std::str::from_utf8(bytes).ok().map(|s| s.to_string())
}

unsafe fn read_f32s<const N: usize>(ptr: *const f32) -> Option<[f32; N]> {
    if ptr.is_null() {
        return None;
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, N) };
    let mut arr = [0f32; N];
    arr.copy_from_slice(slice);
    Some(arr)
}
