use std::path::Path;

use gltforge_unity::unity_pbr_metallic_roughness::ALPHA_MODE_OPAQUE;

const GLTF_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../.samples/ABeautifulGame/glTF"
);

fn load() -> gltforge_unity::unity_gltf::UnityGltf {
    let json = std::fs::read_to_string(Path::new(GLTF_DIR).join("ABeautifulGame.gltf")).unwrap();
    let gltf = gltforge::parser::parse(&json).unwrap();
    let buffers = gltforge::parser::load_buffers(&gltf, Path::new(GLTF_DIR)).unwrap();
    gltforge_unity::convert::build_unity_gltf(&gltf, &buffers, "ABeautifulGame")
        .expect("build_unity_gltf failed")
}

#[test]
fn images_loaded() {
    let unity = load();

    // ABeautifulGame has 33 JPEG images.
    assert_eq!(unity.images.len(), 33, "expected 33 images");

    // Every image must have a URI (no embedded images in this file).
    for (idx, img) in &unity.images {
        assert!(img.uri.is_some(), "image {idx} ({}) has no URI", img.name);
        let uri = img.uri.as_ref().unwrap();
        assert!(
            uri.ends_with(".jpg"),
            "image {idx} URI is not a .jpg: {uri}"
        );
    }
}

#[test]
fn materials_loaded() {
    let unity = load();

    // King_Black is material 0: normalTexture=0, occlusionTexture=1, baseColorTexture=2,
    // metallicRoughnessTexture=1. All resolve to image indices.
    let king_black = unity
        .pbr_metallic_roughness
        .get(&0)
        .expect("material 0 missing");

    assert_eq!(king_black.name, "King_Black");

    // normal → texture 0 → image 0 (King_black_normal.jpg)
    assert_eq!(
        king_black.normal_texture,
        Some(0),
        "King_Black normal_texture"
    );

    // occlusion → texture 1 → image 1 (King_black_ORM.jpg)
    assert_eq!(
        king_black.occlusion_texture,
        Some(1),
        "King_Black occlusion_texture"
    );

    // baseColor → texture 2 → image 2 (King_black_base_color.jpg)
    assert_eq!(
        king_black.base_color_texture,
        Some(2),
        "King_Black base_color_texture"
    );

    // metallicRoughness → texture 1 → image 1 (same ORM as occlusion)
    assert_eq!(
        king_black.metallic_roughness_texture,
        Some(1),
        "King_Black metallic_roughness_texture"
    );

    // Defaults: no emissive texture, OPAQUE, single-sided.
    assert_eq!(king_black.emissive_texture, None);
    assert_eq!(king_black.alpha_mode, ALPHA_MODE_OPAQUE);
    assert!(!king_black.double_sided);
    assert_eq!(king_black.base_color_factor, [1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn submesh_material_links() {
    let unity = load();

    // Every sub-mesh in this file should have a material assigned.
    for (mesh_idx, mesh) in &unity.meshes {
        for (sub_idx, sub) in mesh.sub_meshes.iter().enumerate() {
            assert!(
                sub.material_index.is_some(),
                "mesh {mesh_idx} sub-mesh {sub_idx} has no material"
            );
            // Material index must be in range.
            let mat_idx = sub.material_index.unwrap();
            assert!(
                unity.pbr_metallic_roughness.contains_key(&mat_idx),
                "mesh {mesh_idx} sub-mesh {sub_idx} references out-of-range material {mat_idx}"
            );
        }
    }
}

#[test]
fn image_names_match_uris() {
    let unity = load();

    // Image names and URIs should correspond (name = base name without extension).
    let king_normal = unity.images.get(&0).expect("image 0 missing");
    assert_eq!(king_normal.name, "King_black_normal");
    assert_eq!(king_normal.uri.as_deref(), Some("King_black_normal.jpg"));
}
