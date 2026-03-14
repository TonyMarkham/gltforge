/// Alpha mode constants for [`UnityPbrMetallicRoughness::alpha_mode`].
pub const ALPHA_MODE_OPAQUE: u32 = 0;
pub const ALPHA_MODE_MASK: u32 = 1;
pub const ALPHA_MODE_BLEND: u32 = 2;

/// A glTF 2.0 PBR metallic-roughness material in Unity-ready form.
///
/// All texture slots hold an **image index** (into the parent document's image map),
/// already resolved through the glTF texture → image indirection.
///
/// Scalar values are stored in glTF convention and must be mapped to the target
/// shader's property names by the importer (BiRP and URP use different names).
pub struct UnityPbrMetallicRoughness {
    /// The material name. Falls back to the material index as a string if unnamed.
    pub name: String,

    // ── Textures (image indices; `None` = absent) ────────────────────────────
    /// Base color (albedo) texture image index.
    pub base_color_texture: Option<u32>,

    /// Metallic-roughness texture image index.
    /// glTF packing: G channel = roughness, B channel = metallic.
    pub metallic_roughness_texture: Option<u32>,

    /// Tangent-space normal map image index.
    pub normal_texture: Option<u32>,

    /// Ambient occlusion texture image index. Stored in the R channel.
    pub occlusion_texture: Option<u32>,

    /// Emissive texture image index.
    pub emissive_texture: Option<u32>,

    // ── Scalar factors ───────────────────────────────────────────────────────
    /// RGBA base color factor. Default `[1, 1, 1, 1]`.
    pub base_color_factor: [f32; 4],

    /// Metallic factor in `[0, 1]`. Default `1`.
    pub metallic_factor: f32,

    /// Perceptual roughness factor in `[0, 1]`. Default `1`.
    /// This is glTF roughness, **not** Unity smoothness — importer must invert for shaders
    /// that expect smoothness (`smoothness = 1 - roughness_factor`).
    pub roughness_factor: f32,

    /// Normal map scale multiplier. Default `1`.
    pub normal_scale: f32,

    /// Occlusion strength in `[0, 1]`. Default `1`.
    pub occlusion_strength: f32,

    /// RGB emissive factor. Default `[0, 0, 0]`.
    pub emissive_factor: [f32; 3],

    // ── Rendering mode ───────────────────────────────────────────────────────
    /// Alpha cutoff threshold for [`ALPHA_MODE_MASK`]. Default `0.5`.
    pub alpha_cutoff: f32,

    /// Alpha rendering mode: `0` = [`ALPHA_MODE_OPAQUE`], `1` = [`ALPHA_MODE_MASK`],
    /// `2` = [`ALPHA_MODE_BLEND`].
    pub alpha_mode: u32,

    /// `false` → `CullMode.Back` (2, single-sided); `true` → `CullMode.Off` (0, double-sided).
    pub double_sided: bool,
}
