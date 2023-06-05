use std::{cell::RefCell, f64::consts::PI, rc::Rc, time::Instant};

use lwrgl::glu_sys::{
    glBindTexture, glDisable, glEnable, glPopMatrix, glPushMatrix, glRotatef, glScalef,
    glTranslatef, GLuint, GL_NEAREST, GL_TEXTURE_2D,
};

use crate::{
    entity::{Entity, EntityTrait},
    level::level::Level,
    textures::load_texture,
    timer::PROGRAM_START,
};

use super::cube::Cube;

pub struct Zombie {
    pub entity: Entity,
    pub head: Cube,
    pub body: Cube,
    pub arm0: Cube,
    pub arm1: Cube,
    pub leg0: Cube,
    pub leg1: Cube,
    pub rot: f32,
    pub time_offs: f32,
    pub speed: f32,
    pub rot_a: f32,
}

impl Zombie {
    pub fn new(level: Rc<RefCell<Level>>, x: f32, y: f32, z: f32) -> Zombie {
        let mut entity = Entity::new(level);
        entity.x = x;
        entity.y = y;
        entity.z = z;

        let mut head = Cube::new(0, 0);
        head.add_box(-4.0, -8.0, -4.0, 8, 8, 8);

        let mut body = Cube::new(16, 16);
        body.add_box(-4.0, 0.0, -2.0, 8, 12, 4);

        let mut arm0 = Cube::new(40, 16);
        arm0.add_box(-3.0, -2.0, -2.0, 4, 12, 4);
        arm0.set_pos(-5.0, 2.0, 0.0);

        let mut arm1 = Cube::new(40, 16);
        arm1.add_box(-1.0, -2.0, -2.0, 4, 12, 4);
        arm1.set_pos(5.0, 2.0, 0.0);

        let mut leg0 = Cube::new(0, 16);
        leg0.add_box(-2.0, 0.0, -2.0, 4, 12, 4);
        leg0.set_pos(-2.0, 12.0, 0.0);

        let mut leg1 = Cube::new(0, 16);
        leg1.add_box(-2.0, 0.0, -2.0, 4, 12, 4);
        leg1.set_pos(2.0, 12.0, 0.0);

        Zombie {
            entity,
            head,
            body,
            arm0,
            arm1,
            leg0,
            leg1,
            rot: (rand::random::<f64>() * PI * 2.0) as f32,
            time_offs: rand::random::<f64>() as f32 * 1239813.0,
            speed: 1.0,
            rot_a: (rand::random::<f64>() + 1.0) as f32 * 0.01,
        }
    }

    pub unsafe fn render(&mut self, a: f32) {
        glEnable(GL_TEXTURE_2D);
        glBindTexture(
            GL_TEXTURE_2D,
            load_texture("char.png", GL_NEAREST as i32) as GLuint,
        );
        glPushMatrix();
        let time = Instant::now().duration_since(*PROGRAM_START).as_nanos() as f64 / 1.0E9
            * 10.0
            * self.speed as f64
            + self.time_offs as f64;

        let size = 0.058333334;
        let yy = (-((time * 0.6662).sin().abs()) * 5.0 - 23.0) as f32;
        let this = &mut self.entity;
        glTranslatef(
            this.xo + (this.x - this.xo) * a,
            this.yo + (this.y - this.yo) * a,
            this.zo + (this.z - this.zo) * a,
        );
        glScalef(1.0, -1.0, 1.0);
        glScalef(size, size, size);
        glTranslatef(0.0, yy, 0.0);
        let c = 57.29578 as f32;
        glRotatef(self.rot * c + 180.0, 0.0, 1.0, 0.0);
        self.head.y_rot = (time * 0.83).sin() as f32 * 1.0;
        self.head.x_rot = time.sin() as f32 * 0.8;
        self.arm0.x_rot = (time * 0.6662 + PI).sin() as f32 * 2.0;
        self.arm0.z_rot = ((time * 0.2312).sin() + 1.0) as f32 * 1.0;
        self.arm1.x_rot = (time * 0.6662).sin() as f32 * 2.0;
        self.arm1.z_rot = ((time * 0.2812).sin() - 1.0) as f32 * 1.0;
        self.leg0.x_rot = (time * 0.6662).sin() as f32 * 1.4;
        self.leg1.x_rot = (time * 0.6662 + PI).sin() as f32 * 1.4;
        self.head.render();
        self.body.render();
        self.arm0.render();
        self.arm1.render();
        self.leg0.render();
        self.leg1.render();
        glPopMatrix();
        glDisable(GL_TEXTURE_2D);
    }
}

impl EntityTrait for Zombie {
    fn reset_pos(&mut self) {
        self.entity.reset_pos();
    }

    fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.entity.set_pos(x, y, z);
    }

    fn turn(&mut self, xo: f32, yo: f32) {
        self.entity.turn(xo, yo);
    }

    fn tick(&mut self, _lwrgl: &lwrgl::LWRGL) {
        let this = &mut self.entity;
        this.xo = this.x;
        this.yo = this.y;
        this.zo = this.z;
        self.rot += self.rot_a;
        self.rot_a = (self.rot_a as f64 * 0.99) as f32;
        self.rot_a = (self.rot_a as f64
            + (rand::random::<f64>() - rand::random::<f64>())
                * rand::random::<f64>()
                * rand::random::<f64>()
                * 0.01) as f32;
        let xa = self.rot.sin();
        let ya = self.rot.cos();
        if this.on_ground && rand::random::<f64>() < 0.01 {
            this.yd = 0.12;
        }
        this.move_relative(xa, ya, if this.on_ground { 0.02 } else { 0.005 });
        this.yd = (this.yd as f64 - 0.005) as f32;
        this.move_(this.xd, this.yd, this.zd);
        this.xd *= 0.91;
        this.yd *= 0.98;
        this.zd *= 0.91;
        if this.y > 100.0 {
            this.reset_pos();
        }
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
