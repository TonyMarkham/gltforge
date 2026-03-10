use std::panic::Location;

use error_location::ErrorLocation;
use gltforge::parser::resolve_accessor;
use gltforge::schema::{AccessorComponentType, AccessorType, Gltf, MeshPrimitiveMode};

use crate::error::{ConvertError, ConvertResult};
use crate::mesh::{UnityIndices, UnityMesh, UnitySubmesh};

/// Build a [`UnityMesh`] from a glTF node, loading all of its mesh's primitives as submeshes.
///
/// Applies the right-handed → left-handed coordinate conversion (negate X) and reverses
/// triangle winding order. Vertices are concatenated across primitives into a single shared
/// buffer; indices in each submesh are absolute into that buffer. Index format (u16/u32) is
/// selected once based on the total vertex count across all primitives.
#[track_caller]
pub fn build_unity_mesh(
    gltf: &Gltf,
    buffers: &[Vec<u8>],
    node_idx: u32,
) -> ConvertResult<UnityMesh> {
    // ---- node ---------------------------------------------------------------

    let nodes = gltf.nodes.as_deref().ok_or_else(|| ConvertError::NoNodes {
        location: ErrorLocation::from(Location::caller()),
    })?;

    let node = nodes
        .get(node_idx as usize)
        .ok_or_else(|| ConvertError::NodeIndexOutOfRange {
            index: node_idx as usize,
            location: ErrorLocation::from(Location::caller()),
        })?;

    let mesh_idx = node.mesh.ok_or_else(|| ConvertError::NodeHasNoMesh {
        index: node_idx as usize,
        location: ErrorLocation::from(Location::caller()),
    })?;

    // ---- mesh ---------------------------------------------------------------

    let meshes = gltf
        .meshes
        .as_deref()
        .ok_or_else(|| ConvertError::NoMeshes {
            location: ErrorLocation::from(Location::caller()),
        })?;

    let mesh = meshes
        .get(mesh_idx as usize)
        .ok_or_else(|| ConvertError::MeshIndexOutOfRange {
            index: mesh_idx as usize,
            location: ErrorLocation::from(Location::caller()),
        })?;

    let bvs = gltf.buffer_views.as_deref().unwrap_or(&[]);
    let accessors = gltf.accessors.as_deref().unwrap_or(&[]);

    // ---- primitives ---------------------------------------------------------

    let mut all_positions: Vec<[f32; 3]> = Vec::new();
    let mut raw_submeshes: Vec<Vec<u32>> = Vec::new();

    for prim in &mesh.primitives {
        if prim.mode != MeshPrimitiveMode::Triangles {
            return Err(ConvertError::UnsupportedPrimitiveMode {
                mode: prim.mode,
                location: ErrorLocation::from(Location::caller()),
            });
        }

        let vertex_offset = all_positions.len() as u32;

        // positions
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

        let pos_bytes =
            resolve_accessor(pos_acc, bvs, buffers).map_err(|e| ConvertError::Resolve {
                source: e,
                location: ErrorLocation::from(Location::caller()),
            })?;

        all_positions.extend(pos_bytes.chunks_exact(12).map(|c| {
            let x = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let y = f32::from_le_bytes([c[4], c[5], c[6], c[7]]);
            let z = f32::from_le_bytes([c[8], c[9], c[10], c[11]]);
            [-x, y, z]
        }));

        // indices
        let idx_id = prim.indices.ok_or_else(|| ConvertError::NoIndices {
            location: ErrorLocation::from(Location::caller()),
        })?;

        let idx_acc = accessors.get(idx_id as usize).ok_or_else(|| {
            ConvertError::IndexAccessorOutOfRange {
                location: ErrorLocation::from(Location::caller()),
            }
        })?;

        let idx_bytes =
            resolve_accessor(idx_acc, bvs, buffers).map_err(|e| ConvertError::Resolve {
                source: e,
                location: ErrorLocation::from(Location::caller()),
            })?;

        let raw = decode_indices(idx_bytes, idx_acc.component_type)?;

        // Reverse winding and offset into the shared vertex buffer.
        let wound: Vec<u32> = raw
            .chunks_exact(3)
            .flat_map(|tri| {
                [
                    tri[0] + vertex_offset,
                    tri[2] + vertex_offset,
                    tri[1] + vertex_offset,
                ]
            })
            .collect();

        raw_submeshes.push(wound);
    }

    // ---- index format -------------------------------------------------------

    let total_vertices = all_positions.len();
    let use_u16 = total_vertices <= 65535;

    let submeshes: Vec<UnitySubmesh> = raw_submeshes
        .into_iter()
        .map(|wound| UnitySubmesh {
            indices: if use_u16 {
                UnityIndices::U16(wound.into_iter().map(|i| i as u16).collect())
            } else {
                UnityIndices::U32(wound)
            },
        })
        .collect();

    // ---- name ---------------------------------------------------------------

    let name = derive_mesh_name(
        node.name.as_deref(),
        mesh.name.as_deref(),
        node_idx,
        mesh_idx,
        mesh.primitives.len(),
    );

    Ok(UnityMesh {
        name,
        positions: all_positions,
        submeshes,
    })
}

fn derive_mesh_name(
    node_name: Option<&str>,
    mesh_name: Option<&str>,
    node_idx: u32,
    mesh_idx: u32,
    prim_count: usize,
) -> String {
    match (node_name, mesh_name) {
        (Some(n), Some(m)) => format!("{n}_{m}"),
        (Some(n), None) if prim_count == 1 => n.to_string(),
        (Some(n), None) => format!("{n}_{mesh_idx}"),
        (None, _) => format!("{node_idx}_{mesh_idx}"),
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
