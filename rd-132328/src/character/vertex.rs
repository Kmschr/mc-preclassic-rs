use super::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub u: f32,
    pub v: f32,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
        Vertex {
            pos: Vec3::new(x, y, z),
            u,
            v,
        }
    }

    pub fn from_vertex(vertex: &Vertex, u: f32, v: f32) -> Vertex {
        Vertex {
            pos: vertex.pos,
            u,
            v,
        }
    }

    pub fn remap(&self, u: f32, v: f32) -> Vertex {
        Vertex::from_vertex(self, u, v)
    }
}
