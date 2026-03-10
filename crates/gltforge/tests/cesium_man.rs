use std::path::Path;

use gltforge::parser;
use gltforge::schema::{AccessorComponentType, AccessorType, MeshPrimitiveMode};

const GLTF_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.samples/CesiumMan/glTF");

fn gltf_path(file: &str) -> std::path::PathBuf {
    Path::new(GLTF_DIR).join(file)
}

#[test]
fn parse_cesium_man_schema() {
    let json = std::fs::read_to_string(gltf_path("CesiumMan.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    assert_eq!(gltf.asset.version, "2.0");
    assert_eq!(gltf.scene, Some(0));

    // Top-level counts
    let scenes = gltf.scenes.as_deref().unwrap();
    assert_eq!(scenes.len(), 1);

    let nodes = gltf.nodes.as_deref().unwrap();
    assert_eq!(nodes.len(), 22);

    let meshes = gltf.meshes.as_deref().unwrap();
    assert_eq!(meshes.len(), 1);

    let accessors = gltf.accessors.as_deref().unwrap();
    assert_eq!(accessors.len(), 83);

    assert_eq!(gltf.buffers.as_deref().unwrap().len(), 1);
    assert_eq!(gltf.buffer_views.as_deref().unwrap().len(), 8);
    assert_eq!(gltf.animations.as_deref().unwrap().len(), 1);
    assert_eq!(gltf.skins.as_deref().unwrap().len(), 1);

    // Root scene has one root node
    let root_nodes = scenes[0].nodes.as_deref().unwrap();
    assert_eq!(root_nodes, &[0]);

    // Node 0 is the Z_UP matrix node, with one child: Armature
    let n0 = &nodes[0];
    assert_eq!(n0.name.as_deref(), Some("Z_UP"));
    assert!(n0.matrix.is_some());
    assert!(n0.translation.is_none());
    assert_eq!(n0.children.as_deref().unwrap(), &[1]);

    // Node 1 is the Armature, child of 0; its children include the mesh node and the skeleton root
    let n1 = &nodes[1];
    assert_eq!(n1.name.as_deref(), Some("Armature"));
    assert!(n1.matrix.is_some());
    let armature_children = n1.children.as_deref().unwrap();
    assert!(
        armature_children.contains(&2),
        "Armature should have Cesium_Man (2) as child"
    );
    assert!(
        armature_children.contains(&3),
        "Armature should have torso joint (3) as child"
    );

    // Node 2 is the skinned Cesium_Man mesh node
    let n2 = &nodes[2];
    assert_eq!(n2.name.as_deref(), Some("Cesium_Man"));
    assert_eq!(n2.mesh, Some(0));

    // Node 3 is Skeleton_torso_joint_1, a TRS node (child of Armature)
    let n3 = &nodes[3];
    assert_eq!(n3.name.as_deref(), Some("Skeleton_torso_joint_1"));
    assert!(n3.translation.is_some());
    assert!(n3.rotation.is_some());
    assert!(n3.scale.is_some());
    assert!(n3.matrix.is_none());
    let t3 = n3.translation.unwrap();
    assert!((t3[0] - 0.0).abs() < 1e-4);
    assert!((t3[1] - 0.005).abs() < 1e-3);
    assert!((t3[2] - 0.679).abs() < 1e-3);

    // Mesh 0: Cesium_Man — one TRIANGLES primitive
    let mesh = &meshes[0];
    assert_eq!(mesh.name.as_deref(), Some("Cesium_Man"));
    assert_eq!(mesh.primitives.len(), 1);
    let prim = &mesh.primitives[0];
    assert_eq!(prim.mode, MeshPrimitiveMode::Triangles);
    assert_eq!(prim.indices, Some(0));

    // Accessor 0: SCALAR UNSIGNED_SHORT — index buffer, 14016 indices
    let acc0 = &accessors[0];
    assert_eq!(acc0.component_type, AccessorComponentType::UnsignedShort);
    assert_eq!(acc0.accessor_type, AccessorType::Scalar);
    assert_eq!(acc0.count, 14016);

    // Accessor 3: VEC3 FLOAT — POSITION, 3273 vertices
    let acc3 = &accessors[3];
    assert_eq!(prim.attributes["POSITION"], 3);
    assert_eq!(acc3.component_type, AccessorComponentType::Float);
    assert_eq!(acc3.accessor_type, AccessorType::Vec3);
    assert_eq!(acc3.count, 3273);

    // Accessor 2: VEC3 FLOAT — NORMAL
    assert_eq!(prim.attributes["NORMAL"], 2);
    assert_eq!(accessors[2].accessor_type, AccessorType::Vec3);
    assert_eq!(accessors[2].count, 3273);

    // Accessor 4: VEC2 FLOAT — TEXCOORD_0
    assert_eq!(prim.attributes["TEXCOORD_0"], 4);
    assert_eq!(accessors[4].accessor_type, AccessorType::Vec2);

    // Accessor 1: VEC4 UNSIGNED_SHORT — JOINTS_0
    assert_eq!(prim.attributes["JOINTS_0"], 1);
    assert_eq!(accessors[1].accessor_type, AccessorType::Vec4);
    assert_eq!(
        accessors[1].component_type,
        AccessorComponentType::UnsignedShort
    );

    // Accessor 5: VEC4 FLOAT — WEIGHTS_0
    assert_eq!(prim.attributes["WEIGHTS_0"], 5);
    assert_eq!(accessors[5].accessor_type, AccessorType::Vec4);
    assert_eq!(accessors[5].component_type, AccessorComponentType::Float);
}

#[test]
fn resolve_cesium_man_positions() {
    let json = std::fs::read_to_string(gltf_path("CesiumMan.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    let base_dir = Path::new(GLTF_DIR);
    let buffers = parser::load_buffers(&gltf, base_dir).expect("load_buffers failed");

    let bvs = gltf.buffer_views.as_deref().unwrap();
    let accessors = gltf.accessors.as_deref().unwrap();

    // Accessor 3: POSITION — 3273 VEC3 FLOAT
    let bytes =
        parser::resolve_accessor(&accessors[3], bvs, &buffers).expect("resolve accessor 3 failed");
    assert_eq!(bytes.len(), 3273 * 3 * 4);

    let positions: Vec<[f32; 3]> = bytes
        .chunks_exact(12)
        .map(|c| {
            [
                f32::from_le_bytes(c[0..4].try_into().unwrap()),
                f32::from_le_bytes(c[4..8].try_into().unwrap()),
                f32::from_le_bytes(c[8..12].try_into().unwrap()),
            ]
        })
        .collect();
    assert_eq!(positions.len(), 3273);

    // Sanity-check: all positions within plausible human-figure bounds
    for (i, p) in positions.iter().enumerate() {
        for &v in p {
            assert!(v.is_finite(), "position {i} has non-finite component");
        }
        assert!(p[0].abs() < 2.0, "position {i} X out of range: {}", p[0]);
        assert!(p[1].abs() < 2.0, "position {i} Y out of range: {}", p[1]);
        assert!(p[2].abs() < 3.0, "position {i} Z out of range: {}", p[2]);
    }
}

#[test]
fn resolve_cesium_man_node3_trs() {
    // Verify node 3 (Skeleton_torso_joint_1) TRS values parsed correctly
    let json = std::fs::read_to_string(gltf_path("CesiumMan.gltf")).unwrap();
    let gltf = parser::parse(&json).expect("parse failed");

    let nodes = gltf.nodes.as_deref().unwrap();
    let n = &nodes[3];

    let t = n.translation.expect("node 3 has no translation");
    let r = n.rotation.expect("node 3 has no rotation");
    let s = n.scale.expect("node 3 has no scale");

    assert!((t[0] - 0.0).abs() < 1e-4, "T.x");
    assert!((t[1] - 0.005).abs() < 1e-3, "T.y");
    assert!((t[2] - 0.679).abs() < 1e-3, "T.z");

    // rotation (xyzw): [0, -0.0378, 0, -0.9993]
    assert!((r[0] - 0.0).abs() < 1e-4, "R.x");
    assert!((r[1] - (-0.0378)).abs() < 1e-3, "R.y");
    assert!((r[2] - 0.0).abs() < 1e-4, "R.z");
    assert!((r[3] - (-0.9993)).abs() < 1e-3, "R.w");

    // scale is uniform 1
    assert!((s[0] - 1.0).abs() < 1e-6, "S.x");
    assert!((s[1] - 1.0).abs() < 1e-6, "S.y");
    assert!((s[2] - 1.0).abs() < 1e-6, "S.z");
}
