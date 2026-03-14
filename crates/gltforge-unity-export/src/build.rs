use crate::{
    error::{ExportError, ExportResult},
    export_context::ExportContext,
};

use gltforge::schema::{
    Accessor, AccessorComponentType, AccessorType, Asset, Buffer, BufferView, BufferViewTarget,
    Gltf, GltfId, Mesh, MeshPrimitive, MeshPrimitiveMode, Node, Scene,
};

use bytemuck::cast_slice;
use error_location::ErrorLocation;
use std::{collections::HashMap, panic::Location, path::Path};

pub(crate) fn write(ctx: ExportContext, gltf_path: &Path) -> ExportResult<()> {
    let stem = gltf_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let bin_name = format!("{stem}.bin");
    let bin_path = gltf_path.with_file_name(&bin_name);

    let (gltf, binary) = build_gltf(&ctx, Some(&bin_name));

    std::fs::write(&bin_path, &binary).map_err(|e| ExportError::Io {
        path: bin_path.to_string_lossy().into_owned(),
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })?;

    let json = serde_json::to_string_pretty(&gltf).map_err(|e| ExportError::Json {
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })?;

    std::fs::write(gltf_path, json).map_err(|e| ExportError::Io {
        path: gltf_path.to_string_lossy().into_owned(),
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })?;

    Ok(())
}

pub(crate) fn write_glb(ctx: ExportContext, glb_path: &Path) -> ExportResult<()> {
    let (gltf, binary) = build_gltf(&ctx, None);

    let json = serde_json::to_string(&gltf).map_err(|e| ExportError::Json {
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })?;
    let json_bytes = json.as_bytes();
    let json_padded_len = (json_bytes.len() + 3) & !3;
    let json_padding = json_padded_len - json_bytes.len();

    let has_bin = !binary.is_empty();
    let bin_padded_len = if has_bin { (binary.len() + 3) & !3 } else { 0 };
    let bin_padding = bin_padded_len - binary.len();

    let total_len: u32 = 12 // GLB header
        + 8 + json_padded_len as u32 // JSON chunk
        + if has_bin { 8 + bin_padded_len as u32 } else { 0 }; // BIN chunk

    let mut out: Vec<u8> = Vec::with_capacity(total_len as usize);

    // GLB header
    out.extend_from_slice(&0x46546C67u32.to_le_bytes()); // magic "glTF"
    out.extend_from_slice(&2u32.to_le_bytes()); // version
    out.extend_from_slice(&total_len.to_le_bytes()); // total file length

    // JSON chunk
    out.extend_from_slice(&(json_padded_len as u32).to_le_bytes());
    out.extend_from_slice(&0x4E4F534Au32.to_le_bytes()); // "JSON"
    out.extend_from_slice(json_bytes);
    out.extend(std::iter::repeat_n(0x20u8, json_padding)); // pad with spaces

    // BIN chunk
    if has_bin {
        out.extend_from_slice(&(bin_padded_len as u32).to_le_bytes());
        out.extend_from_slice(&0x004E4942u32.to_le_bytes()); // "BIN\0"
        out.extend_from_slice(&binary);
        out.extend(std::iter::repeat_n(0x00u8, bin_padding));
    }

    std::fs::write(glb_path, &out).map_err(|e| ExportError::Io {
        path: glb_path.to_string_lossy().into_owned(),
        source: e,
        location: ErrorLocation::from(Location::caller()),
    })
}

fn build_gltf(ctx: &ExportContext, bin_uri: Option<&str>) -> (Gltf, Vec<u8>) {
    let mut binary: Vec<u8> = Vec::new();
    let mut buffer_views: Vec<BufferView> = Vec::new();
    let mut accessors: Vec<Accessor> = Vec::new();

    struct MeshAccessors {
        position: GltfId,
        normal: Option<GltfId>,
        uvs: Vec<GltfId>,
        submesh_indices: Vec<GltfId>,
    }

    let mut mesh_accessor_map: Vec<MeshAccessors> = Vec::new();

    for mesh_data in &ctx.meshes {
        let use_u16 = mesh_data.positions.len() <= 65535;

        let gltf_positions = to_gltf_positions(&mesh_data.positions);
        let position = push_vec3(
            &mut binary,
            &mut buffer_views,
            &mut accessors,
            &gltf_positions,
            BufferViewTarget::ArrayBuffer,
            true,
        );

        let normal = if !mesh_data.normals.is_empty() {
            let gltf_normals = to_gltf_normals(&mesh_data.normals);
            Some(push_vec3(
                &mut binary,
                &mut buffer_views,
                &mut accessors,
                &gltf_normals,
                BufferViewTarget::ArrayBuffer,
                false,
            ))
        } else {
            None
        };

        let uvs: Vec<GltfId> = mesh_data
            .uvs
            .iter()
            .map(|ch| {
                let gltf_uvs = to_gltf_uvs(ch);
                push_vec2(
                    &mut binary,
                    &mut buffer_views,
                    &mut accessors,
                    &gltf_uvs,
                    BufferViewTarget::ArrayBuffer,
                )
            })
            .collect();

        let submesh_indices: Vec<GltfId> = mesh_data
            .submeshes
            .iter()
            .map(|sm| {
                let gltf_indices = reverse_winding(&sm.indices);
                if use_u16 {
                    let indices_u16: Vec<u16> = gltf_indices.iter().map(|&i| i as u16).collect();
                    push_indices_u16(&mut binary, &mut buffer_views, &mut accessors, &indices_u16)
                } else {
                    push_indices_u32(
                        &mut binary,
                        &mut buffer_views,
                        &mut accessors,
                        &gltf_indices,
                    )
                }
            })
            .collect();

        mesh_accessor_map.push(MeshAccessors {
            position,
            normal,
            uvs,
            submesh_indices,
        });
    }

    let gltf_meshes: Vec<Mesh> = ctx
        .meshes
        .iter()
        .zip(&mesh_accessor_map)
        .map(|(mesh_data, accs)| {
            let primitives = mesh_data
                .submeshes
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    let mut attributes = HashMap::new();
                    attributes.insert("POSITION".to_string(), accs.position);
                    if let Some(n) = accs.normal {
                        attributes.insert("NORMAL".to_string(), n);
                    }
                    for (ch, &uv_acc) in accs.uvs.iter().enumerate() {
                        attributes.insert(format!("TEXCOORD_{ch}"), uv_acc);
                    }
                    MeshPrimitive {
                        attributes,
                        indices: Some(accs.submesh_indices[i]),
                        material: None,
                        mode: MeshPrimitiveMode::default(),
                        targets: None,
                        extensions: None,
                        extras: None,
                    }
                })
                .collect();

            Mesh {
                primitives,
                weights: None,
                name: mesh_data.name.clone(),
                extensions: None,
                extras: None,
            }
        })
        .collect();

    let gltf_nodes: Vec<Node> = ctx
        .nodes
        .iter()
        .enumerate()
        .map(|(idx, n)| {
            let children: Vec<u32> = ctx
                .nodes
                .iter()
                .enumerate()
                .filter(|(_, cn)| cn.parent == Some(idx as u32))
                .map(|(ci, _)| ci as u32)
                .collect();

            // Unity left-handed → glTF right-handed: negate X on position, negate X+W on rotation.
            // canon() eliminates negative zero. Omit identity components entirely.
            let translation = n.translation.and_then(|[x, y, z]| {
                let t = [canon(-x), canon(y), canon(z)];
                if t == [0.0, 0.0, 0.0] { None } else { Some(t) }
            });
            let rotation = n.rotation.and_then(|[x, y, z, w]| {
                let r = canon_quat([canon(-x), canon(y), canon(z), canon(-w)]);
                if r == [0.0, 0.0, 0.0, 1.0] {
                    None
                } else {
                    Some(r)
                }
            });

            Node {
                children: if children.is_empty() {
                    None
                } else {
                    Some(children)
                },
                mesh: n.mesh_index,
                skin: None,
                camera: None,
                matrix: None,
                translation,
                rotation,
                scale: n.scale.filter(|&s| s != [1.0, 1.0, 1.0]),
                weights: None,
                name: n.name.clone(),
                extensions: None,
                extras: None,
            }
        })
        .collect();

    let root_nodes: Vec<u32> = ctx
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.parent.is_none())
        .map(|(i, _)| i as u32)
        .collect();

    let bin_len = binary.len() as u32;

    let gltf = Gltf {
        asset: Asset {
            version: "2.0".to_string(),
            generator: Some("gltforge".to_string()),
            copyright: None,
            min_version: None,
            extensions: None,
            extras: None,
        },
        scene: Some(0),
        scenes: Some(vec![Scene {
            nodes: if root_nodes.is_empty() {
                None
            } else {
                Some(root_nodes)
            },
            name: None,
            extensions: None,
            extras: None,
        }]),
        nodes: if gltf_nodes.is_empty() {
            None
        } else {
            Some(gltf_nodes)
        },
        meshes: if gltf_meshes.is_empty() {
            None
        } else {
            Some(gltf_meshes)
        },
        accessors: if accessors.is_empty() {
            None
        } else {
            Some(accessors)
        },
        buffer_views: if buffer_views.is_empty() {
            None
        } else {
            Some(buffer_views)
        },
        buffers: if binary.is_empty() {
            None
        } else {
            Some(vec![Buffer {
                byte_length: bin_len,
                uri: bin_uri.map(|s| s.to_string()),
                name: None,
                extensions: None,
                extras: None,
            }])
        },
        animations: None,
        cameras: None,
        images: None,
        materials: None,
        samplers: None,
        skins: None,
        textures: None,
        extensions_used: None,
        extensions_required: None,
        extensions: None,
        extras: None,
    };

    (gltf, binary)
}

// --- Coordinate conversions --------------------------------------------------

/// Canonicalize negative zero to positive zero.
fn canon(x: f32) -> f32 {
    if x == 0.0 { 0.0 } else { x }
}

/// Canonicalize a quaternion to have non-negative W (q and -q represent the same rotation).
fn canon_quat([x, y, z, w]: [f32; 4]) -> [f32; 4] {
    if w < 0.0 {
        [-x, -y, -z, -w]
    } else {
        [x, y, z, w]
    }
}

fn to_gltf_positions(positions: &[[f32; 3]]) -> Vec<[f32; 3]> {
    positions.iter().map(|&[x, y, z]| [-x, y, z]).collect()
}

fn to_gltf_normals(normals: &[[f32; 3]]) -> Vec<[f32; 3]> {
    normals.iter().map(|&[x, y, z]| [-x, y, z]).collect()
}

fn to_gltf_uvs(uvs: &[[f32; 2]]) -> Vec<[f32; 2]> {
    uvs.iter().map(|&[u, v]| [u, 1.0 - v]).collect()
}

fn reverse_winding(indices: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(indices.len());
    for tri in indices.chunks_exact(3) {
        out.push(tri[0]);
        out.push(tri[2]);
        out.push(tri[1]);
    }
    out
}

// --- Buffer packing helpers --------------------------------------------------

fn align_to(buf: &mut Vec<u8>, alignment: usize) {
    let rem = buf.len() % alignment;
    if rem != 0 {
        buf.extend(std::iter::repeat_n(0u8, alignment - rem));
    }
}

fn push_vec3(
    binary: &mut Vec<u8>,
    buffer_views: &mut Vec<BufferView>,
    accessors: &mut Vec<Accessor>,
    data: &[[f32; 3]],
    target: BufferViewTarget,
    compute_min_max: bool,
) -> GltfId {
    align_to(binary, 4);
    let byte_offset = binary.len() as u32;
    let bytes: &[u8] = cast_slice(data);
    binary.extend_from_slice(bytes);

    let bv_idx = buffer_views.len() as u32;
    buffer_views.push(BufferView {
        buffer: 0,
        byte_offset,
        byte_length: bytes.len() as u32,
        byte_stride: None,
        target: Some(target),
        name: None,
        extensions: None,
        extras: None,
    });

    let (min, max) = if compute_min_max && !data.is_empty() {
        let mut mn = data[0];
        let mut mx = data[0];
        for &[x, y, z] in data.iter().skip(1) {
            mn[0] = mn[0].min(x);
            mn[1] = mn[1].min(y);
            mn[2] = mn[2].min(z);
            mx[0] = mx[0].max(x);
            mx[1] = mx[1].max(y);
            mx[2] = mx[2].max(z);
        }
        (
            Some(vec![mn[0] as f64, mn[1] as f64, mn[2] as f64]),
            Some(vec![mx[0] as f64, mx[1] as f64, mx[2] as f64]),
        )
    } else {
        (None, None)
    };

    let acc_idx = accessors.len() as u32;
    accessors.push(Accessor {
        buffer_view: Some(bv_idx),
        byte_offset: None,
        component_type: AccessorComponentType::Float,
        count: data.len() as u32,
        accessor_type: AccessorType::Vec3,
        normalized: None,
        min,
        max,
        sparse: None,
        name: None,
        extensions: None,
        extras: None,
    });
    acc_idx
}

fn push_vec2(
    binary: &mut Vec<u8>,
    buffer_views: &mut Vec<BufferView>,
    accessors: &mut Vec<Accessor>,
    data: &[[f32; 2]],
    target: BufferViewTarget,
) -> GltfId {
    align_to(binary, 4);
    let byte_offset = binary.len() as u32;
    let bytes: &[u8] = cast_slice(data);
    binary.extend_from_slice(bytes);

    let bv_idx = buffer_views.len() as u32;
    buffer_views.push(BufferView {
        buffer: 0,
        byte_offset,
        byte_length: bytes.len() as u32,
        byte_stride: None,
        target: Some(target),
        name: None,
        extensions: None,
        extras: None,
    });

    let acc_idx = accessors.len() as u32;
    accessors.push(Accessor {
        buffer_view: Some(bv_idx),
        byte_offset: None,
        component_type: AccessorComponentType::Float,
        count: data.len() as u32,
        accessor_type: AccessorType::Vec2,
        normalized: None,
        min: None,
        max: None,
        sparse: None,
        name: None,
        extensions: None,
        extras: None,
    });
    acc_idx
}

fn push_indices_u16(
    binary: &mut Vec<u8>,
    buffer_views: &mut Vec<BufferView>,
    accessors: &mut Vec<Accessor>,
    indices: &[u16],
) -> GltfId {
    align_to(binary, 2);
    let byte_offset = binary.len() as u32;
    let bytes: &[u8] = cast_slice(indices);
    binary.extend_from_slice(bytes);

    let bv_idx = buffer_views.len() as u32;
    buffer_views.push(BufferView {
        buffer: 0,
        byte_offset,
        byte_length: bytes.len() as u32,
        byte_stride: None,
        target: Some(BufferViewTarget::ElementArrayBuffer),
        name: None,
        extensions: None,
        extras: None,
    });

    let acc_idx = accessors.len() as u32;
    accessors.push(Accessor {
        buffer_view: Some(bv_idx),
        byte_offset: None,
        component_type: AccessorComponentType::UnsignedShort,
        count: indices.len() as u32,
        accessor_type: AccessorType::Scalar,
        normalized: None,
        min: None,
        max: None,
        sparse: None,
        name: None,
        extensions: None,
        extras: None,
    });
    acc_idx
}

fn push_indices_u32(
    binary: &mut Vec<u8>,
    buffer_views: &mut Vec<BufferView>,
    accessors: &mut Vec<Accessor>,
    indices: &[u32],
) -> GltfId {
    align_to(binary, 4);
    let byte_offset = binary.len() as u32;
    let bytes: &[u8] = cast_slice(indices);
    binary.extend_from_slice(bytes);

    let bv_idx = buffer_views.len() as u32;
    buffer_views.push(BufferView {
        buffer: 0,
        byte_offset,
        byte_length: bytes.len() as u32,
        byte_stride: None,
        target: Some(BufferViewTarget::ElementArrayBuffer),
        name: None,
        extensions: None,
        extras: None,
    });

    let acc_idx = accessors.len() as u32;
    accessors.push(Accessor {
        buffer_view: Some(bv_idx),
        byte_offset: None,
        component_type: AccessorComponentType::UnsignedInt,
        count: indices.len() as u32,
        accessor_type: AccessorType::Scalar,
        normalized: None,
        min: None,
        max: None,
        sparse: None,
        name: None,
        extensions: None,
        extras: None,
    });
    acc_idx
}
