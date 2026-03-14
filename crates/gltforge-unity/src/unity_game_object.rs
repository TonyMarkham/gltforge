use crate::{unity_gltf::UnityGltf, write_name};
use gltforge_unity_core::unity_transform::IDENTITY;

/// Return the total number of GameObjects in the document.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_game_object_count(ptr: *const UnityGltf) -> u32 {
    unsafe { &*ptr }.game_objects.len() as u32
}

/// Return the name of GameObject `go_idx` as UTF-8 bytes, or null if absent.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_game_object_name(
    ptr: *const UnityGltf,
    go_idx: u32,
    out_len: *mut u32,
) -> *const u8 {
    let gltf = unsafe { &*ptr };
    let name = gltf.game_objects.get(&go_idx).map(|n| &n.name);
    unsafe { write_name(name, out_len) }
}

/// Return the number of children of GameObject `go_idx`.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_game_object_child_count(
    ptr: *const UnityGltf,
    go_idx: u32,
) -> u32 {
    unsafe { &*ptr }
        .game_objects
        .get(&go_idx)
        .map(|n| n.children.len() as u32)
        .unwrap_or(0)
}

/// Return the index of the `slot`-th child of GameObject `go_idx`.
/// Returns `u32::MAX` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_game_object_child(
    ptr: *const UnityGltf,
    go_idx: u32,
    slot: u32,
) -> u32 {
    unsafe { &*ptr }
        .game_objects
        .get(&go_idx)
        .and_then(|n| n.children.get(slot as usize))
        .copied()
        .unwrap_or(u32::MAX)
}

/// Return the number of mesh references on GameObject `go_idx`.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_game_object_mesh_count(
    ptr: *const UnityGltf,
    go_idx: u32,
) -> u32 {
    unsafe { &*ptr }
        .game_objects
        .get(&go_idx)
        .map(|n| n.mesh_indices.len() as u32)
        .unwrap_or(0)
}

/// Return the mesh index of the `slot`-th mesh reference on GameObject `go_idx`.
/// Returns `u32::MAX` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_game_object_mesh_index(
    ptr: *const UnityGltf,
    go_idx: u32,
    slot: u32,
) -> u32 {
    unsafe { &*ptr }
        .game_objects
        .get(&go_idx)
        .and_then(|n| n.mesh_indices.get(slot as usize))
        .copied()
        .unwrap_or(u32::MAX)
}

/// Write the local transform of GameObject `go_idx` into the caller-supplied 10-element `f32` buffer.
///
/// Layout: `[px, py, pz,  rx, ry, rz, rw,  sx, sy, sz]`
/// — position (Unity left-handed), rotation quaternion (xyzw, Unity left-handed), scale.
/// Falls back to identity if the index is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out` must point to at least 10 writable `f32` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_game_object_transform(
    ptr: *const UnityGltf,
    go_idx: u32,
    out: *mut f32,
) {
    let gltf = unsafe { &*ptr };
    let t = gltf
        .game_objects
        .get(&go_idx)
        .map(|n| &n.transform)
        .unwrap_or(&IDENTITY);
    let out = unsafe { std::slice::from_raw_parts_mut(out, 10) };
    out[0] = t.position[0];
    out[1] = t.position[1];
    out[2] = t.position[2];
    out[3] = t.rotation[0];
    out[4] = t.rotation[1];
    out[5] = t.rotation[2];
    out[6] = t.rotation[3];
    out[7] = t.scale[0];
    out[8] = t.scale[1];
    out[9] = t.scale[2];
}
