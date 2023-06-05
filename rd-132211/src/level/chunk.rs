use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        atomic::{AtomicI32, Ordering},
        Mutex,
    },
};

use crate::glu::*;

use crate::{phys::aabb::AABB, textures};

use super::{level::Level, tesselator::Tesselator, tile};

lazy_static! {
    static ref TEXTURE: AtomicI32 =
        AtomicI32::new(textures::load_texture("terrain.png", GL_NEAREST as i32));
    static ref TESSELATOR: Mutex<Tesselator> = Mutex::new(Tesselator::new());
}

pub static REBUILT_THIS_FRAME: AtomicI32 = AtomicI32::new(0);
pub static UPDATES: AtomicI32 = AtomicI32::new(0);

pub struct Chunk {
    pub aabb: AABB,
    level: Rc<RefCell<Level>>,
    x0: i32,
    y0: i32,
    z0: i32,
    x1: i32,
    y1: i32,
    z1: i32,
    dirty: bool, // should chunk mesh be rebuilt before next render
    lists: i32,
}

impl Chunk {
    pub fn new(
        level: Rc<RefCell<Level>>,
        x0: i32,
        y0: i32,
        z0: i32,
        x1: i32,
        y1: i32,
        z1: i32,
    ) -> Chunk {
        unsafe {
            Chunk {
                aabb: AABB::new(
                    x0 as f32, y0 as f32, z0 as f32, x1 as f32, y1 as f32, z1 as f32,
                ),
                level,
                x0,
                y0,
                z0,
                x1,
                y1,
                z1,
                dirty: true,
                lists: glGenLists(2) as i32,
            }
        }
    }

    fn rebuild(&mut self, layer: i32) {
        if REBUILT_THIS_FRAME.load(Ordering::SeqCst) == 2 {
            return;
        }
        self.dirty = false;
        UPDATES.store(UPDATES.load(Ordering::SeqCst) + 1, Ordering::SeqCst);
        REBUILT_THIS_FRAME.store(
            REBUILT_THIS_FRAME.load(Ordering::SeqCst) + 1,
            Ordering::SeqCst,
        );
        unsafe {
            glNewList((self.lists + layer) as GLuint, GL_COMPILE);
            glEnable(GL_TEXTURE_2D);
            glBindTexture(GL_TEXTURE_2D, TEXTURE.load(Ordering::SeqCst) as u32);
        }
        TESSELATOR.lock().unwrap().init();
        for x in self.x0..self.x1 {
            for y in self.y0..self.y1 {
                for z in self.z0..self.z1 {
                    if self.level.borrow().is_tile(x, y, z) {
                        let tex = y != self.level.borrow().depth * 2 / 3;
                        if !tex {
                            tile::ROCK.lock().unwrap().render(
                                &mut TESSELATOR.lock().unwrap(),
                                &self.level.borrow(),
                                layer,
                                x,
                                y,
                                z,
                            );
                        } else {
                            tile::GRASS.lock().unwrap().render(
                                &mut TESSELATOR.lock().unwrap(),
                                &self.level.borrow(),
                                layer,
                                x,
                                y,
                                z,
                            );
                        }
                    }
                }
            }
        }
        TESSELATOR.lock().unwrap().flush();
        unsafe {
            glDisable(GL_TEXTURE_2D);
            glEndList();
        }
    }

    pub fn render(&mut self, layer: i32) {
        if self.dirty {
            self.rebuild(0);
            self.rebuild(1);
        }
        unsafe {
            glCallList((self.lists + layer) as GLuint);
        }
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }
}
