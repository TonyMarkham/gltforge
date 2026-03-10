use std::panic::Location;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use error_location::ErrorLocation;
use gltforge::parser;
use gltforge::schema::{AccessorComponentType, AccessorType, Gltf, MeshPrimitiveMode, Node};

mod error;
use error::{CliError, CliResult};

#[derive(Parser)]
#[command(name = "gltforge", about = "glTF 2.0 toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Inspect the contents of a glTF file.
    Inspect {
        /// Path to the .gltf file.
        path: PathBuf,

        /// Print node hierarchy and transforms.
        #[arg(long)]
        nodes: bool,

        /// Print mesh and primitive details.
        #[arg(long)]
        mesh: bool,

        /// Dump raw POSITION vertices for mesh N (pre-conversion, glTF coordinates).
        #[arg(long, value_name = "MESH_INDEX")]
        dump_verts: Option<usize>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Inspect {
            path,
            nodes,
            mesh,
            dump_verts,
        } => match inspect(&path, nodes, mesh, dump_verts) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
    }
}

#[track_caller]
fn inspect(path: &PathBuf, nodes: bool, mesh: bool, dump_verts: Option<usize>) -> CliResult<()> {
    let json = std::fs::read_to_string(path).map_err(|source| CliError::ReadFile {
        path: path.clone(),
        source,
        location: ErrorLocation::from(Location::caller()),
    })?;

    let gltf = parser::parse(&json)?;

    print_summary(&gltf);

    if nodes {
        println!();
        print_nodes(&gltf);
    }

    if mesh {
        println!();
        print_meshes(&gltf);
    }

    if let Some(mesh_idx) = dump_verts {
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));
        let buffers = parser::load_buffers(&gltf, base_dir).map_err(CliError::Parse)?;
        println!();
        dump_mesh_verts(&gltf, &buffers, mesh_idx);
    }

    Ok(())
}

fn dump_mesh_verts(gltf: &Gltf, buffers: &[Vec<u8>], mesh_idx: usize) {
    let meshes = gltf.meshes.as_deref().unwrap_or(&[]);
    let Some(mesh) = meshes.get(mesh_idx) else {
        eprintln!("mesh {mesh_idx} not found");
        return;
    };
    let accessors = gltf.accessors.as_deref().unwrap_or(&[]);
    let bvs = gltf.buffer_views.as_deref().unwrap_or(&[]);

    let name = mesh.name.as_deref().unwrap_or("<unnamed>");
    println!("dump_verts mesh {mesh_idx}: {name}");

    for (pi, prim) in mesh.primitives.iter().enumerate() {
        let Some(&pos_id) = prim.attributes.get("POSITION") else {
            println!("  primitive {pi}: no POSITION attribute");
            continue;
        };
        let Some(acc) = accessors.get(pos_id as usize) else {
            println!("  primitive {pi}: accessor {pos_id} out of range");
            continue;
        };
        if acc.accessor_type != AccessorType::Vec3
            || acc.component_type != AccessorComponentType::Float
        {
            println!("  primitive {pi}: POSITION is not VEC3 FLOAT");
            continue;
        }

        let bv_idx = match acc.buffer_view {
            Some(i) => i as usize,
            None => {
                println!("  primitive {pi}: accessor has no buffer view");
                continue;
            }
        };
        let Some(bv) = bvs.get(bv_idx) else {
            println!("  primitive {pi}: buffer view {bv_idx} out of range");
            continue;
        };
        let Some(buf) = buffers.get(bv.buffer as usize) else {
            println!("  primitive {pi}: buffer {} out of range", bv.buffer);
            continue;
        };

        let element_size = 12usize; // VEC3 float
        let stride = bv.byte_stride.unwrap_or(element_size as u32) as usize;
        let base = bv.byte_offset as usize + acc.byte_offset.unwrap_or(0) as usize;

        println!(
            "  primitive {pi}: {} verts  accessor {}  bv {} (byteStride={stride})",
            acc.count, pos_id, bv_idx
        );
        for i in 0..acc.count as usize {
            let start = base + i * stride;
            let end = start + element_size;
            if end > buf.len() {
                println!("  [{i:3}] OUT OF BOUNDS");
                continue;
            }
            let x = f32::from_le_bytes(buf[start..start + 4].try_into().unwrap());
            let y = f32::from_le_bytes(buf[start + 4..start + 8].try_into().unwrap());
            let z = f32::from_le_bytes(buf[start + 8..start + 12].try_into().unwrap());
            println!("  [{i:3}] ({x:.6}, {y:.6}, {z:.6})");
        }
    }
}

// -------------------------------------------------------------------------- //

fn print_summary(gltf: &Gltf) {
    let asset = &gltf.asset;
    println!(
        "glTF {}{}",
        asset.version,
        asset
            .min_version
            .as_deref()
            .map(|v| format!(" (min {v})"))
            .unwrap_or_default()
    );
    if let Some(generator) = &asset.generator {
        println!("generator: {generator}");
    }

    fn count<T>(opt: &Option<Vec<T>>) -> usize {
        opt.as_deref().map_or(0, |v| v.len())
    }

    println!("scenes:      {}", count(&gltf.scenes));
    println!("nodes:       {}", count(&gltf.nodes));
    println!("meshes:      {}", count(&gltf.meshes));
    println!("accessors:   {}", count(&gltf.accessors));
    println!("buffer views:{}", count(&gltf.buffer_views));
    println!("buffers:     {}", count(&gltf.buffers));
    println!("materials:   {}", count(&gltf.materials));
    println!("textures:    {}", count(&gltf.textures));
    println!("animations:  {}", count(&gltf.animations));
    println!("skins:       {}", count(&gltf.skins));
}

fn print_nodes(gltf: &Gltf) {
    let Some(nodes) = gltf.nodes.as_deref() else {
        println!("no nodes");
        return;
    };

    // Find root nodes from the default scene.
    let scene_idx = gltf.scene.unwrap_or(0) as usize;
    let root_nodes: &[u32] = gltf
        .scenes
        .as_deref()
        .and_then(|s| s.get(scene_idx))
        .and_then(|s| s.nodes.as_deref())
        .unwrap_or(&[]);

    for &root in root_nodes {
        print_node_tree(nodes, root as usize, 0);
    }
}

fn print_node_tree(nodes: &[Node], idx: usize, depth: usize) {
    let Some(node) = nodes.get(idx) else { return };
    let indent = "  ".repeat(depth);
    let name = node.name.as_deref().unwrap_or("<unnamed>");

    print!("{indent}node {idx}: {name}");
    if let Some(mesh) = node.mesh {
        print!("  [mesh {mesh}]");
    }
    println!();

    // Transform
    if let Some(m) = &node.matrix {
        println!("{indent}  matrix:");
        for row in 0..4 {
            // glTF matrix is column-major; print row-major for readability.
            println!(
                "{indent}    [{:8.4}  {:8.4}  {:8.4}  {:8.4}]",
                m[row],
                m[row + 4],
                m[row + 8],
                m[row + 12]
            );
        }
    } else {
        if let Some(t) = node.translation {
            println!(
                "{indent}  translation: [{:.4}, {:.4}, {:.4}]",
                t[0], t[1], t[2]
            );
        }
        if let Some(r) = node.rotation {
            println!(
                "{indent}  rotation:    [{:.4}, {:.4}, {:.4}, {:.4}]  (xyzw)",
                r[0], r[1], r[2], r[3]
            );
        }
        if let Some(s) = node.scale {
            println!(
                "{indent}  scale:       [{:.4}, {:.4}, {:.4}]",
                s[0], s[1], s[2]
            );
        }
        if node.translation.is_none() && node.rotation.is_none() && node.scale.is_none() {
            println!("{indent}  transform:   identity");
        }
    }

    for &child in node.children.as_deref().unwrap_or(&[]) {
        print_node_tree(nodes, child as usize, depth + 1);
    }
}

fn print_meshes(gltf: &Gltf) {
    let Some(meshes) = gltf.meshes.as_deref() else {
        println!("no meshes");
        return;
    };
    let accessors = gltf.accessors.as_deref().unwrap_or(&[]);

    for (mi, mesh) in meshes.iter().enumerate() {
        let name = mesh.name.as_deref().unwrap_or("<unnamed>");
        println!("mesh {mi}: {name}");

        for (pi, prim) in mesh.primitives.iter().enumerate() {
            let mode = mode_str(prim.mode);
            println!("  primitive {pi}  [{mode}]");

            if let Some(idx) = prim.indices {
                print!("    indices:  accessor {idx:<3}");
                if let Some(acc) = accessors.get(idx as usize) {
                    print!(
                        "  {:6}  {:13}  {}",
                        accessor_type_str(acc.accessor_type),
                        component_type_str(acc.component_type),
                        acc.count
                    );
                }
                println!();
            }

            let mut attrs: Vec<_> = prim.attributes.iter().collect();
            attrs.sort_by_key(|(k, _)| k.as_str());
            for (semantic, idx) in attrs {
                let idx = *idx;
                print!("    {semantic:<9} accessor {idx:<3}");
                if let Some(acc) = accessors.get(idx as usize) {
                    print!(
                        "  {:6}  {:13}  {}",
                        accessor_type_str(acc.accessor_type),
                        component_type_str(acc.component_type),
                        acc.count
                    );
                }
                println!();
            }
        }
    }
}

fn mode_str(mode: MeshPrimitiveMode) -> &'static str {
    use MeshPrimitiveMode::*;
    match mode {
        Points => "POINTS",
        Lines => "LINES",
        LineLoop => "LINE_LOOP",
        LineStrip => "LINE_STRIP",
        Triangles => "TRIANGLES",
        TriangleStrip => "TRIANGLE_STRIP",
        TriangleFan => "TRIANGLE_FAN",
    }
}

fn accessor_type_str(t: gltforge::schema::AccessorType) -> &'static str {
    use gltforge::schema::AccessorType::*;
    match t {
        Scalar => "SCALAR",
        Vec2 => "VEC2",
        Vec3 => "VEC3",
        Vec4 => "VEC4",
        Mat2 => "MAT2",
        Mat3 => "MAT3",
        Mat4 => "MAT4",
    }
}

fn component_type_str(t: gltforge::schema::AccessorComponentType) -> &'static str {
    use gltforge::schema::AccessorComponentType::*;
    match t {
        Byte => "BYTE",
        UnsignedByte => "UNSIGNED_BYTE",
        Short => "SHORT",
        UnsignedShort => "UNSIGNED_SHORT",
        UnsignedInt => "UNSIGNED_INT",
        Float => "FLOAT",
    }
}
