#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use character::zombie::Zombie;
use entity::EntityTrait;
use hit_result::HitResult;
use level::level_renderer::LevelRenderer;
use lwrgl::glfw::Key;
use lwrgl::glu_sys::glu;
use lwrgl::glu_sys::glu::*;
use lwrgl::LWRGL;

use level::level::Level;
use player::Player;
mod character;
mod entity;
mod hit_result;
mod level;
mod phys;
mod player;
mod textures;
mod timer;

use crate::level::chunk;
use crate::timer::Timer;

struct RubyDung {
    lwrgl: LWRGL,
    width: i32,
    height: i32,
    fog_color: [GLfloat; 4],
    timer: Timer,
    level: Rc<RefCell<Level>>,
    level_renderer: Rc<RefCell<LevelRenderer>>,
    player: Player,
    select_buffer: [GLuint; 2000],
    viewport_buffer: [GLint; 16],
    hit_result: Option<HitResult>,
    zombies: Vec<Zombie>,
}

impl RubyDung {
    pub fn new() -> RubyDung {
        let col = 0x0E0B0A;
        let fr = 0.5;
        let fg = 0.8;
        let fb = 1.0;
        let fog_color = [
            (col >> 16 & 0xFF) as f32 / 255.0,
            (col >> 8 & 0xFF) as f32 / 255.0,
            (col & 0xFF) as f32 / 255.0,
            1.0,
        ];

        let mut lwrgl = LWRGL::new(1024, 768);

        unsafe {
            let width = lwrgl.get_display_width();
            let height = lwrgl.get_display_height();

            glEnable(GL_TEXTURE_2D);
            glShadeModel(GL_SMOOTH);
            glClearColor(fr, fg, fb, 0.0);
            glClearDepth(1.0);
            glEnable(GL_DEPTH_TEST);
            glDepthFunc(GL_LEQUAL);
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            glMatrixMode(GL_MODELVIEW);

            let level = Rc::new(RefCell::new(Level::new(256, 256, 64)));
            let level_renderer = LevelRenderer::new(Rc::clone(&level));
            let player = Player::new(Rc::clone(&level));

            lwrgl.grab_mouse();

            let mut zombies = Vec::with_capacity(100);
            for _ in 0..100 {
                zombies.push(Zombie::new(Rc::clone(&level), 128.0, 0.0, 128.0));
            }

            RubyDung {
                lwrgl,
                width,
                height,
                fog_color,
                timer: Timer::new(60.0),
                level,
                level_renderer,
                player,
                select_buffer: [0; 2000],
                viewport_buffer: [0; 16],
                hit_result: None,
                zombies,
            }
        }
    }

    pub fn destroy(&self) {
        self.level.borrow().save();
    }

    pub fn run(&mut self) {
        let mut frames = 0;
        let mut last_time = Instant::now();

        loop {
            if self.lwrgl.is_close_requested() || self.lwrgl.is_key_down(Key::Escape) {
                break;
            }

            self.timer.advance_time();
            let mut i = 0;
            while i < self.timer.ticks {
                self.tick();
                i += 1;
            }
            self.render(self.timer.a);
            frames += 1;

            while Instant::now().duration_since(last_time).as_millis() > 1000 {
                println!("{} fps, {}", frames, chunk::UPDATES.load(Ordering::SeqCst));
                chunk::UPDATES.store(0, Ordering::SeqCst);
                last_time = last_time.checked_add(Duration::from_millis(1000)).unwrap();
                frames = 0;
            }
        }

        self.destroy();
    }

    pub fn tick(&mut self) {
        for zombie in &mut self.zombies {
            zombie.tick(&self.lwrgl);
        }
        self.player.tick(&self.lwrgl);
    }

    pub fn move_camera_to_player(&self, a: f32) {
        unsafe {
            glTranslatef(0.0, 0.0, -0.3);
            glRotatef(self.player.entity.x_rot, 1.0, 0.0, 0.0);
            glRotatef(self.player.entity.y_rot, 0.0, 1.0, 0.0);
            let x = self.player.entity.xo + (self.player.entity.x - self.player.entity.xo) * a;
            let y = self.player.entity.yo + (self.player.entity.y - self.player.entity.yo) * a;
            let z = self.player.entity.zo + (self.player.entity.z - self.player.entity.zo) * a;
            glTranslatef(-x, -y, -z);
        }
    }

    pub fn setup_camera(&self, a: f32) {
        unsafe {
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            gluPerspective(70.0, self.width as f64 / self.height as f64, 0.05, 1000.0);
            glMatrixMode(GL_MODELVIEW);
            glLoadIdentity();
            self.move_camera_to_player(a);
        }
    }

    fn setup_pick_camera(&mut self, a: f32, x: i32, y: i32) {
        unsafe {
            glMatrixMode(GL_PROJECTION);
            glLoadIdentity();
            for i in 0..16 {
                self.viewport_buffer[i] = 0;
            }

            glGetIntegerv(GL_VIEWPORT, self.viewport_buffer.as_mut_ptr());
            gluPickMatrix(
                x as f64,
                y as f64,
                5.0,
                5.0,
                self.viewport_buffer.as_mut_ptr(),
            );
            gluPerspective(70.0, self.width as f64 / self.height as f64, 0.05, 1000.0);
            glMatrixMode(GL_MODELVIEW);
            glLoadIdentity();
            self.move_camera_to_player(a);
        }
    }

    pub fn pick(&mut self, a: f32) {
        for i in 0..2000 {
            self.select_buffer[i] = 0;
        }
        unsafe {
            glSelectBuffer(2000, self.select_buffer.as_mut_ptr());
            glRenderMode(GL_SELECT);
            self.setup_pick_camera(a, self.width / 2, self.height / 2);
            self.level_renderer.borrow_mut().pick(&self.player);
            let hits = glRenderMode(GL_RENDER);
            let mut closest = 0;
            let mut names = [0i32; 10];
            let mut hit_name_count = 0;
            let mut pos = 0;
            for i in 0..hits {
                let name_count = self.select_buffer[pos];
                pos += 1;
                let min_z = self.select_buffer[pos];
                pos += 1;
                pos += 1;
                let dist = min_z;
                if dist < closest || i == 0 {
                    closest = dist;
                    hit_name_count = name_count;
                    for j in 0..name_count {
                        names[j as usize] = self.select_buffer[pos] as i32;
                        pos += 1;
                    }
                } else {
                    pos += name_count as usize;
                }
            }

            self.hit_result = if hit_name_count > 0 {
                Some(HitResult::new(
                    names[0], names[1], names[2], names[3], names[4],
                ))
            } else {
                None
            };
        }
    }

    pub fn render(&mut self, a: f32) {
        let xo = self.lwrgl.mouse_dx();
        let yo = self.lwrgl.mouse_dy();

        self.player.turn(xo as f32, yo as f32);
        self.pick(a);

        while self.lwrgl.mouse_next() {
            if self.lwrgl.mouse_event_button() == 1 && self.lwrgl.mouse_event_button_state() {
                if let Some(hit_result) = &self.hit_result {
                    self.level
                        .borrow_mut()
                        .set_tile(hit_result.x, hit_result.y, hit_result.z, 0);
                }
            }
            if self.lwrgl.mouse_event_button() != 0
                || !self.lwrgl.mouse_event_button_state()
                || self.hit_result.is_none()
            {
                continue;
            }
            if let Some(hit_result) = &self.hit_result {
                let mut x = hit_result.x;
                let mut y = hit_result.y;
                let mut z = hit_result.z;

                if hit_result.f == 0 {
                    y -= 1;
                }
                if hit_result.f == 1 {
                    y += 1;
                }
                if hit_result.f == 2 {
                    z -= 1;
                }
                if hit_result.f == 3 {
                    z += 1;
                }
                if hit_result.f == 4 {
                    x -= 1;
                }
                if hit_result.f == 5 {
                    x += 1;
                }

                self.level.borrow_mut().set_tile(x, y, z, 1);
            }
        }

        if self.lwrgl.is_key_down(Key::Enter) {
            self.level.borrow().save();
        }

        unsafe {
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
            self.setup_camera(a);
            glEnable(GL_CULL_FACE);
            glEnable(GL_FOG);
            glFogi(GL_FOG_MODE, GL_EXP as i32);
            glFogf(GL_FOG_DENSITY, 0.2);
            glFogfv(GL_FOG_COLOR, self.fog_color.as_ptr());
            glDisable(GL_FOG);
            self.level_renderer.borrow_mut().render(&self.player, 0);
            for zombie in &mut self.zombies {
                zombie.render(a);
            }
            glEnable(GL_FOG);
            self.level_renderer.borrow_mut().render(&self.player, 1);
            glDisable(GL_TEXTURE_2D);
            if let Some(hit_result) = &self.hit_result {
                self.level_renderer.borrow_mut().render_hit(hit_result);
            }
            glDisable(GL_FOG);
            self.lwrgl.update();
        }
    }
}

pub fn main() {
    let mut rd = RubyDung::new();
    rd.run();
}
