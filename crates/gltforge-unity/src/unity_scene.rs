use crate::{unity_gltf::UnityGltf, write_name};

/// Return the scene name as UTF-8 bytes (not null-terminated).
/// Always non-null — unnamed scenes fall back to the source filename.
/// `out_len` receives the byte length.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_scene_name(
    ptr: *const UnityGltf,
    out_len: *mut u32,
) -> *const u8 {
    let gltf = unsafe { &*ptr };
    unsafe { write_name(Some(&gltf.scene_name), out_len) }
}

/// Return the number of root nodes in the default scene.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_root_node_count(ptr: *const UnityGltf) -> u32 {
    unsafe { &*ptr }.root_nodes.len() as u32
}

/// Return the node index of the `slot`-th root node.
/// Returns `u32::MAX` if `slot` is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_root_node_index(ptr: *const UnityGltf, slot: u32) -> u32 {
    unsafe { &*ptr }
        .root_nodes
        .get(slot as usize)
        .copied()
        .unwrap_or(u32::MAX)
}
