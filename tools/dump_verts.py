"""
Blender script: dump vertex positions for a named mesh.

Usage (Blender Scripting tab):
  1. Import BoxAnimated.gltf via File > Import > glTF 2.0
  2. Open this script in the Text Editor
  3. Set MESH_NAME below if needed
  4. Run Script
  5. Check the System Console (Window > Toggle System Console) for output
"""

import bpy

MESH_NAME = "inner_box"

obj = bpy.data.objects.get(MESH_NAME)
if obj is None:
    # Fall back: find any object whose mesh is named MESH_NAME
    for o in bpy.data.objects:
        if o.type == 'MESH' and o.data.name == MESH_NAME:
            obj = o
            break

if obj is None:
    print(f"[dump_verts] ERROR: no object or mesh named '{MESH_NAME}' found.")
    print("[dump_verts] Available meshes:", [o.data.name for o in bpy.data.objects if o.type == 'MESH'])
else:
    mesh = obj.data
    print(f"[dump_verts] '{MESH_NAME}': {len(mesh.vertices)} verts, {len(mesh.polygons)} polys")
    print("[dump_verts] Vertex positions (object-local space):")
    for i, v in enumerate(mesh.vertices):
        x, y, z = v.co
        print(f"  [{i:3d}] ({x:.6f}, {y:.6f}, {z:.6f})")
    print(f"[dump_verts] done.")
