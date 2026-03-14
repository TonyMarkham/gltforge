use std::path::Path;

use gltforge_unity::unity_indices::UnityIndices;

const GLTF_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.samples/Triangle/glTF");

#[test]
fn convert_triangle() {
    let gltf_path = Path::new(GLTF_DIR).join("Triangle.gltf");
    let json = std::fs::read_to_string(&gltf_path).unwrap();
    let gltf = gltforge::parser::parse(&json).unwrap();
    let buffers = gltforge::parser::load_buffers(&gltf, Path::new(GLTF_DIR)).unwrap();

    let unity = gltforge_unity::convert::build_unity_gltf(&gltf, &buffers, "Triangle")
        .expect("build_unity_gltf failed");

    // Scene: no name in glTF so falls back to file stem.
    assert_eq!(unity.scene_name, "Triangle");
    assert_eq!(unity.root_game_objects, vec![0u32]);

    // Node 0: unnamed, no children, references mesh 0.
    let node = unity.game_objects.get(&0).expect("node 0 missing");
    assert_eq!(node.name, "0");
    assert!(node.children.is_empty());
    assert_eq!(node.mesh_indices, vec![0u32]);

    // Mesh 0: no name in glTF so falls back to "0", 3 vertices, 1 sub-mesh.
    let mesh = unity.meshes.get(&0).expect("mesh 0 missing");
    assert_eq!(mesh.name, "0");
    assert_eq!(mesh.vertices.len(), 3);
    assert_eq!(mesh.sub_meshes.len(), 1);

    // 3 vertices, positions X-negated for left-handed coordinate system.
    assert_eq!(mesh.vertices[0], [-0.0f32, 0.0, 0.0]);
    assert_eq!(mesh.vertices[1], [-1.0f32, 0.0, 0.0]);
    assert_eq!(mesh.vertices[2], [-0.0f32, 1.0, 0.0]);

    // Sub-mesh 0: glTF [0,1,2] → winding reversed [0,2,1], format u16.
    let UnityIndices::U16(ref indices) = mesh.sub_meshes[0].indices else {
        panic!("expected U16 indices for a 3-vertex mesh");
    };
    assert_eq!(indices.as_slice(), &[0u16, 2, 1]);
}
