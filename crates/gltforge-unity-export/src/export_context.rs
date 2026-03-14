use crate::{mesh_data::MeshData, node_data::NodeData, submesh_data::SubmeshData};

/// Accumulates Unity-shaped scene data pushed by C# before the build step writes the glTF files.
pub struct ExportContext {
    pub nodes: Vec<NodeData>,
    pub meshes: Vec<MeshData>,
}

impl Default for ExportContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ExportContext {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            meshes: Vec::new(),
        }
    }

    pub fn add_node(
        &mut self,
        name: Option<String>,
        parent: Option<u32>,
        translation: Option<[f32; 3]>,
        rotation: Option<[f32; 4]>,
        scale: Option<[f32; 3]>,
    ) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(NodeData {
            name,
            parent,
            mesh_index: None,
            translation,
            rotation,
            scale,
        });
        idx
    }

    pub fn add_mesh(&mut self, name: Option<String>) -> u32 {
        let idx = self.meshes.len() as u32;
        self.meshes.push(MeshData {
            name,
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            submeshes: Vec::new(),
        });
        idx
    }

    pub fn set_positions(&mut self, mesh_idx: u32, positions: Vec<[f32; 3]>) {
        if let Some(mesh) = self.meshes.get_mut(mesh_idx as usize) {
            mesh.positions = positions;
        }
    }

    pub fn set_normals(&mut self, mesh_idx: u32, normals: Vec<[f32; 3]>) {
        if let Some(mesh) = self.meshes.get_mut(mesh_idx as usize) {
            mesh.normals = normals;
        }
    }

    pub fn set_uvs(&mut self, mesh_idx: u32, channel: u32, uvs: Vec<[f32; 2]>) {
        if let Some(mesh) = self.meshes.get_mut(mesh_idx as usize) {
            let ch = channel as usize;
            if mesh.uvs.len() <= ch {
                mesh.uvs.resize(ch + 1, Vec::new());
            }
            mesh.uvs[ch] = uvs;
        }
    }

    pub fn add_submesh(&mut self, mesh_idx: u32, indices: Vec<u32>) {
        if let Some(mesh) = self.meshes.get_mut(mesh_idx as usize) {
            mesh.submeshes.push(SubmeshData { indices });
        }
    }

    pub fn set_node_mesh(&mut self, node_idx: u32, mesh_idx: u32) {
        if let Some(node) = self.nodes.get_mut(node_idx as usize) {
            node.mesh_index = Some(mesh_idx);
        }
    }
}
