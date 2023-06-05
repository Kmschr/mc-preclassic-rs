use std::sync::Mutex;

use crate::glu::*;

use crate::phys::aabb::AABB;

pub const RIGHT: usize = 0;
pub const LEFT: usize = 1;
pub const BOTTOM: usize = 2;
pub const TOP: usize = 3;
pub const BACK: usize = 4;
pub const FRONT: usize = 5;
pub const A: usize = 0;
pub const B: usize = 1;
pub const C: usize = 2;
pub const D: usize = 3;

static FRUSTUM: Mutex<Frustum> = Mutex::new(Frustum::new());

pub struct Frustum {
    pub m_frustum: [[f32; 4]; 6],
    _proj: [GLfloat; 16],
    _modl: [GLfloat; 16],
    _clip: [GLfloat; 16],
    proj: [f32; 16],
    modl: [f32; 16],
    clip: [f32; 16],
}

impl Frustum {
    const fn new() -> Frustum {
        Frustum {
            m_frustum: [[0.0; 4]; 6],
            _proj: [0.0; 16],
            _modl: [0.0; 16],
            _clip: [0.0; 16],
            proj: [0.0; 16],
            modl: [0.0; 16],
            clip: [0.0; 16],
        }
    }

    pub fn get_frustum() -> &'static Mutex<Frustum> {
        FRUSTUM.lock().unwrap().calculate_frustum();
        &FRUSTUM
    }

    fn normalize_plane(frust: &mut [[f32; 4]; 6], side: usize) {
        let magnitude = (frust[side][0] * frust[side][0]
            + frust[side][1] * frust[side][1]
            + frust[side][2] * frust[side][2])
            .sqrt();
        let f_array = &mut frust[side];
        f_array[A] /= magnitude;
        f_array[B] /= magnitude;
        f_array[C] /= magnitude;
        f_array[D] /= magnitude;
    }

    fn calculate_frustum(&mut self) {
        for i in 0..16 {
            self._proj[i] = 0.0;
            self._modl[i] = 0.0;
            self._clip[i] = 0.0;
        }
        unsafe {
            glGetFloatv(GL_PROJECTION_MATRIX, self._proj.as_mut_ptr());
            glGetFloatv(GL_MODELVIEW_MATRIX, self._modl.as_mut_ptr());
        }

        self.proj = self._proj;
        self.modl = self._modl;

        self.clip[0] = self.modl[0] * self.proj[0]
            + self.modl[1] * self.proj[4]
            + self.modl[2] * self.proj[8]
            + self.modl[3] * self.proj[12];
        self.clip[1] = self.modl[0] * self.proj[1]
            + self.modl[1] * self.proj[5]
            + self.modl[2] * self.proj[9]
            + self.modl[3] * self.proj[13];
        self.clip[2] = self.modl[0] * self.proj[2]
            + self.modl[1] * self.proj[6]
            + self.modl[2] * self.proj[10]
            + self.modl[3] * self.proj[14];
        self.clip[3] = self.modl[0] * self.proj[3]
            + self.modl[1] * self.proj[7]
            + self.modl[2] * self.proj[11]
            + self.modl[3] * self.proj[15];
        self.clip[4] = self.modl[4] * self.proj[0]
            + self.modl[5] * self.proj[4]
            + self.modl[6] * self.proj[8]
            + self.modl[7] * self.proj[12];
        self.clip[5] = self.modl[4] * self.proj[1]
            + self.modl[5] * self.proj[5]
            + self.modl[6] * self.proj[9]
            + self.modl[7] * self.proj[13];
        self.clip[6] = self.modl[4] * self.proj[2]
            + self.modl[5] * self.proj[6]
            + self.modl[6] * self.proj[10]
            + self.modl[7] * self.proj[14];
        self.clip[7] = self.modl[4] * self.proj[3]
            + self.modl[5] * self.proj[7]
            + self.modl[6] * self.proj[11]
            + self.modl[7] * self.proj[15];
        self.clip[8] = self.modl[8] * self.proj[0]
            + self.modl[9] * self.proj[4]
            + self.modl[10] * self.proj[8]
            + self.modl[11] * self.proj[12];
        self.clip[9] = self.modl[8] * self.proj[1]
            + self.modl[9] * self.proj[5]
            + self.modl[10] * self.proj[9]
            + self.modl[11] * self.proj[13];
        self.clip[10] = self.modl[8] * self.proj[2]
            + self.modl[9] * self.proj[6]
            + self.modl[10] * self.proj[10]
            + self.modl[11] * self.proj[14];
        self.clip[11] = self.modl[8] * self.proj[3]
            + self.modl[9] * self.proj[7]
            + self.modl[10] * self.proj[11]
            + self.modl[11] * self.proj[15];
        self.clip[12] = self.modl[12] * self.proj[0]
            + self.modl[13] * self.proj[4]
            + self.modl[14] * self.proj[8]
            + self.modl[15] * self.proj[12];
        self.clip[13] = self.modl[12] * self.proj[1]
            + self.modl[13] * self.proj[5]
            + self.modl[14] * self.proj[9]
            + self.modl[15] * self.proj[13];
        self.clip[14] = self.modl[12] * self.proj[2]
            + self.modl[13] * self.proj[6]
            + self.modl[14] * self.proj[10]
            + self.modl[15] * self.proj[14];
        self.clip[15] = self.modl[12] * self.proj[3]
            + self.modl[13] * self.proj[7]
            + self.modl[14] * self.proj[11]
            + self.modl[15] * self.proj[15];
        self.m_frustum[RIGHT][A] = self.clip[3] - self.clip[0];
        self.m_frustum[RIGHT][B] = self.clip[7] - self.clip[4];
        self.m_frustum[RIGHT][C] = self.clip[11] - self.clip[8];
        self.m_frustum[RIGHT][D] = self.clip[15] - self.clip[12];
        Self::normalize_plane(&mut self.m_frustum, RIGHT);
        self.m_frustum[LEFT][A] = self.clip[3] + self.clip[0];
        self.m_frustum[LEFT][B] = self.clip[7] + self.clip[4];
        self.m_frustum[LEFT][C] = self.clip[11] + self.clip[8];
        self.m_frustum[LEFT][D] = self.clip[15] + self.clip[12];
        Self::normalize_plane(&mut self.m_frustum, LEFT);
        self.m_frustum[BOTTOM][A] = self.clip[3] + self.clip[1];
        self.m_frustum[BOTTOM][B] = self.clip[7] + self.clip[5];
        self.m_frustum[BOTTOM][C] = self.clip[11] + self.clip[9];
        self.m_frustum[BOTTOM][D] = self.clip[15] + self.clip[13];
        Self::normalize_plane(&mut self.m_frustum, BOTTOM);
        self.m_frustum[TOP][A] = self.clip[3] - self.clip[1];
        self.m_frustum[TOP][B] = self.clip[7] - self.clip[5];
        self.m_frustum[TOP][C] = self.clip[11] - self.clip[9];
        self.m_frustum[TOP][D] = self.clip[15] - self.clip[13];
        Self::normalize_plane(&mut self.m_frustum, TOP);
        self.m_frustum[BACK][A] = self.clip[3] - self.clip[2];
        self.m_frustum[BACK][B] = self.clip[7] - self.clip[6];
        self.m_frustum[BACK][C] = self.clip[11] - self.clip[10];
        self.m_frustum[BACK][D] = self.clip[15] - self.clip[14];
        Self::normalize_plane(&mut self.m_frustum, BACK);
        self.m_frustum[FRONT][A] = self.clip[3] + self.clip[2];
        self.m_frustum[FRONT][B] = self.clip[7] + self.clip[6];
        self.m_frustum[FRONT][C] = self.clip[11] + self.clip[10];
        self.m_frustum[FRONT][D] = self.clip[15] + self.clip[14];
        Self::normalize_plane(&mut self.m_frustum, FRONT);
    }

    pub fn cube_in_frustum(&self, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> bool {
        for i in 0..6 {
            if !(self.m_frustum[i][0] * x1
                + self.m_frustum[i][1] * y1
                + self.m_frustum[i][2] * z1
                + self.m_frustum[i][3]
                > 0.0
                || self.m_frustum[i][0] * x2
                    + self.m_frustum[i][1] * y1
                    + self.m_frustum[i][2] * z1
                    + self.m_frustum[i][3]
                    > 0.0
                || self.m_frustum[i][0] * x1
                    + self.m_frustum[i][1] * y2
                    + self.m_frustum[i][2] * z1
                    + self.m_frustum[i][3]
                    > 0.0
                || self.m_frustum[i][0] * x2
                    + self.m_frustum[i][1] * y2
                    + self.m_frustum[i][2] * z1
                    + self.m_frustum[i][3]
                    > 0.0
                || self.m_frustum[i][0] * x1
                    + self.m_frustum[i][1] * y1
                    + self.m_frustum[i][2] * z2
                    + self.m_frustum[i][3]
                    > 0.0
                || self.m_frustum[i][0] * x2
                    + self.m_frustum[i][1] * y1
                    + self.m_frustum[i][2] * z2
                    + self.m_frustum[i][3]
                    > 0.0
                || self.m_frustum[i][0] * x1
                    + self.m_frustum[i][1] * y2
                    + self.m_frustum[i][2] * z2
                    + self.m_frustum[i][3]
                    > 0.0
                || self.m_frustum[i][0] * x2
                    + self.m_frustum[i][1] * y2
                    + self.m_frustum[i][2] * z2
                    + self.m_frustum[i][3]
                    > 0.0)
            {
                return false;
            }
        }
        true
    }

    pub fn cube_in_frustum_aabb(&self, aabb: &AABB) -> bool {
        self.cube_in_frustum(aabb.x0, aabb.y0, aabb.z0, aabb.x1, aabb.y1, aabb.z1)
    }
}
