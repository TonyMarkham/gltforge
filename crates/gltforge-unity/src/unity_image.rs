use crate::unity_gltf::UnityGltf;

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

/// Return a pointer to the raw encoded image bytes (PNG, JPEG, …) for a
/// buffer-view-embedded image. `out_len` receives the byte count.
/// Returns null if `image_idx` is out of range or the image is URI-based.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
/// The returned pointer is valid for the lifetime of the [`UnityGltf`] handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_image_bytes(
    ptr: *const UnityGltf,
    image_idx: u32,
    out_len: *mut u32,
) -> *const u8 {
    let image = unsafe { &*ptr }.images.get(&image_idx);
    match image.and_then(|i| i.bytes.as_ref()) {
        Some(bytes) => {
            if !out_len.is_null() {
                unsafe { *out_len = bytes.len() as u32 };
            }
            bytes.as_ptr()
        }
        None => {
            if !out_len.is_null() {
                unsafe { *out_len = 0 };
            }
            std::ptr::null()
        }
    }
}
