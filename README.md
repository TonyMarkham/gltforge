# gltforge

> Spec-compliant, extension-first glTF 2.0 for Rust

`gltforge` is a glTF 2.0 toolkit built around a single principle: extensions are first-class citizens, not afterthoughts. Every intermediate stage of parsing, validation, and processing is owned by the caller — nothing is hidden.

- **Extension-first** — typed extension registration, import and export treated equally
- **Open by design** — every intermediate representation is public and caller-owned
- **Cross-platform** — native dylib (Windows, macOS, Linux, iOS, Android), WASM, PyO3 (Blender)
- **Spec-compliant** — `extensionsRequired` enforced as hard errors, not warnings

## Workspace

| Crate | Description | Status |
|---|---|---|
| `gltforge` | Core parser and schema types | Active development |
| `gltforge-cli` | `gltforge inspect` command-line tool | Active development |
| `gltforge-unity` | `#[repr(C)]` P/Invoke bindings for Unity | Active development |
| `gltforge-python` | PyO3 bindings (Blender addon) | Planned |
| `gltforge-wasm` | wasm-bindgen bindings | Planned |

## Usage

```toml
[dependencies]
gltforge = "0.0.4"
```

```rust
use std::path::Path;
use gltforge::parser;
use gltforge::schema::Gltf;

// Parse a .gltf file
let json = std::fs::read_to_string("model.gltf")?;
let gltf: Gltf = parser::parse(&json)?;

// Load external binary buffers
let buffers = parser::load_buffers(&gltf, Path::new("."))?;

// Resolve an accessor to raw bytes
let accessors = gltf.accessors.as_deref().unwrap_or(&[]);
let buffer_views = gltf.buffer_views.as_deref().unwrap_or(&[]);
let bytes = parser::resolve_accessor(&accessors[0], buffer_views, &buffers)?;
```

## CLI

```bash
cargo install gltforge-cli

# Summary
gltforge inspect model.gltf

# With node hierarchy and mesh details
gltforge inspect model.gltf --nodes --mesh

# Dump raw POSITION vertices for mesh 0 (glTF coordinate space)
gltforge inspect model.gltf --dump-verts 0
```

```
glTF 2.0
generator: Khronos Blender glTF 2.0 exporter
scenes:      1
nodes:       22
meshes:      1
accessors:   83
buffer views:8
buffers:     1
materials:   1
textures:    1
animations:  1
skins:       1

node 0: Z_UP
  matrix:
    [  1.0000    0.0000    0.0000    0.0000]
    [  0.0000    0.0000    1.0000    0.0000]
    [  0.0000   -1.0000    0.0000    0.0000]
    [  0.0000    0.0000    0.0000    1.0000]
  node 1: Armature
    node 3: Skeleton_torso_joint_1
      translation: [0.0000, 0.0050, 0.6790]
      rotation:    [0.0000, -0.0378, 0.0000, -0.9993]  (xyzw)
      scale:       [1.0000, 1.0000, 1.0000]

mesh 0: Cesium_Man
  primitive 0  [TRIANGLES]
    indices:  accessor 0    SCALAR  UNSIGNED_SHORT  14016
    JOINTS_0  accessor 1    VEC4    UNSIGNED_SHORT  3273
    NORMAL    accessor 2    VEC3    FLOAT           3273
    POSITION  accessor 3    VEC3    FLOAT           3273
    TEXCOORD_0 accessor 4   VEC2    FLOAT           3273
    WEIGHTS_0 accessor 5    VEC4    FLOAT           3273
```

## Unity

`gltforge-unity` compiles to a native plugin (`gltforge_unity.dll`) and exposes a P/Invoke API. A `ScriptedImporter` lets you drag `.gltf` files directly into the Unity project panel.

The importer builds the full scene graph as a prefab hierarchy, with mesh geometry and transforms ready to use. Coordinates are converted from glTF's right-handed system to Unity's left-handed system (X negated, winding reversed). Index format (`UInt16`/`UInt32`) is selected automatically based on vertex count.

**What's imported today:**

| Data | Support |
|---|---|
| Scene graph (nodes, hierarchy) | ✅ |
| Node transforms (TRS and matrix) | ✅ |
| Meshes (positions, normals, tangents, UVs) | ✅ |
| Sub-meshes (one per glTF primitive) | ✅ |
| Materials / textures | Planned |
| Skinning / animations | Planned |

## Roadmap

- [x] glTF 2.0 schema types
- [x] JSON parser + buffer loading
- [x] Accessor resolution
- [x] CLI inspector (`--nodes`, `--mesh`, `--dump-verts`)
- [x] Unity P/Invoke bindings — scene graph, node transforms, meshes, normals, tangents, UVs, sub-meshes
- [x] Coordinate system conversion (glTF right-handed → Unity left-handed)
- [ ] GLB binary chunk handling
- [ ] Materials and textures
- [ ] Skinning (`JOINTS_0` / `WEIGHTS_0`) and animations
- [ ] Extension registry + typed dispatch
- [ ] `KHR_draco_mesh_compression`
- [ ] `EXT_meshopt_compression`
- [ ] PyO3 / Blender addon
- [ ] WASM bindings

## License

MIT — see [LICENSE](LICENSE)
