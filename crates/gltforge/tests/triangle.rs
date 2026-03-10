use std::path::Path;

use gltforge::parser;
use gltforge::schema::{AccessorComponentType, AccessorType, BufferViewTarget};

const GLTF_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.samples/Triangle/glTF");

fn gltf_path(file: &str) -> std::path::PathBuf {
    Path::new(GLTF_DIR).join(file)
}

#[test]
fn parse_triangle_schema() {
    let json = std::fs::read_to_string(gltf_path("Triangle.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    assert_eq!(gltf.asset.version, "2.0");
    assert_eq!(gltf.scene, Some(0));

    let scenes = gltf.scenes.as_deref().unwrap();
    assert_eq!(scenes.len(), 1);
    assert_eq!(scenes[0].nodes.as_deref().unwrap(), &[0]);

    let nodes = gltf.nodes.as_deref().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].mesh, Some(0));

    let meshes = gltf.meshes.as_deref().unwrap();
    assert_eq!(meshes.len(), 1);
    let prim = &meshes[0].primitives[0];
    assert_eq!(prim.indices, Some(0));
    assert_eq!(prim.attributes["POSITION"], 1);

    let buffers = gltf.buffers.as_deref().unwrap();
    assert_eq!(buffers.len(), 1);
    assert_eq!(buffers[0].byte_length, 44);

    let bvs = gltf.buffer_views.as_deref().unwrap();
    assert_eq!(bvs.len(), 2);
    assert_eq!(bvs[0].target, Some(BufferViewTarget::ElementArrayBuffer));
    assert_eq!(bvs[1].target, Some(BufferViewTarget::ArrayBuffer));

    let accessors = gltf.accessors.as_deref().unwrap();
    assert_eq!(accessors.len(), 2);
    assert_eq!(
        accessors[0].component_type,
        AccessorComponentType::UnsignedShort
    );
    assert_eq!(accessors[0].accessor_type, AccessorType::Scalar);
    assert_eq!(accessors[0].count, 3);
    assert_eq!(accessors[1].component_type, AccessorComponentType::Float);
    assert_eq!(accessors[1].accessor_type, AccessorType::Vec3);
    assert_eq!(accessors[1].count, 3);
}

#[test]
fn resolve_triangle_accessors() {
    let json = std::fs::read_to_string(gltf_path("Triangle.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    let base_dir = Path::new(GLTF_DIR);
    let buffers = parser::load_buffers(&gltf, base_dir).expect("load_buffers failed");
    assert_eq!(buffers[0].len(), 44);

    let bvs = gltf.buffer_views.as_deref().unwrap();
    let accessors = gltf.accessors.as_deref().unwrap();

    // Accessor 0: SCALAR UNSIGNED_SHORT indices [0, 1, 2]
    let index_bytes =
        parser::resolve_accessor(&accessors[0], bvs, &buffers).expect("resolve accessor 0 failed");
    assert_eq!(index_bytes.len(), 6);
    let indices: Vec<u16> = index_bytes
        .chunks_exact(2)
        .map(|c| u16::from_le_bytes([c[0], c[1]]))
        .collect();
    assert_eq!(indices, vec![0u16, 1, 2]);

    // Accessor 1: VEC3 FLOAT positions [(0,0,0), (1,0,0), (0,1,0)]
    let pos_bytes =
        parser::resolve_accessor(&accessors[1], bvs, &buffers).expect("resolve accessor 1 failed");
    assert_eq!(pos_bytes.len(), 36);
    let positions: Vec<f32> = pos_bytes
        .chunks_exact(4)
        .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    assert_eq!(
        positions,
        vec![0.0f32, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]
    );
}
