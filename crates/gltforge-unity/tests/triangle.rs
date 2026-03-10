use std::path::Path;

use gltforge_unity::mesh::UnityIndices;

const GLTF_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../.samples/Triangle/glTF");

#[test]
fn convert_triangle_node() {
    let gltf_path = Path::new(GLTF_DIR).join("Triangle.gltf");
    let json = std::fs::read_to_string(&gltf_path).unwrap();
    let gltf = gltforge::parser::parse(&json).unwrap();
    let buffers = gltforge::parser::load_buffers(&gltf, Path::new(GLTF_DIR)).unwrap();

    // Node 0: unnamed, mesh 0: unnamed, 1 primitive → name "0_0"
    let mesh = gltforge_unity::convert::build_unity_mesh(&gltf, &buffers, 0)
        .expect("build_unity_mesh failed");

    assert_eq!(mesh.name, "0_0");

    // 3 vertices across 1 primitive → u16 indices.
    assert_eq!(mesh.positions.len(), 3);
    assert_eq!(mesh.submeshes.len(), 1);

    // Positions: glTF [(0,0,0),(1,0,0),(0,1,0)] → Unity X-negated.
    assert_eq!(mesh.positions[0], [-0.0f32, 0.0, 0.0]);
    assert_eq!(mesh.positions[1], [-1.0f32, 0.0, 0.0]);
    assert_eq!(mesh.positions[2], [-0.0f32, 1.0, 0.0]);

    // Indices: glTF [0,1,2] → winding reversed [0,2,1], format u16.
    let UnityIndices::U16(ref indices) = mesh.submeshes[0].indices else {
        panic!("expected U16 indices for a 3-vertex mesh");
    };
    assert_eq!(indices.as_slice(), &[0u16, 2, 1]);
}
