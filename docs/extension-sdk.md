# gltforge Extension SDK — Design Notes

## Philosophy

**Rust is the brain. Unity is dumb.**

The Rust pipeline owns all glTF knowledge — parsing, validation, coordinate conversion,
extension dispatch, and eventually serialization for export. Unity contributes raw data
(meshes, textures, materials) and receives raw data back in a shape that maps 1-to-1 with
Unity analogues. No C# glTF schema mirror is needed — the FFI surface is the contract.

Extensions follow the same rule: the extension Rust crate owns the glTF parsing and data
extraction. The extension C# package owns only the Unity wiring.

---

## Extension Manifest

Extensions are declared in `gltforge.toml`, which lives in the same directory as
`gltforge_unity.dll`. The runtime locates it by resolving the DLL's own path at runtime
(cross-platform: `GetModuleFileName` on Windows, `dladdr` on Unix/macOS) — no hardcoded
paths, no environment variables.

The manifest is **never read unless a loaded glTF declares at least one extension** in
`extensionsUsed`. Files with no extensions pay zero I/O cost.

```toml
[dependencies]
KHR_texture_transform      = { version = "1.0.0", dll = "gltforge_ext_khr_texture_transform", features = ["unity"] }
KHR_draco_mesh_compression = { version = "1.0.0", dll = "gltforge_ext_draco",                 features = ["unity"] }
STUDIO_custom_lod          = { version = "0.1.0", dll = "gltforge_ext_studio_lod",            features = ["unity", "blender"] }
```

- **Key** — the glTF extension name, matched exactly against `extensionsUsed`
- **`dll`** — filename (without extension) of the extension DLL, in the same directory
- **`version`** — semver; validated before the DLL is trusted. Mismatch = warn and skip, not crash.
- **`features`** — which platform helpers the extension supports, using Cargo feature semantics.
  The runtime only calls platform-specific hooks for features listed here.

### Properties

- **Explicit** — no scanning, no magic. Every loaded extension is declared.
- **Disable without deleting** — comment out a line to deactivate.
- **Auditable** — readable by anyone; a studio knows exactly what is loaded.
- **Source-controllable** — plain TOML, checked in alongside the DLLs.

---

## Crate Structure

```
gltforge                        ← core parser (no SDK dependency)
gltforge-sdk                    ← ExtensionHandler trait, ExtensionContext, ExtensionData,
                                   ImportLog, ExtensionRegistry (depends on gltforge for schema types)
gltforge-unity                  ← Unity FFI bridge (depends on gltforge + gltforge-sdk)
gltforge-ext-khr-texture-transform ← dogfood extension (depends on gltforge-sdk only)
```

`gltforge` core has **no dependency** on `gltforge-sdk` — the extension hook lives in
`gltforge-unity`, avoiding any circular dependency. `gltforge-sdk` can freely depend on
`gltforge` to reuse schema types (`AccessorComponentType`, `AccessorType`, etc.) without
redefining them.

Extension authors depend only on `gltforge-sdk`, never on the full parser:

```toml
[dependencies]
gltforge-sdk = { version = "0.0.5" }

[features]
unity   = ["gltforge-sdk/unity"]
blender = ["gltforge-sdk/blender"]
```

---

## SDK Core Types

### ExtensionContext

Exposes core glTF schema objects to handlers on demand. The handler pulls only what it needs.

```rust
pub struct ExtensionContext<'a> {
    pub fn get_accessor(&self, index: u32) -> Option<&Accessor>;
    pub fn get_buffer_view(&self, index: u32) -> Option<&BufferView>;
    pub fn get_buffer_view_bytes(&self, index: u32) -> Option<&[u8]>;
    pub fn get_texture(&self, index: u32) -> Option<&Texture>;
    pub fn get_material(&self, index: u32) -> Option<&Material>;
    // etc. — grows as extension needs are discovered
}
```

### ExtensionData

Marker trait for typed data stored on schema objects by extension handlers.

```rust
pub trait ExtensionData: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}
```

### Resolved Sidecar

Every schema object that can carry extensions gains a `resolved` map alongside the
existing raw `extensions` field. The SDK dispatcher populates it during load.

```rust
pub struct TextureInfo {
    pub index: GltfId,
    pub tex_coord: u32,
    pub extensions: Option<Extensions>,                       // raw JSON (unchanged)
    pub resolved: HashMap<String, Box<dyn ExtensionData>>,    // typed, populated by SDK
    pub extras: Option<Extras>,
}
```

### ExtensionHandler

One hook per glTF attachment point. All default to no-op — implement only the hooks
relevant to the extension.

```rust
pub trait ExtensionHandler: Send + Sync {
    fn name(&self) -> &str;

    fn on_texture_info(
        &self, value: &serde_json::Value,
        ctx: &ExtensionContext,
        log: &mut ImportLog,
    ) -> SdkResult<Option<Box<dyn ExtensionData>>> { Ok(None) }

    fn on_material(
        &self, value: &serde_json::Value,
        ctx: &ExtensionContext,
        log: &mut ImportLog,
    ) -> SdkResult<Option<Box<dyn ExtensionData>>> { Ok(None) }

    fn on_primitive(
        &self, value: &serde_json::Value,
        ctx: &ExtensionContext,
        log: &mut ImportLog,
    ) -> SdkResult<Option<Box<dyn ExtensionData>>> { Ok(None) }

    // on_node, on_mesh, on_image, on_accessor, on_root — added as needed
}
```

### ExtensionRegistry

Built lazily during `gltforge_load`, only when `extensionsUsed` is non-empty.

```rust
pub struct ExtensionRegistry {
    handlers: HashMap<String, Box<dyn ExtensionHandler>>,
}
```

### ImportLog

Travels through the entire pipeline — Rust core, SDK dispatcher, extension handlers.
Attached to `GltforgeAsset` at the end of import. Opt-in per asset via a
`[SerializeField] bool _enableImportLog` on `GltforgeImporter` (defaults to `true`).

```rust
pub struct ImportLog {
    pub entries: Vec<LogEntry>,
}

pub struct LogEntry {
    pub level: LogLevel,
    pub source: String,    // e.g. "KHR_texture_transform" or "gltforge-unity"
    pub message: String,
}

pub enum LogLevel { Info, Warning, Error }
```

On the C# side, `GltforgeAsset` carries the log as a serialized field — inspectable
at any time in the Unity Editor, not just during import.

---

## Load Pipeline

```
gltforge_load(path, enable_log) called
  │
  ├─ parse JSON → Gltf schema
  │
  ├─ extensions_used non-empty?
  │     YES → resolve DLL dir → read gltforge.toml → load extension DLLs
  │           → build ExtensionRegistry
  │     NO  → empty registry (zero I/O, zero overhead)
  │
  ├─ load_buffers
  │
  ├─ SDK dispatcher walks schema:
  │     for each object with extensions:
  │       call registered handler hook → store result in object.resolved
  │
  └─ build_unity_gltf (existing pipeline, unchanged)
        → materials, meshes, textures processed as today
        → resolved sidecars ride along inside UnityGltf
```

---

## Unity Import — Post-Process Event System

After `GltforgeImporter` finishes building a Unity `Material`, it iterates the resolved
extension cache and fires C# events. Extension C# packages register as listeners via
`[InitializeOnLoad]` — no discovery mechanism needed, Unity handles it automatically.

```csharp
// GltforgeImporter fires after each material is built:
OnTextureInfo?.Invoke(new TextureInfoEventArgs {
    material       = unityMaterial,
    textureIndex   = texInfoIndex,
    extensionData  = resolvedSidecar   // pre-extracted by Rust, 1-to-1 ready
});
```

The event args carry 1-to-1 data already extracted from the Rust sidecar — no FFI
calls needed from the handler side.

```csharp
// In the extension's C# package:
[InitializeOnLoad]
static class TextureTransformHelper {
    static TextureTransformHelper() {
        GltforgeImporter.OnTextureInfo += Apply;
    }

    static void Apply(TextureInfoEventArgs args) {
        if (!args.TryGetExtension("KHR_texture_transform", out TextureTransformData t))
            return;
        args.material.SetTextureOffset("_BaseMap", t.offset);
        args.material.SetTextureScale("_BaseMap",  t.scale);
        // rotation applied via shader property
    }
}
```

`GltforgeImporter` never names any specific extension. New extensions drop in a C#
package that self-registers — zero changes to the importer.

---

## Shader Requirements

Extensions that affect how geometry or textures are sampled on the GPU must ship their
own Shader Graph assets alongside the Rust DLL and C# package. The Shader Graph is
part of the extension's deliverable.

Example: `KHR_texture_transform` requires a shader with UV transform logic
(offset, rotation, scale applied before texture sample) and matching exposed properties.
The extension C# handler sets those properties on the material; the shader does the math.

---

## Compatibility Handshake

Performed as the first step of `OnImportAsset`, before touching any glTF data:

```csharp
var diag = GltforgeNative.gltforge_check_compatibility(absolutePath, manifest);

if (diag.hasErrors) {
    foreach (var error in diag.errors)
        ctx.LogImportError($"[gltforge] {error}");
    return;
}
```

Rust reads `extensionsRequired`, cross-references the TOML registry, and produces
specific, actionable diagnostics:

```
[gltforge] ERROR:   Asset requires 'KHR_draco_mesh_compression' but no handler is registered
[gltforge] WARNING: Asset uses 'EXT_meshopt_compression' — no handler registered, geometry may be missing
```

---

## Dogfooding Target

`KHR_texture_transform` is the first implementation target for the extension SDK.

It was chosen over `KHR_draco_mesh_compression` because:
- It is a post-parse extension — no pre-accessor interception complexity
- It validates the resolved sidecar pattern and the C# event system end-to-end
- It has immediate practical value (texture atlasing, tiling overrides)
- It exercises the shader requirement pattern (extension ships its own Shader Graph)
- It is entirely self-contained: Rust handler + C# event listener + Shader Graph

`KHR_draco_mesh_compression` remains a future target once the SDK surface is proven.

---

## Future Direction

- `KHR_draco_mesh_compression` — pre-accessor interception (hardest case)
- `EXT_meshopt_compression` — buffer-view level decompression
- glTF export path — Unity raw data → Rust FFI → glTF serialization.
  The same "Rust is the brain" principle applies in reverse: Unity passes raw mesh/material
  data, Rust handles buffer packing, accessor construction, and JSON generation.
- `gltforge-blender` — Blender addon following the same architecture
- crates.io publishing for extension crates, with manifest-driven resolution
