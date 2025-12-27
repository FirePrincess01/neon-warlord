//! Combines different geometries into one

type Vec3 = cgmath::Vector3<f32>;

#[allow(unused)]
pub trait MeshInterface {
    fn vertices(&self) -> &[Vec3];
    fn normals(&self) -> &[Vec3];
    fn indices(&self) -> &[u16];
}

#[allow(unused)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<u16>,
}

#[allow(unused)]
impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn add(&mut self, other: &impl MeshInterface) {
        let indices_base = self.vertices.len() as u16;

        for elem in other.vertices() {
            self.vertices.push(*elem);
        }

        for elem in other.normals() {
            self.normals.push(*elem);
        }

        for elem in other.indices() {
            self.indices.push(indices_base + *elem);
        }
    }
}
