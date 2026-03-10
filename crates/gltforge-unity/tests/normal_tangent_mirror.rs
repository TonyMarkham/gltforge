use std::path::Path;

use gltforge_unity::unity_indices::UnityIndices;

const GLTF_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../.samples/NormalTangentMirrorTest/glTF"
);

#[test]
fn convert_normal_tangent_mirror() {
    let gltf_path = Path::new(GLTF_DIR).join("NormalTangentMirrorTest.gltf");
    let json = std::fs::read_to_string(&gltf_path).unwrap();
    let gltf = gltforge::parser::parse(&json).unwrap();
    let buffers = gltforge::parser::load_buffers(&gltf, Path::new(GLTF_DIR)).unwrap();

    let unity =
        gltforge_unity::convert::build_unity_gltf(&gltf, &buffers, "NormalTangentMirrorTest")
            .expect("build_unity_gltf failed");

    // One mesh with one primitive.
    let mesh = unity.meshes.get(&0).expect("mesh 0 missing");
    assert_eq!(mesh.name, "NormalTangentTest_low");
    assert_eq!(mesh.vertices.len(), 2770);
    assert_eq!(mesh.sub_meshes.len(), 1);

    // 15720 indices (5240 triangles), u16 format for 2770 vertices.
    let UnityIndices::U16(ref indices) = mesh.sub_meshes[0].indices else {
        panic!("expected U16 indices for a 2770-vertex mesh");
    };
    assert_eq!(indices.len(), 15720);

    // Normals and tangents must be present, same length as vertices.
    assert_eq!(mesh.normals.len(), 2770);
    assert_eq!(mesh.tangents.len(), 2770);

    // All normals are unit vectors.
    for n in &mesh.normals {
        let len_sq = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
        assert!((len_sq - 1.0).abs() < 1e-4, "normal not unit length: {n:?}");
    }

    // All tangent XYZ components are unit vectors; W is ±1.
    for t in &mesh.tangents {
        let len_sq = t[0] * t[0] + t[1] * t[1] + t[2] * t[2];
        assert!(
            (len_sq - 1.0).abs() < 1e-4,
            "tangent not unit length: {t:?}"
        );
        assert!(
            (t[3].abs() - 1.0).abs() < 1e-4,
            "tangent w not ±1: {}",
            t[3]
        );
    }

    // UV channel 0 must be present, same length as vertices.
    assert_eq!(mesh.uvs.len(), 1, "expected 1 UV channel");
    assert_eq!(mesh.uvs[0].len(), 2770);

    // All UVs must be in [0, 1] range (this model uses a single atlas).
    for uv in &mesh.uvs[0] {
        assert!(uv[0] >= 0.0 && uv[0] <= 1.0, "UV u out of range: {}", uv[0]);
        assert!(uv[1] >= 0.0 && uv[1] <= 1.0, "UV v out of range: {}", uv[1]);
    }
}
