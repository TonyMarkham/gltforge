use crate::unity_gltf::UnityGltf;

use gltforge_unity_core::ALPHA_MODE_OPAQUE;

// ─── FFI ────────────────────────────────────────────────────────────────────

/// Return the number of `GLTF/PbrMetallicRoughness` materials.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_count(ptr: *const UnityGltf) -> u32 {
    unsafe { &*ptr }.pbr_metallic_roughness.len() as u32
}

/// Return a pointer to the UTF-8 name bytes for material `mat_idx`.
/// `out_len` receives the byte length. Returns null if `mat_idx` is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out_len` may be null.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_name(
    ptr: *const UnityGltf,
    mat_idx: u32,
    out_len: *mut u32,
) -> *const u8 {
    unsafe {
        crate::write_name(
            { &*ptr }
                .pbr_metallic_roughness
                .get(&mat_idx)
                .map(|m| &m.name),
            out_len,
        )
    }
}

/// Return the image index for `_MainTex` (base color texture), or `-1` if absent.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_base_color_texture(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> i32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .and_then(|m| m.base_color_texture)
        .map_or(-1, |i| i as i32)
}

/// Return the image index for `_MetallicGlossMap` (metallic-roughness texture), or `-1` if absent.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_metallic_roughness_texture(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> i32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .and_then(|m| m.metallic_roughness_texture)
        .map_or(-1, |i| i as i32)
}

/// Return the image index for `_BumpMap` (normal texture), or `-1` if absent.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_normal_texture(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> i32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .and_then(|m| m.normal_texture)
        .map_or(-1, |i| i as i32)
}

/// Return the image index for `_OcclusionMap` (occlusion texture), or `-1` if absent.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_occlusion_texture(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> i32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .and_then(|m| m.occlusion_texture)
        .map_or(-1, |i| i as i32)
}

/// Return the image index for `_EmissionMap` (emissive texture), or `-1` if absent.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_emissive_texture(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> i32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .and_then(|m| m.emissive_texture)
        .map_or(-1, |i| i as i32)
}

/// Write the RGBA `_Color` (base color factor) into `out` (4 consecutive `f32`s).
/// Writes `[1, 1, 1, 1]` if `mat_idx` is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out` must point to at least 4 writable `f32`s.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_base_color_factor(
    ptr: *const UnityGltf,
    mat_idx: u32,
    out: *mut f32,
) {
    let f = unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .map_or([1.0_f32, 1.0, 1.0, 1.0], |m| m.base_color_factor);
    if !out.is_null() {
        unsafe {
            *out = f[0];
            *out.add(1) = f[1];
            *out.add(2) = f[2];
            *out.add(3) = f[3];
        }
    }
}

/// Return `_Metallic` (metallic factor) for material `mat_idx`. Returns `1.0` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_metallic_factor(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> f32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .map_or(1.0, |m| m.metallic_factor)
}

/// Return `_Glossiness` (roughness factor) for material `mat_idx`. Returns `1.0` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_roughness_factor(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> f32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .map_or(1.0, |m| m.roughness_factor)
}

/// Return `_BumpScale` (normal map scale) for material `mat_idx`. Returns `1.0` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_normal_scale(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> f32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .map_or(1.0, |m| m.normal_scale)
}

/// Return `_OcclusionStrength` for material `mat_idx`. Returns `1.0` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_occlusion_strength(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> f32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .map_or(1.0, |m| m.occlusion_strength)
}

/// Write the RGB `_EmissionColor` (emissive factor) into `out` (3 consecutive `f32`s).
/// Writes `[0, 0, 0]` if `mat_idx` is out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle. `out` must point to at least 3 writable `f32`s.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_emissive_factor(
    ptr: *const UnityGltf,
    mat_idx: u32,
    out: *mut f32,
) {
    let f = unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .map_or([0.0_f32, 0.0, 0.0], |m| m.emissive_factor);
    if !out.is_null() {
        unsafe {
            *out = f[0];
            *out.add(1) = f[1];
            *out.add(2) = f[2];
        }
    }
}

/// Return `_Cutoff` (alpha cutoff) for material `mat_idx`. Returns `0.5` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_alpha_cutoff(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> f32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .map_or(0.5, |m| m.alpha_cutoff)
}

/// Return `_Mode` (alpha mode) for material `mat_idx`:
/// `0` = OPAQUE, `1` = MASK, `2` = BLEND. Returns `0` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_alpha_mode(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> u32 {
    unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .map_or(ALPHA_MODE_OPAQUE, |m| m.alpha_mode)
}

/// Return `_Cull` for material `mat_idx`:
/// `0` = `CullMode.Off` (double-sided), `2` = `CullMode.Back` (single-sided).
/// Returns `2` if out of range.
///
/// # Safety
/// `ptr` must be a valid, non-null handle.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn gltforge_pbr_metallic_roughness_cull(
    ptr: *const UnityGltf,
    mat_idx: u32,
) -> u32 {
    let double_sided = unsafe { &*ptr }
        .pbr_metallic_roughness
        .get(&mat_idx)
        .is_some_and(|m| m.double_sided);
    if double_sided { 0 } else { 2 }
}
