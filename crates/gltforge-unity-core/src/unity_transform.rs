pub const ZERO_VEC3: [f32; 3] = [0.0, 0.0, 0.0];
pub const ONE_VEC3: [f32; 3] = [1.0, 1.0, 1.0];
pub const IDENTITY_QUATERNION: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// The local transform of a Unity node, pre-converted to Unity's left-handed coordinate system.
///
/// Regardless of whether the source glTF node used a `matrix` or TRS properties,
/// this always holds a decomposed TRS ready to hand to `UnityEngine.Transform`.
pub struct UnityTransform {
    /// Local position — X negated relative to glTF.
    pub position: [f32; 3],

    /// Local rotation as a unit quaternion `[x, y, z, w]` — X and W negated relative to glTF.
    pub rotation: [f32; 4],

    /// Local scale — unchanged from glTF.
    pub scale: [f32; 3],
}

pub const IDENTITY: UnityTransform = UnityTransform {
    position: ZERO_VEC3,
    rotation: IDENTITY_QUATERNION,
    scale: ONE_VEC3,
};

impl Default for UnityTransform {
    fn default() -> Self {
        IDENTITY
    }
}
