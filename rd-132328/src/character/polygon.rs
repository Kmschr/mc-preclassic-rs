use lwrgl::glu_sys::{glColor3f, glTexCoord2f, glVertex3f};

use super::vertex::Vertex;

pub struct Polygon {
    pub vertices: Vec<Vertex>,
    pub vertex_count: i32,
}

impl Polygon {
    pub fn new(vertices: Vec<Vertex>) -> Polygon {
        let vertex_count = vertices.len() as i32;
        Polygon {
            vertices,
            vertex_count,
        }
    }

    pub fn from_uvs(vertices: Vec<Vertex>, u0: i32, v0: i32, u1: i32, v1: i32) -> Polygon {
        let mut p = Polygon::new(vertices);
        p.vertices[0] = p.vertices[0].remap(u1 as f32, v0 as f32);
        p.vertices[1] = p.vertices[1].remap(u0 as f32, v0 as f32);
        p.vertices[2] = p.vertices[2].remap(u0 as f32, v1 as f32);
        p.vertices[3] = p.vertices[3].remap(u1 as f32, v1 as f32);
        p
    }

    pub unsafe fn render(&self) {
        glColor3f(1.0, 1.0, 1.0);
        let mut i = 3;
        while i >= 0 {
            let v = &self.vertices[i as usize];
            glTexCoord2f(v.u / 64.0, v.v / 32.0);
            glVertex3f(v.pos.x, v.pos.y, v.pos.z);
            i -= 1;
        }
    }
}
