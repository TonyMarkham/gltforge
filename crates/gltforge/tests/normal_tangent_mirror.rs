use std::path::Path;

use gltforge::parser;
use gltforge::schema::{AccessorComponentType, AccessorType};

const GLTF_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../.samples/NormalTangentMirrorTest/glTF"
);

fn gltf_path(file: &str) -> std::path::PathBuf {
    Path::new(GLTF_DIR).join(file)
}

#[test]
fn parse_normal_tangent_mirror_schema() {
    let json = std::fs::read_to_string(gltf_path("NormalTangentMirrorTest.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    assert_eq!(gltf.asset.version, "2.0");
    assert_eq!(gltf.scene, Some(0));

    let nodes = gltf.nodes.as_deref().unwrap();
    assert_eq!(nodes.len(), 1);
    assert_eq!(nodes[0].mesh, Some(0));

    let meshes = gltf.meshes.as_deref().unwrap();
    assert_eq!(meshes.len(), 1);
    assert_eq!(meshes[0].name.as_deref(), Some("NormalTangentTest_low"));
    let prim = &meshes[0].primitives[0];
    assert_eq!(prim.indices, Some(0));
    assert_eq!(prim.attributes["POSITION"], 1);
    assert_eq!(prim.attributes["NORMAL"], 2);
    assert_eq!(prim.attributes["TANGENT"], 3);
    assert_eq!(prim.attributes["TEXCOORD_0"], 4);

    let accessors = gltf.accessors.as_deref().unwrap();
    assert_eq!(accessors.len(), 5);

    // Accessor 0: SCALAR UNSIGNED_SHORT — 15720 indices (5240 triangles).
    assert_eq!(
        accessors[0].component_type,
        AccessorComponentType::UnsignedShort
    );
    assert_eq!(accessors[0].accessor_type, AccessorType::Scalar);
    assert_eq!(accessors[0].count, 15720);

    // Accessor 1: VEC3 FLOAT — 2770 positions.
    assert_eq!(accessors[1].accessor_type, AccessorType::Vec3);
    assert_eq!(accessors[1].component_type, AccessorComponentType::Float);
    assert_eq!(accessors[1].count, 2770);

    // Accessor 2: VEC3 FLOAT — 2770 normals.
    assert_eq!(accessors[2].accessor_type, AccessorType::Vec3);
    assert_eq!(accessors[2].count, 2770);

    // Accessor 3: VEC4 FLOAT — 2770 tangents.
    assert_eq!(accessors[3].accessor_type, AccessorType::Vec4);
    assert_eq!(accessors[3].component_type, AccessorComponentType::Float);
    assert_eq!(accessors[3].count, 2770);

    // Accessor 4: VEC2 FLOAT — 2770 UVs.
    assert_eq!(accessors[4].accessor_type, AccessorType::Vec2);
    assert_eq!(accessors[4].count, 2770);

    assert_eq!(gltf.buffer_views.as_deref().unwrap().len(), 5);
    assert_eq!(gltf.buffers.as_deref().unwrap().len(), 1);
    assert_eq!(gltf.textures.as_deref().unwrap().len(), 3);
}

#[test]
fn resolve_normal_tangent_mirror_attributes() {
    let json = std::fs::read_to_string(gltf_path("NormalTangentMirrorTest.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    let base_dir = Path::new(GLTF_DIR);
    let buffers = parser::load_buffers(&gltf, base_dir).expect("load_buffers failed");

    let bvs = gltf.buffer_views.as_deref().unwrap();
    let accessors = gltf.accessors.as_deref().unwrap();

    // Accessor 1: POSITION — 2770 VEC3 FLOAT.
    let pos_bytes =
        parser::resolve_accessor(&accessors[1], bvs, &buffers).expect("resolve positions failed");
    assert_eq!(pos_bytes.len(), 2770 * 12);

    // Accessor 2: NORMAL — 2770 VEC3 FLOAT, all unit vectors.
    let norm_bytes =
        parser::resolve_accessor(&accessors[2], bvs, &buffers).expect("resolve normals failed");
    assert_eq!(norm_bytes.len(), 2770 * 12);

    for n in norm_bytes.chunks_exact(12) {
        let x = f32::from_le_bytes(n[0..4].try_into().unwrap());
        let y = f32::from_le_bytes(n[4..8].try_into().unwrap());
        let z = f32::from_le_bytes(n[8..12].try_into().unwrap());
        let len_sq = x * x + y * y + z * z;
        assert!(
            (len_sq - 1.0).abs() < 1e-4,
            "normal not unit length: [{x},{y},{z}]"
        );
    }

    // Accessor 3: TANGENT — 2770 VEC4 FLOAT, XYZ unit, W = ±1.
    let tang_bytes =
        parser::resolve_accessor(&accessors[3], bvs, &buffers).expect("resolve tangents failed");
    assert_eq!(tang_bytes.len(), 2770 * 16);

    for t in tang_bytes.chunks_exact(16) {
        let x = f32::from_le_bytes(t[0..4].try_into().unwrap());
        let y = f32::from_le_bytes(t[4..8].try_into().unwrap());
        let z = f32::from_le_bytes(t[8..12].try_into().unwrap());
        let w = f32::from_le_bytes(t[12..16].try_into().unwrap());
        let len_sq = x * x + y * y + z * z;
        assert!(
            (len_sq - 1.0).abs() < 1e-4,
            "tangent XYZ not unit length: [{x},{y},{z}]"
        );
        assert!((w.abs() - 1.0).abs() < 1e-4, "tangent W not ±1: {w}");
    }

    // Accessor 4: TEXCOORD_0 — 2770 VEC2 FLOAT, in [0, 1].
    let uv_bytes =
        parser::resolve_accessor(&accessors[4], bvs, &buffers).expect("resolve UVs failed");
    assert_eq!(uv_bytes.len(), 2770 * 8);

    for uv in uv_bytes.chunks_exact(8) {
        let u = f32::from_le_bytes(uv[0..4].try_into().unwrap());
        let v = f32::from_le_bytes(uv[4..8].try_into().unwrap());
        assert!(u >= 0.0 && u <= 1.0, "UV u out of range: {u}");
        assert!(v >= 0.0 && v <= 1.0, "UV v out of range: {v}");
    }
}
