use lwrgl::glu_sys::{
    glBegin, glEnd, glPopMatrix, glPushMatrix, glRotatef, glTranslatef, GL_QUADS,
};

use super::{polygon::Polygon, vertex::Vertex};

pub struct Cube {
    vertices: Vec<Vertex>,
    polygons: Vec<Polygon>,
    x_tex_offs: i32,
    y_tex_offs: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub x_rot: f32,
    pub y_rot: f32,
    pub z_rot: f32,
}

impl Cube {
    pub fn new(x_tex_offs: i32, y_tex_offs: i32) -> Cube {
        Cube {
            vertices: vec![],
            polygons: vec![],
            x_tex_offs,
            y_tex_offs,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            x_rot: 0.0,
            y_rot: 0.0,
            z_rot: 0.0,
        }
    }

    pub fn add_box(&mut self, x0: f32, y0: f32, z0: f32, w: i32, h: i32, d: i32) {
        self.vertices = Vec::with_capacity(8);
        self.polygons = Vec::with_capacity(6);
        let x1 = x0 + w as f32;
        let y1 = y0 + h as f32;
        let z1 = z0 + d as f32;
        let u0 = Vertex::new(x0, y0, z0, 0.0, 0.0);
        let u1 = Vertex::new(x1, y0, z0, 0.0, 8.0);
        let u2 = Vertex::new(x1, y1, z0, 8.0, 8.0);
        let u3 = Vertex::new(x0, y1, z0, 8.0, 0.0);
        let l0 = Vertex::new(x0, y0, z1, 0.0, 0.0);
        let l1 = Vertex::new(x1, y0, z1, 0.0, 8.0);
        let l2 = Vertex::new(x1, y1, z1, 8.0, 8.0);
        let l3 = Vertex::new(x0, y1, z1, 8.0, 0.0);
        self.vertices.push(u0.clone());
        self.vertices.push(u1.clone());
        self.vertices.push(u2.clone());
        self.vertices.push(u3.clone());
        self.vertices.push(l0.clone());
        self.vertices.push(l1.clone());
        self.vertices.push(l2.clone());
        self.vertices.push(l3.clone());
        self.polygons.push(Polygon::from_uvs(
            vec![l1.clone(), u1.clone(), u2.clone(), l2.clone()],
            self.x_tex_offs + d + w,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w + d,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![u0.clone(), l0.clone(), l3.clone(), u3.clone()],
            self.x_tex_offs + 0,
            self.y_tex_offs + d,
            self.x_tex_offs + d,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![l1.clone(), l0.clone(), u0.clone(), u1.clone()],
            self.x_tex_offs + d,
            self.y_tex_offs + 0,
            self.x_tex_offs + d + w,
            self.y_tex_offs + d,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![u2.clone(), u3.clone(), l3.clone(), l2.clone()],
            self.x_tex_offs + d + w,
            self.y_tex_offs + 0,
            self.x_tex_offs + d + w + w,
            self.y_tex_offs + d,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![u1.clone(), u0.clone(), u3.clone(), u2.clone()],
            self.x_tex_offs + d,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w,
            self.y_tex_offs + d + h,
        ));
        self.polygons.push(Polygon::from_uvs(
            vec![l0.clone(), l1.clone(), l2.clone(), l3.clone()],
            self.x_tex_offs + d + w + d,
            self.y_tex_offs + d,
            self.x_tex_offs + d + w + d + w,
            self.y_tex_offs + d + h,
        ));
    }

    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub unsafe fn render(&self) {
        let c = 57.29578 as f32;
        glPushMatrix();
        glTranslatef(self.x, self.y, self.z);
        glRotatef(self.z_rot * c, 0.0, 0.0, 1.0);
        glRotatef(self.y_rot * c, 0.0, 1.0, 0.0);
        glRotatef(self.x_rot * c, 1.0, 0.0, 0.0);
        glBegin(GL_QUADS);
        for polygon in &self.polygons {
            polygon.render();
        }
        glEnd();
        glPopMatrix();
    }
}
