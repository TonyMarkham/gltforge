# gltforge

> Spec-compliant, extension-first glTF 2.0 for Rust

`gltforge` is a glTF 2.0 library built around a single principle: extensions are first-class citizens, not afterthoughts. Every intermediate stage of parsing, validation, and processing is owned by the caller — nothing is hidden.

## Why gltforge?

Existing glTF libraries treat custom extensions as a bolted-on concern. `gltforge` is designed from the ground up so that extension authors have full typed access to the document, buffer views, accessors, and validation pipeline — at every stage.

- **Extension-first** — typed extension registration, import and export treated equally
- **Open by design** — every intermediate representation is public and caller-owned
- **Async from scratch** — progressive loading via streams, shared caches, structured cancellation
- **Cross-platform** — native dylib (Windows, macOS, Linux, iOS, Android), WASM, PyO3 (Blender)
- **Spec-compliant** — `extensionsRequired` enforced as hard errors, not warnings

## Status

Early development. Not yet published to crates.io.

## Workspace

```
gltforge/          # core library
gltforge-unity/    # #[repr(C)] P/Invoke bindings for Unity
gltforge-python/   # PyO3 bindings (Blender addon)
gltforge-wasm/     # wasm-bindgen bindings
gltforge-cli/      # command line tooling
```

## Usage

```toml
[dependencies]
gltforge = "0.1"
```

```rust
use gltforge::{GltfPipeline, ExtensionRegistry};

let registry = ExtensionRegistry::new()
    .register::<MyCustomExtension>();

let pipeline = GltfPipeline::new(registry);
let mut stream = pipeline.load_stream("model.glb").await?;

while let Some(event) = stream.next().await {
    match event {
        GltfLoadEvent::MeshProcessed { index, name } => { /* hand off to renderer */ }
        GltfLoadEvent::TextureDecoded { index, .. } => { /* upload to GPU */ }
        GltfLoadEvent::SceneReady(scene) => { /* final assembly */ }
        GltfLoadEvent::Warning(w) => eprintln!("warn: {w}"),
    }
}
```

## Custom Extensions

```rust
pub struct MyExtension {
    pub custom_value: f32,
}

impl GltfExtension for MyExtension {
    const NAME: &'static str = "MY_custom_extension";

    fn deserialize(
        value: serde_json::Value,
        ctx: &ExtensionContext,
    ) -> Result<Self, ExtensionError> {
        Ok(Self {
            custom_value: value["customValue"].as_f64().unwrap_or(0.0) as f32,
        })
    }

    fn serialize(&self) -> Result<serde_json::Value, ExtensionError> {
        Ok(serde_json::json!({ "customValue": self.custom_value }))
    }

    fn validate(&self, ctx: &ValidationContext) -> Result<(), ExtensionError> {
        if self.custom_value < 0.0 {
            return Err(ExtensionError::invalid("customValue must be non-negative"));
        }
        Ok(())
    }
}

// Access your extension data after load — it's always there
let ext = node.extensions.get::<MyExtension>();
```

## Platform Bindings

### Unity (C#)

```csharp
[DllImport("gltforge", CallingConvention = CallingConvention.Cdecl)]
private static extern GltfError ProcessGltf(byte[] data, uint length, out GltfResult result);

[DllImport("gltforge", CallingConvention = CallingConvention.Cdecl)]
private static extern void FreeGltfResult(ref GltfResult result);
```

### Python / Blender

```python
import gltforge

registry = gltforge.ExtensionRegistry()
registry.register(MyExtension())

doc = gltforge.Document.from_path("model.glb", registry)
```

## Roadmap

- [ ] Core JSON parse + validation
- [ ] GLB binary chunk handling
- [ ] Accessor + bufferView resolution
- [ ] Mesh processing (tangents, meshopt, quantization)
- [ ] Skinning + animation baking
- [ ] Extension registry + typed dispatch
- [ ] `KHR_draco_mesh_compression`
- [ ] `EXT_meshopt_compression`
- [ ] `KHR_texture_basisu`
- [ ] Unity P/Invoke bindings
- [ ] PyO3 / Blender addon
- [ ] WASM bindings
- [ ] Protobuf disk cache layer

## License

MIT — see [LICENSE](LICENSE)