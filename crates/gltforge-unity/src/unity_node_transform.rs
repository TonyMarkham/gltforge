/// The local transform of a Unity node, pre-converted to Unity's left-handed coordinate system.
///
/// Regardless of whether the source glTF node used a `matrix` or TRS properties,
/// this always holds a decomposed TRS ready to hand to `UnityEngine.Transform`.
pub struct UnityNodeTransform {
    /// Local position — X negated relative to glTF.
    pub position: [f32; 3],

    /// Local rotation as a unit quaternion `[x, y, z, w]` — X and W negated relative to glTF.
    pub rotation: [f32; 4],

    /// Local scale — unchanged from glTF.
    pub scale: [f32; 3],
}

pub const IDENTITY: UnityNodeTransform = UnityNodeTransform {
    position: [0.0, 0.0, 0.0],
    rotation: [0.0, 0.0, 0.0, 1.0],
    scale: [1.0, 1.0, 1.0],
};

impl Default for UnityNodeTransform {
    fn default() -> Self {
        IDENTITY
    }
}
