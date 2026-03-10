use std::path::Path;

use gltforge::parser;
use gltforge::schema::{AccessorComponentType, AccessorType, BufferViewTarget};

const GLTF_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.samples/Box/glTF");

fn gltf_path(file: &str) -> std::path::PathBuf {
    Path::new(GLTF_DIR).join(file)
}

#[test]
fn parse_box_schema() {
    let json = std::fs::read_to_string(gltf_path("Box.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    assert_eq!(gltf.asset.version, "2.0");
    assert_eq!(gltf.scene, Some(0));

    let scenes = gltf.scenes.as_deref().unwrap();
    assert_eq!(scenes.len(), 1);
    assert_eq!(scenes[0].nodes.as_deref().unwrap(), &[0]);

    let nodes = gltf.nodes.as_deref().unwrap();
    assert_eq!(nodes.len(), 2);

    // Node 0: unnamed, matrix transform, child is node 1.
    assert!(nodes[0].name.is_none());
    assert!(nodes[0].matrix.is_some());
    assert_eq!(nodes[0].children.as_deref().unwrap(), &[1]);

    // Node 1: unnamed, identity transform, references mesh 0.
    assert!(nodes[1].name.is_none());
    assert!(nodes[1].matrix.is_none());
    assert!(nodes[1].translation.is_none());
    assert_eq!(nodes[1].mesh, Some(0));

    let meshes = gltf.meshes.as_deref().unwrap();
    assert_eq!(meshes.len(), 1);
    assert_eq!(meshes[0].name.as_deref(), Some("Mesh"));
    let prim = &meshes[0].primitives[0];
    assert_eq!(prim.indices, Some(0));
    assert_eq!(prim.attributes["POSITION"], 2);
    assert_eq!(prim.attributes["NORMAL"], 1);

    let accessors = gltf.accessors.as_deref().unwrap();
    assert_eq!(accessors.len(), 3);

    // Accessor 0: SCALAR UNSIGNED_SHORT — 36 indices (12 triangles).
    assert_eq!(
        accessors[0].component_type,
        AccessorComponentType::UnsignedShort
    );
    assert_eq!(accessors[0].accessor_type, AccessorType::Scalar);
    assert_eq!(accessors[0].count, 36);

    // Accessor 1: VEC3 FLOAT — 24 normals.
    assert_eq!(accessors[1].accessor_type, AccessorType::Vec3);
    assert_eq!(accessors[1].component_type, AccessorComponentType::Float);
    assert_eq!(accessors[1].count, 24);

    // Accessor 2: VEC3 FLOAT — 24 positions.
    assert_eq!(accessors[2].accessor_type, AccessorType::Vec3);
    assert_eq!(accessors[2].component_type, AccessorComponentType::Float);
    assert_eq!(accessors[2].count, 24);

    let bvs = gltf.buffer_views.as_deref().unwrap();
    assert_eq!(bvs.len(), 2);
    assert_eq!(bvs[0].target, Some(BufferViewTarget::ElementArrayBuffer));
    assert_eq!(bvs[1].target, Some(BufferViewTarget::ArrayBuffer));

    assert_eq!(gltf.buffers.as_deref().unwrap().len(), 1);
}

#[test]
fn resolve_box_positions_and_normals() {
    let json = std::fs::read_to_string(gltf_path("Box.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    let base_dir = Path::new(GLTF_DIR);
    let buffers = parser::load_buffers(&gltf, base_dir).expect("load_buffers failed");

    let bvs = gltf.buffer_views.as_deref().unwrap();
    let accessors = gltf.accessors.as_deref().unwrap();

    // Accessor 2: POSITION — 24 VEC3 FLOAT.
    let pos_bytes =
        parser::resolve_accessor(&accessors[2], bvs, &buffers).expect("resolve accessor 2 failed");
    assert_eq!(pos_bytes.len(), 24 * 12);

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
    assert_eq!(positions.len(), 24);

    // Box vertices lie on a unit cube (±0.5 on each axis).
    for p in &positions {
        for &v in p {
            assert!(
                v.abs() <= 0.5 + 1e-5,
                "position component out of range: {v}"
            );
        }
    }

    // Accessor 1: NORMAL — 24 VEC3 FLOAT, all axis-aligned unit vectors.
    let norm_bytes =
        parser::resolve_accessor(&accessors[1], bvs, &buffers).expect("resolve accessor 1 failed");
    assert_eq!(norm_bytes.len(), 24 * 12);

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
        assert!((len_sq - 1.0).abs() < 1e-5, "normal not unit length: {n:?}");
    }
}
