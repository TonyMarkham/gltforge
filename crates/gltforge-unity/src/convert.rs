use error_location::ErrorLocation;
use gltforge::{
    parser::resolve_accessor,
    schema::{AccessorComponentType, AccessorType, Gltf, MeshPrimitive, MeshPrimitiveMode},
};
use std::{collections::HashMap, panic::Location};

use crate::{
    error::{ConvertError, ConvertResult},
    unity_gltf::UnityGltf,
    unity_indices::UnityIndices,
    unity_mesh::UnityMesh,
    unity_node::UnityNode,
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
            },
        );
    }

    // ---- meshes -------------------------------------------------------------

    let bvs = gltf.buffer_views.as_deref().unwrap_or(&[]);
    let accessors = gltf.accessors.as_deref().unwrap_or(&[]);
    let mut meshes: HashMap<u32, UnityMesh> = HashMap::new();

    for (mesh_idx, mesh) in gltf.meshes.as_deref().unwrap_or(&[]).iter().enumerate() {
        // --- First pass: resolve each primitive's positions and wound indices ---

        let mut prim_positions: Vec<Vec<[f32; 3]>> = Vec::new();
        let mut prim_indices: Vec<Vec<u32>> = Vec::new();

        for prim in &mesh.primitives {
            let (positions, wound) = resolve_primitive(prim, accessors, bvs, buffers)?;
            prim_positions.push(positions);
            prim_indices.push(wound);
        }

        // --- Second pass: merge vertices, offset indices, pick format --------

        let total_verts: usize = prim_positions.iter().map(|p| p.len()).sum();
        let use_u32 = total_verts > 65535;

        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(total_verts);
        let mut sub_meshes: Vec<UnitySubMesh> = Vec::with_capacity(prim_positions.len());

        for (positions, wound) in prim_positions.into_iter().zip(prim_indices) {
            let base = vertices.len() as u32;
            vertices.extend_from_slice(&positions);

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

/// Resolve a single glTF primitive into left-handed positions and wound indices.
#[track_caller]
fn resolve_primitive(
    prim: &MeshPrimitive,
    accessors: &[gltforge::schema::Accessor],
    bvs: &[gltforge::schema::BufferView],
    buffers: &[Vec<u8>],
) -> ConvertResult<(Vec<[f32; 3]>, Vec<u32>)> {
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

    Ok((positions, wound))
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
