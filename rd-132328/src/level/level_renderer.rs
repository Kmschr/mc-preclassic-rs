use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::time::UNIX_EPOCH;
use std::{cell::RefCell, time::SystemTime};

use crate::glu::*;

use crate::{hit_result::HitResult, player::Player};

use super::{
    chunk::{self, Chunk},
    frustrum::Frustum,
    level::Level,
    level_listener::LevelListener,
    tesselator::Tesselator,
    tile,
};

const CHUNK_SIZE: i32 = 16;

pub struct LevelRenderer {
    level: Rc<RefCell<Level>>,
    chunks: Vec<Option<Chunk>>,
    x_chunks: i32,
    y_chunks: i32,
    z_chunks: i32,
    t: Tesselator,
}

impl LevelRenderer {
    pub fn new(level: Rc<RefCell<Level>>) -> Rc<RefCell<LevelRenderer>> {
        let x_chunks = level.borrow().width / CHUNK_SIZE;
        let y_chunks = level.borrow().depth / CHUNK_SIZE;
        let z_chunks = level.borrow().height / CHUNK_SIZE;

        let mut chunks: Vec<Option<Chunk>> = std::iter::repeat_with(|| None)
            .take((x_chunks * y_chunks * z_chunks) as usize)
            .collect();

        for x in 0..x_chunks {
            for y in 0..y_chunks {
                for z in 0..z_chunks {
                    let x0 = x * CHUNK_SIZE;
                    let y0 = y * CHUNK_SIZE;
                    let z0 = z * CHUNK_SIZE;
                    let mut x1 = (x + 1) * CHUNK_SIZE;
                    let mut y1 = (y + 1) * CHUNK_SIZE;
                    let mut z1 = (z + 1) * CHUNK_SIZE;
                    if x1 > level.borrow().width {
                        x1 = level.borrow().width;
                    }
                    if y1 > level.borrow().depth {
                        y1 = level.borrow().depth;
                    }
                    if z1 > level.borrow().height {
                        z1 = level.borrow().height;
                    }
                    chunks[((x + y * x_chunks) * z_chunks + z) as usize] =
                        Some(Chunk::new(Rc::clone(&level), x0, y0, z0, x1, y1, z1));
                }
            }
        }

        let lr = Rc::new(RefCell::new(LevelRenderer {
            level,
            chunks,
            x_chunks,
            y_chunks,
            z_chunks,
            t: Tesselator::new(),
        }));

        lr.borrow()
            .level
            .borrow_mut()
            .add_listener(Rc::clone(&lr) as Rc<RefCell<dyn LevelListener>>);

        lr
    }

    pub fn render(&mut self, _player: &Player, layer: i32) {
        chunk::REBUILT_THIS_FRAME.store(0, Ordering::SeqCst);
        let frustum = Frustum::get_frustum();

        for chunk in &mut self.chunks {
            if let Some(chunk) = chunk {
                if frustum.lock().unwrap().cube_in_frustum_aabb(&chunk.aabb) {
                    chunk.render(layer);
                }
            }
        }
    }

    pub fn pick(&mut self, player: &Player) {
        let r = 3.0;
        let box_aabb = player.entity.bb.grow(r, r, r);
        let x0 = box_aabb.x0 as i32;
        let x1 = (box_aabb.x1 + 1.0) as i32;
        let y0 = box_aabb.y0 as i32;
        let y1 = (box_aabb.y1 + 1.0) as i32;
        let z0 = box_aabb.z0 as i32;
        let z1 = (box_aabb.z1 + 1.0) as i32;

        unsafe {
            glInitNames();
            for x in x0..x1 {
                glPushName(x as u32);
                for y in y0..y1 {
                    glPushName(y as u32);
                    for z in z0..z1 {
                        glPushName(z as u32);
                        if self.level.borrow().is_solid_tile(x, y, z) {
                            glPushName(0);
                            for i in 0..6 {
                                glPushName(i);
                                self.t.init();
                                tile::ROCK.lock().unwrap().render_face(
                                    &mut self.t,
                                    x,
                                    y,
                                    z,
                                    i as i32,
                                );
                                self.t.flush();
                                glPopName();
                            }
                            glPopName();
                        }
                        glPopName();
                    }
                    glPopName();
                }
                glPopName();
            }
        }
    }

    pub fn render_hit(&mut self, h: &HitResult) {
        unsafe {
            glEnable(GL_BLEND);
            glBlendFunc(GL_SRC_ALPHA, 1);

            let current_time_millis = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();

            glColor4f(
                1.0,
                1.0,
                1.0,
                ((current_time_millis as f64 / 100.0).sin() * 0.2 + 0.4) as f32,
            );
            self.t.init();
            tile::ROCK
                .lock()
                .unwrap()
                .render_face(&mut self.t, h.x, h.y, h.z, h.f);
            self.t.flush();
            glDisable(GL_BLEND);
        }
    }

    pub fn set_dirty(&mut self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) {
        let mut x0 = x0 / CHUNK_SIZE;
        let mut x1 = x1 / CHUNK_SIZE;
        let mut y0 = y0 / CHUNK_SIZE;
        let mut y1 = y1 / CHUNK_SIZE;
        let mut z0 = z0 / CHUNK_SIZE;
        let mut z1 = z1 / CHUNK_SIZE;
        if x0 < 0 {
            x0 = 0;
        }
        if y0 < 0 {
            y0 = 0;
        }
        if z0 < 0 {
            z0 = 0;
        }
        if x1 >= self.x_chunks {
            x1 = self.x_chunks - 1;
        }
        if y1 >= self.y_chunks {
            y1 = self.y_chunks - 1;
        }
        if z1 >= self.z_chunks {
            z1 = self.z_chunks - 1;
        }

        for x in x0..=x1 {
            for y in y0..=y1 {
                for z in z0..=z1 {
                    if let Some(chunk) =
                        &mut self.chunks[((x + y * self.x_chunks) * self.z_chunks + z) as usize]
                    {
                        chunk.set_dirty();
                    }
                }
            }
        }
    }
}

impl LevelListener for LevelRenderer {
    fn tile_changed(&mut self, x: i32, y: i32, z: i32) {
        self.set_dirty(x - 1, y - 1, z - 1, x + 1, y + 1, z + 1);
    }

    fn light_column_changed(&mut self, x: i32, z: i32, y0: i32, y1: i32) {
        self.set_dirty(x - 1, y0 - 1, z - 1, x + 1, y1 + 1, z + 1);
    }

    fn all_changed(&mut self) {
        let x1 = self.level.borrow().width;
        let y1 = self.level.borrow().depth;
        let z1 = self.level.borrow().height;
        self.set_dirty(0, 0, 0, x1, y1, z1);
    }
}
