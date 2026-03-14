use std::path::Path;

use gltforge_unity::unity_indices::UnityIndices;

const GLTF_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.samples/CesiumMan/glTF");

fn approx(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

fn load() -> gltforge_unity::unity_gltf::UnityGltf {
    let json = std::fs::read_to_string(Path::new(GLTF_DIR).join("CesiumMan.gltf")).unwrap();
    let gltf = gltforge::parser::parse(&json).unwrap();
    let buffers = gltforge::parser::load_buffers(&gltf, Path::new(GLTF_DIR)).unwrap();
    gltforge_unity::convert::build_unity_gltf(&gltf, &buffers, "CesiumMan")
        .expect("build_unity_gltf failed")
}

#[test]
fn convert_cesium_man_structure() {
    let unity = load();

    // No scene name in glTF → falls back to file stem.
    assert_eq!(unity.scene_name, "CesiumMan");
    assert_eq!(unity.root_game_objects, vec![0u32]);
    assert_eq!(unity.game_objects.len(), 22);
    assert_eq!(unity.meshes.len(), 1);

    // Node 0: Z_UP matrix node, one child (Armature).
    let n0 = unity.game_objects.get(&0).expect("node 0 missing");
    assert_eq!(n0.name, "Z_UP");
    assert_eq!(n0.children, vec![1u32]);
    assert!(n0.mesh_indices.is_empty());

    // Node 1: Armature, children include both the mesh node (2) and the skeleton root (3).
    let n1 = unity.game_objects.get(&1).expect("node 1 missing");
    assert_eq!(n1.name, "Armature");
    assert!(
        n1.children.contains(&2),
        "Armature missing Cesium_Man child"
    );
    assert!(
        n1.children.contains(&3),
        "Armature missing torso joint child"
    );

    // Node 2: skinned mesh node, references mesh 0.
    let n2 = unity.game_objects.get(&2).expect("node 2 missing");
    assert_eq!(n2.name, "Cesium_Man");
    assert_eq!(n2.mesh_indices, vec![0u32]);

    // Node 3: Skeleton_torso_joint_1 — TRS node (no matrix in glTF).
    let n3 = unity.game_objects.get(&3).expect("node 3 missing");
    assert_eq!(n3.name, "Skeleton_torso_joint_1");
}

#[test]
fn convert_cesium_man_node3_trs() {
    let unity = load();
    let n3 = unity.game_objects.get(&3).expect("node 3 missing");
    let t = &n3.transform;

    // glTF T=[0, 0.005, 0.679] → Unity position=[-gltf_x, y, z]=[0, 0.005, 0.679]
    assert!(approx(t.position[0], 0.0), "pos.x: {}", t.position[0]);
    assert!(approx(t.position[1], 0.005), "pos.y: {}", t.position[1]);
    assert!(approx(t.position[2], 0.679), "pos.z: {}", t.position[2]);

    // glTF R=[0, -0.0378, 0, -0.9993] → Unity=[-qx, qy, qz, -qw]=[0, -0.0378, 0, 0.9993]
    assert!(approx(t.rotation[0], 0.0), "rot.x: {}", t.rotation[0]);
    assert!(approx(t.rotation[1], -0.0378), "rot.y: {}", t.rotation[1]);
    assert!(approx(t.rotation[2], 0.0), "rot.z: {}", t.rotation[2]);
    assert!(approx(t.rotation[3], 0.9993), "rot.w: {}", t.rotation[3]);

    // Scale is identity [1, 1, 1].
    assert!(approx(t.scale[0], 1.0), "scale.x: {}", t.scale[0]);
    assert!(approx(t.scale[1], 1.0), "scale.y: {}", t.scale[1]);
    assert!(approx(t.scale[2], 1.0), "scale.z: {}", t.scale[2]);
}

#[test]
fn convert_cesium_man_z_up_matrix() {
    let unity = load();
    let n0 = unity.game_objects.get(&0).expect("node 0 missing");
    let t = &n0.transform;

    // Z_UP matrix is a pure rotation (no translation, scale=1).
    assert!(approx(t.position[0], 0.0));
    assert!(approx(t.position[1], 0.0));
    assert!(approx(t.position[2], 0.0));
    assert!(approx(t.scale[0], 1.0));
    assert!(approx(t.scale[1], 1.0));
    assert!(approx(t.scale[2], 1.0));

    // glTF matrix Rx(90°) → glTF quat [-√2/2, 0, 0, √2/2]
    // Unity conversion (-qx, qy, qz, -qw): [√2/2, 0, 0, -√2/2]
    let sqrt2_2 = std::f32::consts::FRAC_1_SQRT_2;
    assert!(approx(t.rotation[0], sqrt2_2), "rot.x: {}", t.rotation[0]);
    assert!(approx(t.rotation[1], 0.0), "rot.y: {}", t.rotation[1]);
    assert!(approx(t.rotation[2], 0.0), "rot.z: {}", t.rotation[2]);
    assert!(approx(t.rotation[3], -sqrt2_2), "rot.w: {}", t.rotation[3]);
}

#[test]
fn convert_cesium_man_mesh() {
    let unity = load();
    let mesh = unity.meshes.get(&0).expect("mesh 0 missing");

    assert_eq!(mesh.name, "Cesium_Man");
    assert_eq!(mesh.vertices.len(), 3273);
    assert_eq!(mesh.sub_meshes.len(), 1);

    // 14016 indices, u16 (3273 vertices < 65535).
    let UnityIndices::U16(ref indices) = mesh.sub_meshes[0].indices else {
        panic!("expected U16 indices for a 3273-vertex mesh");
    };
    assert_eq!(indices.len(), 14016);

    // Normals present, one per vertex.
    assert_eq!(mesh.normals.len(), 3273);

    // CesiumMan has no TANGENT attribute.
    assert!(mesh.tangents.is_empty());

    // One UV channel (TEXCOORD_0).
    assert_eq!(mesh.uvs.len(), 1);
    assert_eq!(mesh.uvs[0].len(), 3273);

    // All normals are unit vectors.
    for n in &mesh.normals {
        let len_sq = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
        assert!((len_sq - 1.0).abs() < 1e-4, "normal not unit length: {n:?}");
    }

    // All vertex positions are finite and within plausible human-figure bounds (X already negated).
    for (i, v) in mesh.vertices.iter().enumerate() {
        for &c in v {
            assert!(c.is_finite(), "vertex {i} has non-finite component");
        }
        assert!(v[0].abs() < 2.0, "vertex {i} X out of range: {}", v[0]);
        assert!(v[1].abs() < 2.0, "vertex {i} Y out of range: {}", v[1]);
        assert!(v[2].abs() < 3.0, "vertex {i} Z out of range: {}", v[2]);
    }
}
