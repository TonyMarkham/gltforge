use crate::unity_gltf::UnityGltf;

/// A glTF image entry, providing the URI needed for Unity to load the texture asset.
pub struct UnityImage {
    /// The image name. Falls back to the image index as a string if unnamed.
    pub name: String,

    /// The URI of the image, if it references an external file.
    /// `None` for buffer-view-embedded images.
    pub uri: Option<String>,
}

// ─── FFI ────────────────────────────────────────────────────────────────────

/// Return the number of images.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_image_count(ptr: *const UnityGltf) -> u32 {
    unsafe { &*ptr }.images.len() as u32
}

/// Return a pointer to the UTF-8 name bytes for image `image_idx`.
/// `out_len` receives the byte length. Returns null if `image_idx` is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_image_name(
    ptr: *const UnityGltf,
    image_idx: u32,
    out_len: *mut u32,
) -> *const u8 {
    unsafe { crate::write_name({ &*ptr }.images.get(&image_idx).map(|i| &i.name), out_len) }
}

/// Return a pointer to the UTF-8 URI bytes for image `image_idx`.
/// `out_len` receives the byte length.
/// Returns null if `image_idx` is out of range or the image is buffer-view-embedded.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_image_uri(
    ptr: *const UnityGltf,
    image_idx: u32,
    out_len: *mut u32,
) -> *const u8 {
    unsafe {
        crate::write_name(
            { &*ptr }
                .images
                .get(&image_idx)
                .and_then(|i| i.uri.as_ref()),
            out_len,
        )
    }
}
