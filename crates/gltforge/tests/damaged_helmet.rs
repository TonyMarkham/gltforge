use gltforge::parser;
use gltforge::schema::{AccessorComponentType, AccessorType};

const GLB_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../.samples/DamagedHelmet/glTF-Binary/DamagedHelmet.glb"
);

#[test]
fn parse_damaged_helmet_schema() {
    let data = std::fs::read(GLB_PATH).unwrap();
    let (gltf, _buffers) = parser::parse_glb(&data).expect("parse_glb failed");

    assert_eq!(gltf.asset.version, "2.0");
    assert_eq!(gltf.scene, Some(0));

    // Scene
    let scenes = gltf.scenes.as_deref().unwrap();
    assert_eq!(scenes.len(), 1);
    assert_eq!(scenes[0].name.as_deref(), Some("Scene"));
    assert_eq!(scenes[0].nodes.as_deref().unwrap(), &[0]);

    // Nodes
    let nodes = gltf.nodes.as_deref().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].name.as_deref(), Some("node_damagedHelmet_-6514"));
    assert_eq!(nodes[0].mesh, Some(0));

    // Mesh
    let meshes = gltf.meshes.as_deref().unwrap();
    assert_eq!(meshes.len(), 1);
    assert_eq!(
        meshes[0].name.as_deref(),
        Some("mesh_helmet_LP_13930damagedHelmet")
    );
    let prim = &meshes[0].primitives[0];
    assert_eq!(prim.indices, Some(0));
    assert_eq!(prim.attributes["POSITION"], 1);
    assert_eq!(prim.attributes["NORMAL"], 2);
    assert_eq!(prim.attributes["TEXCOORD_0"], 3);
    assert_eq!(prim.material, Some(0));

    // Accessors
    let accessors = gltf.accessors.as_deref().unwrap();
    assert_eq!(accessors.len(), 4);

    // Accessor 0: SCALAR UNSIGNED_SHORT — 46356 indices.
    assert_eq!(
        accessors[0].component_type,
        AccessorComponentType::UnsignedShort
    );
    assert_eq!(accessors[0].accessor_type, AccessorType::Scalar);
    assert_eq!(accessors[0].count, 46356);

    // Accessor 1: VEC3 FLOAT — 14556 positions.
    assert_eq!(accessors[1].component_type, AccessorComponentType::Float);
    assert_eq!(accessors[1].accessor_type, AccessorType::Vec3);
    assert_eq!(accessors[1].count, 14556);

    // Accessor 2: VEC3 FLOAT — 14556 normals.
    assert_eq!(accessors[2].component_type, AccessorComponentType::Float);
    assert_eq!(accessors[2].accessor_type, AccessorType::Vec3);
    assert_eq!(accessors[2].count, 14556);

    // Accessor 3: VEC2 FLOAT — 14556 UVs.
    assert_eq!(accessors[3].component_type, AccessorComponentType::Float);
    assert_eq!(accessors[3].accessor_type, AccessorType::Vec2);
    assert_eq!(accessors[3].count, 14556);

    // Material
    let materials = gltf.materials.as_deref().unwrap();
    assert_eq!(materials.len(), 1);
    assert_eq!(materials[0].name.as_deref(), Some("Material_MR"));

    // Images
    let images = gltf.images.as_deref().unwrap();
    assert_eq!(images.len(), 5);

    // Textures
    let textures = gltf.textures.as_deref().unwrap();
    assert_eq!(textures.len(), 5);
}

#[test]
fn resolve_damaged_helmet_positions_and_normals() {
    let data = std::fs::read(GLB_PATH).unwrap();
    let (gltf, buffers) = parser::parse_glb(&data).expect("parse_glb failed");

    // The GLB binary chunk (geometry + embedded textures) becomes buffers[0].
    assert_eq!(buffers.len(), 1);
    assert!(
        buffers[0].len() >= 558504,
        "buffer too small: {}",
        buffers[0].len()
    );

    let bvs = gltf.buffer_views.as_deref().unwrap();
    let accessors = gltf.accessors.as_deref().unwrap();

    // Accessor 1: POSITION — 14556 VEC3 FLOAT.
    let pos_bytes =
        parser::resolve_accessor(&accessors[1], bvs, &buffers).expect("resolve accessor 1 failed");
    assert_eq!(pos_bytes.len(), 14556 * 12);

    let positions: Vec<[f32; 3]> = pos_bytes
        .chunks_exact(12)
        .map(|c| {
            [
                f32::from_le_bytes(c[0..4].try_into().unwrap()),
                f32::from_le_bytes(c[4..8].try_into().unwrap()),
                f32::from_le_bytes(c[8..12].try_into().unwrap()),
            ]
        })
        .collect();

    // Positions lie within the glTF-supplied min/max bounds.
    for p in &positions {
        assert!(
            p[0] >= -0.9475 - 1e-4 && p[0] <= 0.9425 + 1e-4,
            "X out of bounds: {}",
            p[0]
        );
        assert!(
            p[1] >= -1.1872 - 1e-4 && p[1] <= 0.8129 + 1e-4,
            "Y out of bounds: {}",
            p[1]
        );
        assert!(
            p[2] >= -0.9010 - 1e-4 && p[2] <= 0.9010 + 1e-4,
            "Z out of bounds: {}",
            p[2]
        );
    }

    // Accessor 2: NORMAL — 14556 VEC3 FLOAT, all unit-length.
    let norm_bytes =
        parser::resolve_accessor(&accessors[2], bvs, &buffers).expect("resolve accessor 2 failed");
    assert_eq!(norm_bytes.len(), 14556 * 12);

    let normals: Vec<[f32; 3]> = norm_bytes
        .chunks_exact(12)
        .map(|c| {
            [
                f32::from_le_bytes(c[0..4].try_into().unwrap()),
                f32::from_le_bytes(c[4..8].try_into().unwrap()),
                f32::from_le_bytes(c[8..12].try_into().unwrap()),
            ]
        })
        .collect();

    for n in &normals {
        let len_sq = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
        assert!((len_sq - 1.0).abs() < 1e-4, "normal not unit length: {n:?}");
    }
}
