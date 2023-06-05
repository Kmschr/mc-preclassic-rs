use std::ffi::c_void;

use crate::glu::*;

const MAX_VERTICES: usize = 100000;

pub struct Tesselator {
    vertex_buffer: Box<[GLfloat; MAX_VERTICES * 3]>,
    tex_coord_buffer: Box<[GLfloat; MAX_VERTICES * 2]>,
    color_buffer: Box<[GLfloat; MAX_VERTICES * 3]>,
    vertices: usize,
    u: f32,
    v: f32,
    r: f32,
    g: f32,
    b: f32,
    has_color: bool,
    has_texture: bool,
}

impl Tesselator {
    pub fn new() -> Tesselator {
        Tesselator {
            vertex_buffer: Box::new([0.0; MAX_VERTICES * 3]),
            tex_coord_buffer: Box::new([0.0; MAX_VERTICES * 2]),
            color_buffer: Box::new([0.0; MAX_VERTICES * 3]),
            vertices: 0,
            u: 0.0,
            v: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            has_color: false,
            has_texture: false,
        }
    }

    pub fn flush(&mut self) {
        unsafe {
            glVertexPointer(3, GL_FLOAT, 0, self.vertex_buffer.as_ptr() as *const c_void);
            if self.has_texture {
                glTexCoordPointer(
                    2,
                    GL_FLOAT,
                    0,
                    self.tex_coord_buffer.as_ptr() as *const c_void,
                );
            }
            if self.has_color {
                glColorPointer(3, GL_FLOAT, 0, self.color_buffer.as_ptr() as *const c_void);
            }
            glEnableClientState(GL_VERTEX_ARRAY);
            if self.has_texture {
                glEnableClientState(GL_TEXTURE_COORD_ARRAY);
            }
            if self.has_color {
                glEnableClientState(GL_COLOR_ARRAY);
            }
            glDrawArrays(GL_QUADS, 0, self.vertices as GLint);
            glDisableClientState(GL_VERTEX_ARRAY);
            if self.has_texture {
                glDisableClientState(GL_TEXTURE_COORD_ARRAY);
            }
            if self.has_color {
                glDisableClientState(GL_COLOR_ARRAY);
            }
            self.clear();
        }
    }

    fn clear(&mut self) {
        self.vertices = 0;
    }

    pub fn init(&mut self) {
        self.clear();
        self.has_color = false;
        self.has_texture = false;
    }

    pub fn tex(&mut self, u: f32, v: f32) {
        self.has_texture = true;
        self.u = u;
        self.v = v;
    }

    pub fn color(&mut self, r: f32, g: f32, b: f32) {
        self.has_color = true;
        self.r = r;
        self.g = g;
        self.b = b;
    }

    pub fn vertex(&mut self, x: f32, y: f32, z: f32) {
        self.vertex_buffer[(self.vertices * 3) as usize] = x;
        self.vertex_buffer[(self.vertices * 3 + 1) as usize] = y;
        self.vertex_buffer[(self.vertices * 3 + 2) as usize] = z;
        if self.has_texture {
            self.tex_coord_buffer[(self.vertices * 2) as usize] = self.u;
            self.tex_coord_buffer[(self.vertices * 2 + 1) as usize] = self.v;
        }
        if self.has_color {
            self.color_buffer[(self.vertices * 3) as usize] = self.r;
            self.color_buffer[(self.vertices * 3 + 1) as usize] = self.g;
            self.color_buffer[(self.vertices * 3 + 2) as usize] = self.b;
        }
        self.vertices += 1;
        if self.vertices == MAX_VERTICES {
            self.flush()
        }
    }
}
