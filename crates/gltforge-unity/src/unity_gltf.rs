use crate::convert;
use gltforge_unity_core::{UnityGameObject, UnityImage, UnityMesh, UnityPbrMetallicRoughness};

use std::{collections::HashMap, ffi::CStr, os::raw::c_char, path::Path, sync::Arc};

/// A Unity-shaped glTF — same structural relationships as glTF but with all
/// data pre-converted to Unity's left-handed coordinate system.
pub struct UnityGltf {
    /// The name of the default scene. Falls back to the source filename (without extension)
    /// if the glTF scene has no name.
    pub scene_name: String,

    /// Indices of the root GameObjects in the default scene.
    pub root_game_objects: Vec<u32>,

    /// All GameObjects, keyed by their glTF node index.
    pub game_objects: HashMap<u32, UnityGameObject>,

    /// All meshes, keyed by their glTF mesh index.
    /// Each [`UnityMesh`] contains a shared vertex array and one sub-mesh per glTF primitive.
    pub meshes: HashMap<u32, UnityMesh>,

    /// All images, keyed by their glTF image index.
    pub images: HashMap<u32, UnityImage>,

    /// All `GLTF/PbrMetallicRoughness` materials, keyed by their glTF material index.
    pub pbr_metallic_roughness: HashMap<u32, UnityPbrMetallicRoughness>,
}

/// Increment the reference count of a [`UnityGltf`] handle.
///
/// # Safety
/// `ptr` must have been returned by [`gltforge_load`] and not yet fully released.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_retain(ptr: *const UnityGltf) {
    if !ptr.is_null() {
        unsafe { Arc::increment_strong_count(ptr) };
    }
}

/// Decrement the reference count of a [`UnityGltf`] handle.
/// Frees all data when the count reaches zero.
///
/// # Safety
/// `ptr` must have been returned by [`gltforge_load`] or [`gltforge_retain`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_release(ptr: *const UnityGltf) {
    if !ptr.is_null() {
        unsafe { drop(Arc::from_raw(ptr)) };
    }
}

/// Parse a glTF file and build a [`UnityGltf`].
///
/// Returns an `Arc<UnityGltf>` as an opaque pointer, or null on any error.
/// The caller must eventually pass the pointer to [`gltforge_release`].
///
/// # Safety
/// `path` must be a valid, null-terminated UTF-8 string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_load(path: *const c_char) -> *const UnityGltf {
    let result = std::panic::catch_unwind(|| {
        let path_str = unsafe { CStr::from_ptr(path) }.to_str().ok()?;
        let path = Path::new(path_str);
        let base_dir = path.parent()?;

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        let data = std::fs::read(path).ok()?;
        let (gltf, buffers) = if data.starts_with(b"glTF") {
            gltforge::parser::parse_glb(&data).ok()?
        } else {
            let json = std::str::from_utf8(&data).ok()?;
            let gltf = gltforge::parser::parse(json).ok()?;
            let buffers = gltforge::parser::load_buffers(&gltf, base_dir).ok()?;
            (gltf, buffers)
        };
        let unity_gltf = convert::build_unity_gltf(&gltf, &buffers, file_stem).ok()?;

        Some(Arc::into_raw(Arc::new(unity_gltf)))
    });

    result.ok().flatten().unwrap_or(std::ptr::null())
}
