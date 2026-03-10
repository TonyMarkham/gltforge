use crate::{unity_gltf::UnityGltf, unity_node_transform::UnityNodeTransform, write_name};

/// A Unity-shaped glTF node. References children and meshes by index,
/// mirroring the glTF node structure.
pub struct UnityNode {
    /// The node name. Falls back to the glTF node index if the source node is unnamed.
    pub name: String,

    /// Indices of child nodes (into [`UnityGltf::nodes`]).
    pub children: Vec<u32>,

    /// Indices of meshes referenced by this node (into [`UnityGltf::meshes`]).
    /// In glTF a node references at most one mesh, but that mesh may have
    /// multiple primitives stored as separate entries.
    pub mesh_indices: Vec<u32>,

    /// Local transform in Unity's left-handed coordinate system.
    pub transform: UnityNodeTransform,
}

/// Return the total number of nodes in the document.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_node_count(ptr: *const UnityGltf) -> u32 {
    unsafe { &*ptr }.nodes.len() as u32
}

/// Return the name of node `node_idx` as UTF-8 bytes, or null if absent.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_node_name(
    ptr: *const UnityGltf,
    node_idx: u32,
    out_len: *mut u32,
) -> *const u8 {
    let gltf = unsafe { &*ptr };
    let name = gltf.nodes.get(&node_idx).map(|n| &n.name);
    unsafe { write_name(name, out_len) }
}

/// Return the number of children of node `node_idx`.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_node_child_count(ptr: *const UnityGltf, node_idx: u32) -> u32 {
    unsafe { &*ptr }
        .nodes
        .get(&node_idx)
        .map(|n| n.children.len() as u32)
        .unwrap_or(0)
}

/// Return the node index of the `slot`-th child of node `node_idx`.
/// Returns `u32::MAX` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_node_child(
    ptr: *const UnityGltf,
    node_idx: u32,
    slot: u32,
) -> u32 {
    unsafe { &*ptr }
        .nodes
        .get(&node_idx)
        .and_then(|n| n.children.get(slot as usize))
        .copied()
        .unwrap_or(u32::MAX)
}

/// Return the number of mesh references on node `node_idx`.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_node_mesh_count(ptr: *const UnityGltf, node_idx: u32) -> u32 {
    unsafe { &*ptr }
        .nodes
        .get(&node_idx)
        .map(|n| n.mesh_indices.len() as u32)
        .unwrap_or(0)
}

/// Return the mesh index of the `slot`-th mesh reference on node `node_idx`.
/// Returns `u32::MAX` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_node_mesh_index(
    ptr: *const UnityGltf,
    node_idx: u32,
    slot: u32,
) -> u32 {
    unsafe { &*ptr }
        .nodes
        .get(&node_idx)
        .and_then(|n| n.mesh_indices.get(slot as usize))
        .copied()
        .unwrap_or(u32::MAX)
}

/// Write the local transform of node `node_idx` into the caller-supplied 10-element `f32` buffer.
///
/// Layout: `[px, py, pz,  rx, ry, rz, rw,  sx, sy, sz]`
/// — position (Unity left-handed), rotation quaternion (xyzw, Unity left-handed), scale.
/// Falls back to identity if the node index is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out` must point to at least 10 writable `f32` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_node_transform(
    ptr: *const UnityGltf,
    node_idx: u32,
    out: *mut f32,
) {
    use crate::unity_node_transform::IDENTITY;
    let gltf = unsafe { &*ptr };
    let t = gltf
        .nodes
        .get(&node_idx)
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
