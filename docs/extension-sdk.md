# gltforge Extension SDK — Design Notes

## Overview

The extension system allows third-party Rust crates to handle glTF custom extensions
as separate DLLs, loaded and dispatched by the gltforge core at runtime. The core
stays small and stable; extension support is purely additive.

## Extension Manifest

Extensions are registered in a `gltforge.toml` file that sits beside `gltforge_unity.dll`
in the Unity `Plugins` folder. The syntax mirrors `Cargo.toml` dependency declarations —
familiar to any Rust developer, human-readable, and source-controllable.

```toml
[dependencies]
KHR_draco_mesh_compression = { version = "1.0.0", dll = "gltforge_ext_draco" }
EXT_meshopt_compression    = { version = "0.2.1", dll = "gltforge_ext_meshopt" }
STUDIO_custom_lod          = { version = "0.1.0", dll = "gltforge_ext_studio_lod" }
```

- **Key** — the glTF extension name, matched exactly against `extensionsUsed` in the asset
- **`dll`** — the DLL filename (without extension) to load from the same directory
- **`version`** — semver version of the extension handler; the core validates compatibility
  before trusting the registration entry point. Mismatch = warn and skip, not crash.

## Properties

- **Explicit** — no filesystem glob scanning, no magic. Every loaded extension is declared.
- **Disable without deleting** — comment out a line to deactivate an extension.
- **Auditable** — a studio can read the file and know exactly what is loaded.
- **Unity-friendly** — lives in the `Plugins` folder alongside the DLLs, no registry,
  no install step.

## Handler Design — Pull on Demand

The core design principle is **inversion of control**: the plugin drives, the core serves.

The core passes the raw extension JSON to the plugin and exposes an `ExtensionContext`
that the plugin uses to request exactly the data it needs. The core never pre-fetches
or pushes data the plugin didn't ask for.

```rust
trait ExtensionHandler {
    fn handle(&self, extension_json: &str, ctx: &ExtensionContext) -> ExtensionResult;
}

// The plugin pulls only what it needs
impl ExtensionContext {
    fn get_buffer_view(&self, index: u32) -> &[u8];
    fn get_accessor(&self, index: u32) -> AccessorData;
    fn get_image_bytes(&self, index: u32) -> &[u8];
    // etc.
}
```

For example, a Draco handler would:
1. Receive the extension JSON (which contains the `bufferView` index)
2. Parse that JSON itself to extract the index
3. Call `ctx.get_buffer_view(index)` to get the compressed bytes
4. Decompress and return resolved accessor data

**Why this matters:**
- The plugin only pulls what it actually needs — no over-fetching
- The core doesn't need to know what any extension requires upfront
- `ExtensionContext` is the stable ABI surface, not the data shape of each extension
- Works uniformly for both simple post-parse extensions and complex pre-accessor ones

## Handler Lifecycle

Extensions fall into two categories based on when they must run:

### Pre-accessor extensions
Must intercept *before* accessor resolution because the compressed/encoded data IS the
buffer (e.g. `KHR_draco_mesh_compression`, `EXT_meshopt_compression`). The plugin
receives the extension JSON, extracts the relevant buffer view index, and pulls the
raw bytes via `ExtensionContext`. They cannot be handled in C# after the fact —
they require a Rust handler.

### Post-parse extensions
Operate on the parsed glTF schema after accessor resolution. Unknown/unregistered
extensions fall through to the raw JSON passthrough and are surfaced to the C# consumer
via the SO cache.

## C# Consumption

Raw extension JSON for unhandled or passthrough extensions is surfaced through the
`GltforgeAsset` SO cache alongside typed Unity objects. C# extension handlers register
against extension names and receive the raw JSON for their extension on each relevant
object (material, mesh, node, etc.).

## Crate Structure

A dedicated `gltforge-sdk` crate keeps the trait definitions and shared types out of
production surfaces. Extension authors depend only on the SDK, never on the full parser:

```toml
# An extension author's Cargo.toml
[dependencies]
gltforge-sdk = { version = "0.1.0" }  # trait defs, ExtensionContext, registration macro

[dev-dependencies]
gltforge = { version = "0.0.5" }      # only needed for integration testing
```

Workspace layout:
```
gltforge-sdk       ← ExtensionHandler trait, ExtensionContext, shared types, registration macro
gltforge           ← core parser (depends on sdk for trait bounds)
gltforge-unity     ← Unity FFI (depends on sdk)
gltforge-ext-draco ← dogfood extension (depends on sdk only)
```

The `gltforge-sdk` crate is a dev-level dependency for the core but a production
dependency for extension authors — clean separation, minimal surface.

## Full Manifest Schema

The TOML manifest is not just an extension registry — it is a full pipeline configuration
that declares shader targets, Unity package dependencies, and version constraints:

```toml
[dependencies]
KHR_draco_mesh_compression = {
    version = "1.0.0",
    dll     = "gltforge_ext_draco"
}

STUDIO_custom_material = {
    version         = "1.0.0",
    dll             = "gltforge_ext_studio_mat",
    target          = "Shader Graphs/Studio Custom PBR",
    blend_target    = "Shader Graphs/Studio Custom PBR Blend",
    unity_packages  = [
        "com.unity.render-pipelines.universal@14.0.0",
        "com.studio.custom-shaders@2.1.0"
    ]
}
```

- **`target` / `blend_target`** — Unity shader to route the extension's material output to.
  The Rust extension produces a well-defined material data structure; the TOML wires it
  to the correct shader. The extension DLL never needs to know Unity shader names.
- **`unity_packages`** — required Unity packages with minimum versions. Validated at
  import time before any DLL loads. Fail-fast with a clear diagnostic rather than a
  silent magenta material.

Studios ship an extension DLL alongside a TOML snippet documenting configuration.
Drop both into the Plugins folder — no C# code required for material routing.

## Compatibility Handshake

The ScriptedImporter performs a fail-fast compatibility check as the very first step
of `OnImportAsset`, before touching any glTF data:

```csharp
public override void OnImportAsset(AssetImportContext ctx)
{
    var manifest  = ReadUnityPackageManifest(); // Packages/manifest.json
    var diag      = GltforgeNative.gltforge_check_compatibility(absolutePath, manifest);

    if (diag.hasErrors)
    {
        foreach (var error in diag.errors)
            ctx.LogImportError($"[gltforge] {error}");
        return;
    }

    foreach (var warning in diag.warnings)
        ctx.LogImportWarning($"[gltforge] {warning}");

    IntPtr handle = GltforgeNative.gltforge_load(absolutePath);
    // ...
}
```

The Rust core reads the glTF's `extensionsRequired`, cross-references against the TOML
registry, and validates each extension's `unity_packages` against the passed manifest.
Diagnostics are specific and actionable:

```
[gltforge] ERROR: Asset requires 'KHR_draco_mesh_compression' but no handler is registered in gltforge.toml
[gltforge] ERROR: STUDIO_custom_material requires 'com.studio.custom-shaders@2.1.0' — found 2.0.3
[gltforge] WARNING: Asset uses 'EXT_meshopt_compression' which has no handler — geometry may be missing
```

This replaces the current glTF ecosystem experience of a `NullReferenceException` deep
in import code with no actionable context.

## C# Prototype → Rust Production Workflow

C# developers can prototype extension logic against the existing FFI surface and SO
cache without any Rust toolchain. The C# implementation proves correctness and acts
as a living specification. A Rust developer then reimplements it as a native plugin
for production performance.

- The C# implementation becomes the integration test for the Rust plugin
- Both can run simultaneously — C# for validation, Rust for production
- Studios without Rust developers can ship C# extensions and port later if performance demands it
- Rust developers are not gatekeeping extension authorship

## SO Cache and the GltforgeAsset

The `GltforgeAsset` ScriptableObject is a glTF-shaped reference in Unity — not a cache
of the entire glTF, just a lightweight index mapping glTF structure to Unity objects.
Existing entries are enriched with their glTF origin indices rather than adding separate
accessor/bufferView tables:

```csharp
public class MeshEntry
{
    public int  meshIndex;
    public Mesh mesh;

    // glTF origin breadcrumbs
    public int   positionAccessorIndex;
    public int   normalAccessorIndex;
    public int   tangentAccessorIndex;
    public int[] texcoordAccessorIndices;
    public int[] submeshIndexAccessorIndices;
}
```

Benefits:
- **Debugging** — trace any Unity object back to the exact glTF spec element that produced it
- **C# extension handlers** — look up accessor indices, call FFI for raw bytes, no separate tables
- **glTF literacy** — junior developers can cross-reference Unity objects against the glTF JSON
- **Lean** — accessor indices are just `int` fields, no data duplication

SO creation is toggleable via the ScriptedImporter — disable for production import
pipelines where only the Rust extension SDK is needed and the managed overhead is waste.

## Future Direction

The Cargo.toml-style manifest opens the door to a future where extension crates are
published to crates.io and a toolchain resolves, compiles, and packages them
automatically — analogous to how cargo resolves dependencies today.

## Dogfooding Target

`KHR_draco_mesh_compression` is the intended first implementation target for the
extension SDK. It exercises the hardest case (pre-accessor interception) and is
genuinely useful, making it a real test rather than a toy example. If the hook
architecture handles Draco cleanly, the C# JSON passthrough case is trivial by
comparison.
