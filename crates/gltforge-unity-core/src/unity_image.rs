/// A glTF image entry.
pub struct UnityImage {
    /// The image name. Falls back to the image index as a string if unnamed.
    pub name: String,

    /// The URI of the image, if it references an external file.
    /// `None` for buffer-view-embedded images (e.g. GLB).
    pub uri: Option<String>,

    /// The raw encoded image bytes (PNG, JPEG, …) for buffer-view-embedded images.
    /// `None` when the image is URI-based.
    pub bytes: Option<Vec<u8>>,
}
