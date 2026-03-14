use std::path::Path;

use gltforge_unity::unity_indices::UnityIndices;

const GLTF_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.samples/Box/glTF");

#[test]
fn convert_box() {
    let gltf_path = Path::new(GLTF_DIR).join("Box.gltf");
    let json = std::fs::read_to_string(&gltf_path).unwrap();
    let gltf = gltforge::parser::parse(&json).unwrap();
    let buffers = gltforge::parser::load_buffers(&gltf, Path::new(GLTF_DIR)).unwrap();

    let unity = gltforge_unity::convert::build_unity_gltf(&gltf, &buffers, "Box")
        .expect("build_unity_gltf failed");

    // Scene: no name in glTF so falls back to file stem.
    assert_eq!(unity.scene_name, "Box");
    assert_eq!(unity.root_game_objects, vec![0u32]);

    // Node 0: unnamed, one child (node 1), no mesh.
    let node0 = unity.game_objects.get(&0).expect("node 0 missing");
    assert_eq!(node0.name, "0");
    assert_eq!(node0.children, vec![1u32]);
    assert!(node0.mesh_indices.is_empty());

    // Node 1: unnamed, no children, references mesh 0.
    let node1 = unity.game_objects.get(&1).expect("node 1 missing");
    assert_eq!(node1.name, "1");
    assert!(node1.children.is_empty());
    assert_eq!(node1.mesh_indices, vec![0u32]);

    // Mesh 0: named "Mesh", 24 vertices, 24 normals, 1 sub-mesh.
    let mesh = unity.meshes.get(&0).expect("mesh 0 missing");
    assert_eq!(mesh.name, "Mesh");
    assert_eq!(mesh.vertices.len(), 24);
    assert_eq!(mesh.normals.len(), 24);
    assert_eq!(mesh.sub_meshes.len(), 1);

    // All normals are unit vectors (length ≈ 1.0) and axis-aligned for a box.
    for n in &mesh.normals {
        let len_sq = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
        assert!((len_sq - 1.0).abs() < 1e-5, "normal not unit length: {n:?}");
    }

    // Sub-mesh 0: 36 indices (12 triangles), u16 format for 24 vertices.
    let UnityIndices::U16(ref indices) = mesh.sub_meshes[0].indices else {
        panic!("expected U16 indices for a 24-vertex mesh");
    };
    assert_eq!(indices.len(), 36);
}
