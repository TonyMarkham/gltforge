use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use gltforge::parser;
use gltforge::schema::{Gltf, MeshPrimitiveMode};

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

        /// Print mesh and primitive details.
        #[arg(long)]
        mesh: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Inspect { path, mesh } => match inspect(&path, mesh) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
    }
}

fn inspect(path: &PathBuf, mesh: bool) -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let gltf = parser::parse(&json)?;

    print_summary(&gltf);

    if mesh {
        println!();
        print_meshes(&gltf);
    }

    Ok(())
}

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
