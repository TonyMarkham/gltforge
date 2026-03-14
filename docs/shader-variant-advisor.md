# gltforge Shader Variant Advisor — Design Notes

## Problem

Unity's built-in Shader Variant Collection tool is manual and disconnected from project
data. As glTF material extensions multiply, the keyword combinations grow combinatorially.
Maintaining the `.shadervariants` file by hand is error-prone and gives no signal about
which combinations are actually needed.

## Approach

A gltforge-aware Editor tool that reads the `GltforgeAsset` ScriptableObjects already
present in the project, determines which keyword combinations are actually in use, and
suggests additions to the `.shadervariants` file.

Since `GltforgeAsset` carries resolved extension and material data, the tool has exact
knowledge of which combinations exist across the project — no shader source scanning,
no guessing.

## Behaviour

- **Suggests, does not write** — the user reviews and applies. The tool presents a diff
  between the current `.shadervariants` contents and what it recommends.
- **Project-scoped** — scans all `GltforgeAsset` SOs in the project via `AssetDatabase`.
- **Extension-aware** — as new extension handlers are added, the tool automatically
  accounts for their keywords since they surface through the same `GltforgeAsset` data.

## UI

Unity `EditorWindow`, accessible via **gltforge → Shader Variant Advisor** in the
Unity menu bar.

Displays:
- Currently covered variants (already in `.shadervariants`)
- Recommended additions (in use by project assets but not yet covered)
- Variants in the file but unused by any current asset (candidates for removal)

## WebGL

WebGL builds cannot compile shaders at runtime — all required variants must be included
upfront in the `.shadervariants` file. A missing variant produces an error shader
(pink/magenta material) with no fallback and no useful diagnostic.

Unity's aggressive WebGL stripping commonly drops Ambient Occlusion variants unless
explicitly included. Unlike a missing variant (which produces an error shader), dropped
AO produces silent visual degradation — materials still render but occluded areas are
too bright, with no warning and no obvious signal that AO has been lost.

The advisor should flag AO variants as high-priority inclusions when the build target
is WebGL, precisely because the failure mode is invisible.

The CI headless mode (see below) is especially valuable for WebGL pipelines — silent
visual regressions are far harder to catch than hard errors.

## Future Direction

- Per-asset variant preview — select a `GltforgeAsset`, see exactly which variants it
  requires
- CI integration — a headless mode that fails the build if required variants are missing
  from the collection, preventing silent fallback to error-pink materials in builds
