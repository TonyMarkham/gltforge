# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo check                        # type-check without building
cargo build                        # debug build
cargo build --release              # release build
cargo test                         # run all tests
cargo test <test_name>             # run a single test
cargo clippy                       # lint
```

All dependencies are pinned at the workspace level in the root `Cargo.toml`. Add new deps there under `[workspace.dependencies]` and reference them with `{ workspace = true }` in crate manifests.

## Architecture

`gltforge` is a glTF 2.0 library where extensions are first-class citizens. The planned pipeline is: streaming parse → typed schema → validation → accessor/bufferView resolution → extension dispatch. Only the schema layer exists so far.

### Module layout

```
crates/gltforge/src/
├── lib.rs              — pub mod error; pub mod schema;
├── error/
│   ├── mod.rs          — re-exports SchemaError, SchemaResult
│   └── schema/mod.rs   — SchemaError enum (thiserror), SchemaResult<T> type alias
└── schema/
    ├── mod.rs          — declares all sub-modules; flat re-export surface for every public type
    └── <type>/         — one directory (or file) per glTF schema object
```

### Schema conventions

Each glTF schema object lives in its own file or sub-module directory. Sub-types (enums, nested structs) get their own files within that directory and are re-exported through the parent `mod.rs`. Every public type is then re-exported again from `schema/mod.rs` so callers only ever need `use gltforge::schema::Foo`.

Naming convention: types are prefixed with their domain to stay unambiguous at the flat re-export level — `AnimationSampler`, `MeshPrimitive`, `SamplerMagFilter`, `MaterialAlphaMode`, etc.

Integer-valued enums (component types, filter modes, etc.) implement `TryFrom<u32>` / `From<u32>` with `#[serde(try_from = "u32", into = "u32")]`. The `TryFrom` impl is annotated `#[track_caller]` and returns `SchemaResult<Self>` with `SchemaError::InvalidValue { type_name, value, location }` on failure, where `location` is captured via `error_location::ErrorLocation::from(Location::caller())`.

String-valued enums use `#[serde(rename_all = "...")]` matching the glTF spec casing (e.g. `SCREAMING_SNAKE_CASE` for `OPAQUE`/`MASK`/`BLEND`, `lowercase` for camera types and animation paths).

Optional fields use `#[serde(skip_serializing_if = "Option::is_none")]`. Fields with spec-defined defaults use `#[serde(default)]` or `#[serde(default = "fn_name")]`.

### Error handling

`crate::error::SchemaError` — errors originating from schema parsing/conversion.
`crate::error::SchemaResult<T>` — `std::result::Result<T, SchemaError>`.
Both are re-exported from `crate::error` directly.

### Planned crates (not yet added to workspace)

- `gltforge-unity` — `#[repr(C)]` P/Invoke bindings
- `gltforge-python` — PyO3 / Blender addon
- `gltforge-wasm` — wasm-bindgen bindings
- `gltforge-cli` — command-line tooling
