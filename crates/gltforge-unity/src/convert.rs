use error_location::ErrorLocation;
use gltforge::{
    parser::resolve_accessor,
    schema::{AccessorComponentType, AccessorType, Gltf, MeshPrimitive, MeshPrimitiveMode},
};
use std::{collections::HashMap, panic::Location};

use crate::{
    error::{ConvertError, ConvertResult},
    gltf_primitive_data::GltfPrimitiveData,
    unity_gltf::UnityGltf,
    unity_indices::UnityIndices,
    unity_mesh::UnityMesh,
    unity_node::UnityNode,
    unity_node_transform::UnityNodeTransform,
    unity_submesh::UnitySubMesh,
};

/// Build a [`UnityGltf`] from a parsed glTF document and its loaded buffers.
///
/// Converts all nodes and meshes into Unity's left-handed coordinate system.
/// Each glTF mesh becomes one [`UnityMesh`] with a merged vertex array and one
/// sub-mesh per glTF primitive, with indices pre-offset into the shared array.
#[track_caller]
pub fn build_unity_gltf(
    gltf: &Gltf,
    buffers: &[Vec<u8>],
    file_stem: &str,
) -> ConvertResult<UnityGltf> {
    // ---- scene --------------------------------------------------------------

    let scene_idx = gltf.scene.unwrap_or(0) as usize;
    let scene = gltf.scenes.as_deref().and_then(|s| s.get(scene_idx));

    let scene_name = scene
        .and_then(|s| s.name.clone())
        .unwrap_or_else(|| file_stem.to_string());
    let root_nodes = scene
        .and_then(|s| s.nodes.as_deref())
        .unwrap_or(&[])
        .to_vec();

    // ---- nodes --------------------------------------------------------------

    let mut nodes: HashMap<u32, UnityNode> = HashMap::new();

    for (idx, node) in gltf.nodes.as_deref().unwrap_or(&[]).iter().enumerate() {
        let children = node.children.as_deref().unwrap_or(&[]).to_vec();
        let mesh_indices = node.mesh.map(|m| vec![m]).unwrap_or_default();

        nodes.insert(
            idx as u32,
            UnityNode {
                name: node.name.clone().unwrap_or_else(|| idx.to_string()),
                children,
                mesh_indices,
                transform: node_transform(node),
            },
        );
    }

    // ---- meshes -------------------------------------------------------------

    let bvs = gltf.buffer_views.as_deref().unwrap_or(&[]);
    let accessors = gltf.accessors.as_deref().unwrap_or(&[]);
    let mut meshes: HashMap<u32, UnityMesh> = HashMap::new();

    for (mesh_idx, mesh) in gltf.meshes.as_deref().unwrap_or(&[]).iter().enumerate() {
        // --- First pass: resolve each primitive's positions and wound indices ---

        let mut prims: Vec<GltfPrimitiveData> = Vec::new();

        for prim in &mesh.primitives {
            prims.push(resolve_primitive(prim, accessors, bvs, buffers)?);
        }

        // --- Second pass: merge vertices, offset indices, pick format --------

        let total_verts: usize = prims.iter().map(|p| p.positions.len()).sum();
        let use_u32 = total_verts > 65535;
        let all_have_normals = prims.iter().all(|p| p.normals.is_some());
        let all_have_tangents = prims.iter().all(|p| p.tangents.is_some());

        // Determine how many UV channels are shared by every primitive (stop at first gap).
        let max_uv_channels = prims.iter().map(|p| p.uvs.len()).min().unwrap_or(0);
        let uv_channel_count = (0..max_uv_channels)
            .take_while(|&ch| prims.iter().all(|p| p.uvs[ch].is_some()))
            .count();

        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(total_verts);
        let mut normals: Vec<[f32; 3]> = if all_have_normals {
            Vec::with_capacity(total_verts)
        } else {
            Vec::new()
        };
        let mut tangents: Vec<[f32; 4]> = if all_have_tangents {
            Vec::with_capacity(total_verts)
        } else {
            Vec::new()
        };
        let mut uvs: Vec<Vec<[f32; 2]>> = (0..uv_channel_count)
            .map(|_| Vec::with_capacity(total_verts))
            .collect();
        let mut sub_meshes: Vec<UnitySubMesh> = Vec::with_capacity(prims.len());

        for GltfPrimitiveData {
            positions,
            normals: prim_norms,
            tangents: prim_tangs,
            uvs: prim_uvs,
            wound,
        } in prims
        {
            let base = vertices.len() as u32;
            vertices.extend_from_slice(&positions);

            if let Some(n) = prim_norms {
                normals.extend_from_slice(&n);
            }

            if let Some(t) = prim_tangs {
                tangents.extend_from_slice(&t);
            }

            for (ch, ch_buf) in uvs.iter_mut().enumerate() {
                if let Some(Some(ch_uvs)) = prim_uvs.get(ch) {
                    ch_buf.extend_from_slice(ch_uvs);
                }
            }

            let indices = if use_u32 {
                UnityIndices::U32(wound.into_iter().map(|i| i + base).collect())
            } else {
                UnityIndices::U16(wound.into_iter().map(|i| (i + base) as u16).collect())
            };

            sub_meshes.push(UnitySubMesh { indices });
        }

        let name = mesh.name.clone().unwrap_or_else(|| mesh_idx.to_string());

        meshes.insert(
            mesh_idx as u32,
            UnityMesh {
                name,
                vertices,
                normals,
                tangents,
                uvs,
                sub_meshes,
            },
        );
    }

    Ok(UnityGltf {
        scene_name,
        root_nodes,
        nodes,
        meshes,
    })
}

/// Resolve a single glTF primitive into left-handed positions, optional normals, and wound indices.
#[track_caller]
fn resolve_primitive(
    prim: &MeshPrimitive,
    accessors: &[gltforge::schema::Accessor],
    bvs: &[gltforge::schema::BufferView],
    buffers: &[Vec<u8>],
) -> ConvertResult<GltfPrimitiveData> {
    if prim.mode != MeshPrimitiveMode::Triangles {
        return Err(ConvertError::UnsupportedPrimitiveMode {
            mode: prim.mode,
            location: ErrorLocation::from(Location::caller()),
        });
    }

    // ---- positions ----------------------------------------------------------

    let pos_id =
        *prim
            .attributes
            .get("POSITION")
            .ok_or_else(|| ConvertError::NoPositionAttribute {
                location: ErrorLocation::from(Location::caller()),
            })? as usize;

    let pos_acc =
        accessors
            .get(pos_id)
            .ok_or_else(|| ConvertError::PositionAccessorOutOfRange {
                location: ErrorLocation::from(Location::caller()),
            })?;

    if pos_acc.accessor_type != AccessorType::Vec3
        || pos_acc.component_type != AccessorComponentType::Float
    {
        return Err(ConvertError::InvalidPositionType {
            location: ErrorLocation::from(Location::caller()),
        });
    }

    let pos_bytes = resolve_accessor(pos_acc, bvs, buffers).map_err(|e| ConvertError::Resolve {
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })?;

    let positions: Vec<[f32; 3]> = pos_bytes
        .chunks_exact(12)
        .map(|c| {
            let x = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let y = f32::from_le_bytes([c[4], c[5], c[6], c[7]]);
            let z = f32::from_le_bytes([c[8], c[9], c[10], c[11]]);
            [-x, y, z]
        })
        .collect();

    // ---- indices ------------------------------------------------------------

    let idx_id = prim.indices.ok_or_else(|| ConvertError::NoIndices {
        location: ErrorLocation::from(Location::caller()),
    })?;

    let idx_acc =
        accessors
            .get(idx_id as usize)
            .ok_or_else(|| ConvertError::IndexAccessorOutOfRange {
                location: ErrorLocation::from(Location::caller()),
            })?;

    let idx_bytes = resolve_accessor(idx_acc, bvs, buffers).map_err(|e| ConvertError::Resolve {
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })?;

    let raw = decode_indices(idx_bytes, idx_acc.component_type)?;

    // Reverse winding order (glTF right-handed → Unity left-handed).
    let wound: Vec<u32> = raw
        .chunks_exact(3)
        .flat_map(|tri| [tri[0], tri[2], tri[1]])
        .collect();

    // ---- normals (optional) -------------------------------------------------

    let normals = if let Some(&norm_id) = prim.attributes.get("NORMAL") {
        let norm_acc = accessors.get(norm_id as usize).ok_or_else(|| {
            ConvertError::PositionAccessorOutOfRange {
                location: ErrorLocation::from(Location::caller()),
            }
        })?;

        let norm_bytes =
            resolve_accessor(norm_acc, bvs, buffers).map_err(|e| ConvertError::Resolve {
                source: e,
                location: ErrorLocation::from(Location::caller()),
            })?;

        Some(
            norm_bytes
                .chunks_exact(12)
                .map(|c| {
                    let x = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                    let y = f32::from_le_bytes([c[4], c[5], c[6], c[7]]);
                    let z = f32::from_le_bytes([c[8], c[9], c[10], c[11]]);
                    [-x, y, z]
                })
                .collect(),
        )
    } else {
        None
    };

    // ---- tangents (optional) ------------------------------------------------

    let tangents = if let Some(&tang_id) = prim.attributes.get("TANGENT") {
        let tang_acc = accessors.get(tang_id as usize).ok_or_else(|| {
            ConvertError::PositionAccessorOutOfRange {
                location: ErrorLocation::from(Location::caller()),
            }
        })?;

        let tang_bytes =
            resolve_accessor(tang_acc, bvs, buffers).map_err(|e| ConvertError::Resolve {
                source: e,
                location: ErrorLocation::from(Location::caller()),
            })?;

        Some(
            tang_bytes
                .chunks_exact(16)
                .map(|c| {
                    let x = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                    let y = f32::from_le_bytes([c[4], c[5], c[6], c[7]]);
                    let z = f32::from_le_bytes([c[8], c[9], c[10], c[11]]);
                    let w = f32::from_le_bytes([c[12], c[13], c[14], c[15]]);
                    // Negate X (coordinate flip) and W (bitangent handedness flip).
                    [-x, y, z, -w]
                })
                .collect(),
        )
    } else {
        None
    };

    // ---- UV channels (optional, TEXCOORD_0 … TEXCOORD_7) -------------------

    let mut uvs: Vec<Option<Vec<[f32; 2]>>> = Vec::new();
    for ch in 0u32..8 {
        let key = format!("TEXCOORD_{ch}");
        let Some(&uv_id) = prim.attributes.get(&key) else {
            break; // Stop at the first absent channel.
        };

        let uv_acc = accessors.get(uv_id as usize).ok_or_else(|| {
            ConvertError::PositionAccessorOutOfRange {
                location: ErrorLocation::from(Location::caller()),
            }
        })?;

        let uv_bytes =
            resolve_accessor(uv_acc, bvs, buffers).map_err(|e| ConvertError::Resolve {
                source: e,
                location: ErrorLocation::from(Location::caller()),
            })?;

        let channel: Vec<[f32; 2]> = uv_bytes
            .chunks_exact(8)
            .map(|c| {
                let u = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                let v = f32::from_le_bytes([c[4], c[5], c[6], c[7]]);
                // Flip V: glTF origin is top-left, Unity origin is bottom-left.
                [u, 1.0 - v]
            })
            .collect();

        uvs.push(Some(channel));
    }

    Ok(GltfPrimitiveData {
        positions,
        normals,
        tangents,
        uvs,
        wound,
    })
}

/// Build a [`UnityNodeTransform`] from a glTF node, converting to Unity's left-handed coordinate system.
///
/// Handles both the `matrix` form (column-major 4×4, decomposed into TRS) and the
/// separate `translation`/`rotation`/`scale` properties. Missing components default to identity.
fn node_transform(node: &gltforge::schema::Node) -> UnityNodeTransform {
    if let Some(m) = &node.matrix {
        mat4_to_node_transform(m)
    } else {
        let position = node
            .translation
            .map(|t| [-t[0], t[1], t[2]])
            .unwrap_or([0.0, 0.0, 0.0]);
        let rotation = node
            .rotation
            .map(|r| [-r[0], r[1], r[2], -r[3]])
            .unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let scale = node.scale.unwrap_or([1.0, 1.0, 1.0]);
        UnityNodeTransform {
            position,
            rotation,
            scale,
        }
    }
}

/// Decompose a glTF column-major 4×4 matrix into TRS and convert to Unity left-handed space.
fn mat4_to_node_transform(m: &[f32; 16]) -> UnityNodeTransform {
    // glTF matrix is column-major: column k starts at index k*4.
    // Translation is the last column.
    let tx = m[12];
    let ty = m[13];
    let tz = m[14];

    // Scale = length of each basis column (columns 0, 1, 2).
    let sx = (m[0] * m[0] + m[1] * m[1] + m[2] * m[2]).sqrt();
    let sy = (m[4] * m[4] + m[5] * m[5] + m[6] * m[6]).sqrt();
    let sz = (m[8] * m[8] + m[9] * m[9] + m[10] * m[10]).sqrt();

    // Rotation matrix: normalize each basis column.
    // Row-major indexing: r[row][col] = m[col*4 + row] / scale[col]
    let r00 = m[0] / sx;
    let r10 = m[1] / sx;
    let r20 = m[2] / sx;
    let r01 = m[4] / sy;
    let r11 = m[5] / sy;
    let r21 = m[6] / sy;
    let r02 = m[8] / sz;
    let r12 = m[9] / sz;
    let r22 = m[10] / sz;

    let [qx, qy, qz, qw] = rot_mat_to_quat([r00, r01, r02, r10, r11, r12, r20, r21, r22]);

    UnityNodeTransform {
        position: [-tx, ty, tz],
        rotation: [-qx, qy, qz, -qw],
        scale: [sx, sy, sz],
    }
}

/// Convert a 3×3 rotation matrix (row-major, packed as `[r00,r01,r02, r10,r11,r12, r20,r21,r22]`)
/// to a unit quaternion `[x, y, z, w]`.
///
/// Uses Shepperd's method for numerical stability.
fn rot_mat_to_quat([r00, r01, r02, r10, r11, r12, r20, r21, r22]: [f32; 9]) -> [f32; 4] {
    let trace = r00 + r11 + r22;

    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0; // s = 4w
        let w = 0.25 * s;
        let x = (r21 - r12) / s;
        let y = (r02 - r20) / s;
        let z = (r10 - r01) / s;
        [x, y, z, w]
    } else if r00 > r11 && r00 > r22 {
        let s = (1.0 + r00 - r11 - r22).sqrt() * 2.0; // s = 4x
        let w = (r21 - r12) / s;
        let x = 0.25 * s;
        let y = (r01 + r10) / s;
        let z = (r02 + r20) / s;
        [x, y, z, w]
    } else if r11 > r22 {
        let s = (1.0 + r11 - r00 - r22).sqrt() * 2.0; // s = 4y
        let w = (r02 - r20) / s;
        let x = (r01 + r10) / s;
        let y = 0.25 * s;
        let z = (r12 + r21) / s;
        [x, y, z, w]
    } else {
        let s = (1.0 + r22 - r00 - r11).sqrt() * 2.0; // s = 4z
        let w = (r10 - r01) / s;
        let x = (r02 + r20) / s;
        let y = (r12 + r21) / s;
        let z = 0.25 * s;
        [x, y, z, w]
    }
}

/// Decode raw index bytes into a flat `Vec<u32>` regardless of source format.
#[track_caller]
fn decode_indices(bytes: &[u8], component_type: AccessorComponentType) -> ConvertResult<Vec<u32>> {
    match component_type {
        AccessorComponentType::UnsignedByte => Ok(bytes.iter().map(|&b| b as u32).collect()),
        AccessorComponentType::UnsignedShort => Ok(bytes
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]) as u32)
            .collect()),
        AccessorComponentType::UnsignedInt => Ok(bytes
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect()),
        other => Err(ConvertError::UnsupportedIndexComponentType {
            component_type: other,
            location: ErrorLocation::from(Location::caller()),
        }),
    }
}
