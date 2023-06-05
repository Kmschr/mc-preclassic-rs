use std::cell::RefCell;
use std::rc::Rc;

use crate::entity::{Entity, EntityTrait};
use crate::level::level::Level;
use lwrgl::glfw::Key;
use lwrgl::LWRGL;

pub struct Player {
    pub entity: Entity,
}

impl Player {
    pub fn new(level: Rc<RefCell<Level>>) -> Player {
        let mut entity = Entity::new(level);
        entity.height_offset = 1.62;
        Player { entity }
    }
}

impl EntityTrait for Player {
    fn reset_pos(&mut self) {
        self.entity.reset_pos()
    }

    fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.entity.set_pos(x, y, z);
    }

    fn turn(&mut self, xo: f32, yo: f32) {
        self.entity.turn(xo, yo);
    }

    fn tick(&mut self, lwrgl: &LWRGL) {
        let this = &mut self.entity;
        this.xo = this.x;
        this.yo = this.y;
        this.zo = this.z;
        let mut xa = 0.0;
        let mut ya = 0.0;
        if lwrgl.is_key_down(Key::R) {
            this.reset_pos();
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
        if (lwrgl.is_key_down(Key::Space) || lwrgl.is_key_down(Key::LeftSuper)) && this.on_ground {
            this.yd = 0.12;
        }
        let speed = if this.on_ground { 0.02 } else { 0.005 };
        this.move_relative(xa, ya, speed);
        this.yd = (this.yd as f64 - 0.005) as f32;
        this.move_(this.xd, this.yd, this.zd);
        this.xd *= 0.91;
        this.yd *= 0.98;
        this.zd *= 0.91;
        if this.on_ground {
            this.xd *= 0.8;
            this.zd *= 0.8;
        }
    }

    fn move_(&mut self, xa: f32, ya: f32, za: f32) {
        self.entity.move_(xa, ya, za);
    }

    fn move_relative(&mut self, xa: f32, za: f32, speed: f32) {
        self.entity.move_relative(xa, za, speed);
    }
}
