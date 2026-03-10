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
| `gltforge` | Core parser and schema types | Early development |
| `gltforge-cli` | `gltforge inspect` command-line tool | Early development |
| `gltforge-unity` | `#[repr(C)]` P/Invoke bindings for Unity | Early development |
| `gltforge-python` | PyO3 bindings (Blender addon) | Planned |
| `gltforge-wasm` | wasm-bindgen bindings | Planned |

## Usage

```toml
[dependencies]
gltforge = "0.0.3"
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

gltforge inspect model.gltf
gltforge inspect model.gltf --mesh
```

```
glTF 2.0
scenes:      1
nodes:       1
meshes:      1
accessors:   2
buffer views:2
buffers:     1

mesh 0: Triangle
  primitive 0  [TRIANGLES]
    indices:  accessor 0   SCALAR  UNSIGNED_SHORT  3
    POSITION  accessor 1   VEC3    FLOAT           3
```

## Unity

`gltforge-unity` compiles to a native plugin (`gltforge_unity.dll`) and exposes a P/Invoke API. Drop the DLL into `Assets/Plugins/x86_64/` and import meshes at runtime:

```csharp
var handle = GltforgeNative.gltforge_load_mesh(path, nodeIndex);

uint posLen;
float[] pos = new float[posLen];
Marshal.Copy(GltforgeNative.gltforge_mesh_positions(handle, out posLen), pos, 0, (int)posLen);

uint submeshCount = GltforgeNative.gltforge_mesh_submesh_count(handle);
// ... SetTriangles per submesh

GltforgeNative.gltforge_mesh_release(handle);
```

Coordinates are converted from glTF's right-handed system to Unity's left-handed system (X negated, winding reversed). Index format (`UInt16`/`UInt32`) is selected automatically based on vertex count.

## Roadmap

- [x] glTF 2.0 schema types
- [x] JSON parser + buffer loading
- [x] Accessor resolution
- [x] CLI inspector
- [x] Unity P/Invoke bindings (positions, indices, submeshes, names)
- [ ] Node transforms (translation, rotation, scale)
- [ ] Normals, UVs, tangents
- [ ] Scene graph traversal
- [ ] GLB binary chunk handling
- [ ] Extension registry + typed dispatch
- [ ] `KHR_draco_mesh_compression`
- [ ] `EXT_meshopt_compression`
- [ ] PyO3 / Blender addon
- [ ] WASM bindings

## License

MIT — see [LICENSE](LICENSE)
