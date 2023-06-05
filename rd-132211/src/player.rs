use std::cell::RefCell;
use std::rc::Rc;

use crate::{level::level::Level, phys::aabb::AABB};
use lwrgl::glfw::Key;
use lwrgl::LWRGL;

pub struct Player {
    level: Rc<RefCell<Level>>,
    pub xo: f32,
    pub yo: f32,
    pub zo: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub xd: f32,
    pub yd: f32,
    pub zd: f32,
    pub y_rot: f32,
    pub x_rot: f32,
    pub bb: AABB,
    pub on_ground: bool,
}

impl Player {
    pub fn new(level: Rc<RefCell<Level>>) -> Player {
        let x = rand::random::<f32>() * level.borrow().width as f32;
        let y = (level.borrow().depth + 10) as f32;
        let z = rand::random::<f32>() * level.borrow().height as f32;

        let w = 0.3;
        let h = 0.9;

        Player {
            level,
            xo: 0.0,
            yo: 0.0,
            zo: 0.0,
            x,
            y,
            z,
            xd: 0.0,
            yd: 0.0,
            zd: 0.0,
            y_rot: 0.0,
            x_rot: 0.0,
            bb: AABB::new(x - w, y - h, z - w, x + w, y + h, z + w),
            on_ground: false,
        }
    }

    pub fn reset_pos(&mut self) {
        let x = rand::random::<f32>() * self.level.borrow().width as f32;
        let y = (self.level.borrow().depth + 10) as f32;
        let z = rand::random::<f32>() * self.level.borrow().height as f32;
        self.set_pos(x, y, z);
    }

    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
        let w = 0.3;
        let h = 0.9;
        self.bb = AABB::new(x - w, y - h, z - w, x + w, y + h, z + w);
    }

    pub fn turn(&mut self, xo: i32, yo: i32) {
        self.y_rot = (self.y_rot as f64 + (xo as f64 * 0.15)) as f32;
        self.x_rot = (self.x_rot as f64 + (yo as f64 * 0.15)) as f32;

        if self.x_rot < -90.0 {
            self.x_rot = -90.0;
        }
        if self.x_rot > 90.0 {
            self.x_rot = 90.0;
        }
    }

    pub fn tick(&mut self, lwrgl: &LWRGL) {
        self.xo = self.x;
        self.yo = self.y;
        self.zo = self.z;
        let mut xa = 0.0;
        let mut ya = 0.0;

        if lwrgl.is_key_down(Key::R) {
            self.reset_pos();
        }
        if lwrgl.is_key_down(Key::Up) || lwrgl.is_key_down(Key::W) {
            ya -= 1.0;
        }
        if lwrgl.is_key_down(Key::Down) || lwrgl.is_key_down(Key::S) {
            ya += 1.0;
        }
        if lwrgl.is_key_down(Key::Left) || lwrgl.is_key_down(Key::A) {
            xa -= 1.0;
        }
        if lwrgl.is_key_down(Key::Right) || lwrgl.is_key_down(Key::D) {
            xa += 1.0;
        }
        if (lwrgl.is_key_down(Key::Space) || lwrgl.is_key_down(Key::LeftSuper)) && self.on_ground {
            self.yd = 0.12;
        }
        let speed = if self.on_ground { 0.02 } else { 0.005 };
        self.move_relative(xa, ya, speed);
        self.yd = (self.yd as f64 - 0.005) as f32;
        self.move_(self.xd, self.yd, self.zd);
        self.xd *= 0.91;
        self.yd *= 0.98;
        self.zd *= 0.91;
        if self.on_ground {
            self.xd *= 0.8;
            self.zd *= 0.8;
        }
    }

    pub fn move_(&mut self, xa: f32, ya: f32, za: f32) {
        let mut xa = xa;
        let mut ya = ya;
        let mut za = za;
        let xa_org = xa;
        let ya_org = ya;
        let za_org = za;
        let aabbs = self.level.borrow().get_cubes(self.bb.expand(xa, ya, za));
        for aabb in &aabbs {
            ya = aabb.clip_y_collide(&self.bb, ya);
        }
        self.bb.move_(0.0, ya, 0.0);
        for aabb in &aabbs {
            xa = aabb.clip_x_collide(&self.bb, xa);
        }
        self.bb.move_(xa, 0.0, 0.0);
        for aabb in &aabbs {
            za = aabb.clip_z_collide(&self.bb, za);
        }
        self.bb.move_(0.0, 0.0, za);
        self.on_ground = ya_org != ya && ya_org < 0.0;
        if xa_org != xa {
            self.xd = 0.0;
        }
        if ya_org != ya {
            self.yd = 0.0;
        }
        if za_org != za {
            self.zd = 0.0;
        }
        self.x = (self.bb.x0 + self.bb.x1) / 2.0;
        self.y = self.bb.y0 + 1.62;
        self.z = (self.bb.z0 + self.bb.z1) / 2.0;
    }

    pub fn move_relative(&mut self, xa: f32, za: f32, speed: f32) {
        let mut xa = xa;
        let mut za = za;
        let mut dist = xa * xa + za * za;
        if dist < 0.01 {
            return;
        }
        dist = speed / dist.sqrt();
        let sin = (self.y_rot as f64 * std::f64::consts::PI / 180.0).sin() as f32;
        let cos = (self.y_rot as f64 * std::f64::consts::PI / 180.0).cos() as f32;
        xa *= dist;
        za *= dist;
        self.xd += xa * cos - za * sin;
        self.zd += za * cos + xa * sin;
    }
}
