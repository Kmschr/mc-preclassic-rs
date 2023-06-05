use std::fs::File;
use std::io::prelude::*;
use std::{cell::RefCell, rc::Rc};

use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;

use crate::phys::aabb::AABB;

use super::level_listener::LevelListener;

pub struct Level {
    pub width: i32,
    pub height: i32,
    pub depth: i32,
    blocks: Vec<u8>,
    light_depths: Vec<i32>,
    level_listeners: Vec<Rc<RefCell<dyn LevelListener>>>,
}

impl Level {
    pub fn new(w: i32, h: i32, d: i32) -> Level {
        let mut level = Level {
            width: w,
            height: h,
            depth: d,
            blocks: vec![0u8; (w * h * d) as usize],
            light_depths: vec![0; (w * h) as usize],
            level_listeners: vec![],
        };

        for x in 0..w {
            for y in 0..d {
                for z in 0..h {
                    let i = (y * h + z) * w + x;
                    level.blocks[i as usize] = if y <= d * 2 / 3 { 1 } else { 0 };
                }
            }
        }

        level.calc_light_depths(0, 0, w, h);
        level.load();

        level
    }

    pub fn load(&mut self) {
        if let Ok(file) = std::fs::read("level.dat") {
            let mut gz = GzDecoder::new(&file[..]);
            self.blocks.clear();
            gz.read_to_end(&mut self.blocks).unwrap();
            self.calc_light_depths(0, 0, self.width, self.height);
            for level_listener in &self.level_listeners {
                level_listener.borrow_mut().all_changed();
            }
        }
    }

    pub fn save(&self) {
        let w = File::create("level.dat").unwrap();
        let mut e = GzEncoder::new(w, Compression::default());
        e.write_all(&self.blocks).unwrap();
    }

    pub fn calc_light_depths(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        for x in x0..(x0 + x1) {
            for z in y0..(y0 + y1) {
                let old_depth = self.light_depths[(x + z * self.width) as usize];
                let mut y = self.depth - 1;
                while y > 0 && !self.is_light_blocker(x, y, z) {
                    y -= 1;
                }
                self.light_depths[(x + z * self.width) as usize] = y;
                if old_depth != y {
                    let yl0 = if old_depth < y { old_depth } else { y };
                    let yl1 = if old_depth > y { old_depth } else { y };
                    for level_listener in &self.level_listeners {
                        level_listener
                            .borrow_mut()
                            .light_column_changed(x, y, yl0, yl1);
                    }
                }
            }
        }
    }

    pub fn add_listener(&mut self, level_listener: Rc<RefCell<dyn LevelListener>>) {
        self.level_listeners.push(level_listener);
    }

    pub fn is_tile(&self, x: i32, y: i32, z: i32) -> bool {
        if x < 0 || y < 0 || z < 0 || x >= self.width || y >= self.depth || z >= self.height {
            return false;
        }
        self.blocks[((y * self.height + z) * self.width + x) as usize] == 1
    }

    pub fn is_solid_tile(&self, x: i32, y: i32, z: i32) -> bool {
        self.is_tile(x, y, z)
    }

    pub fn is_light_blocker(&self, x: i32, y: i32, z: i32) -> bool {
        self.is_solid_tile(x, y, z)
    }

    pub fn get_cubes(&self, aabb: AABB) -> Vec<AABB> {
        let mut aabbs = vec![];
        let mut x0 = aabb.x0 as i32;
        let mut x1 = (aabb.x1 + 1.0) as i32;
        let mut y0 = aabb.y0 as i32;
        let mut y1 = (aabb.y1 + 1.0) as i32;
        let mut z0 = aabb.z0 as i32;
        let mut z1 = (aabb.z1 + 1.0) as i32;
        if x0 < 0 {
            x0 = 0;
        }
        if y0 < 0 {
            y0 = 0;
        }
        if z0 < 0 {
            z0 = 0;
        }
        if x1 > self.width {
            x1 = self.width;
        }
        if y1 > self.depth {
            y1 = self.depth;
        }
        if z1 > self.height {
            z1 = self.height;
        }

        for x in x0..x1 {
            for y in y0..y1 {
                for z in z0..z1 {
                    if self.is_solid_tile(x, y, z) {
                        aabbs.push(AABB::new(
                            x as f32,
                            y as f32,
                            z as f32,
                            (x + 1) as f32,
                            (y + 1) as f32,
                            (z + 1) as f32,
                        ));
                    }
                }
            }
        }

        aabbs
    }

    pub fn get_brightness(&self, x: i32, y: i32, z: i32) -> f32 {
        let dark = 0.8;
        let light = 1.0;
        if x < 0 || y < 0 || z < 0 || x >= self.width || y >= self.depth || z >= self.height {
            return light;
        }
        if y < self.light_depths[(x + z * self.width) as usize] {
            return dark;
        }
        light
    }

    pub fn set_tile(&mut self, x: i32, y: i32, z: i32, tile_type: i32) {
        if x < 0 || y < 0 || z < 0 || x >= self.width || y >= self.depth || z >= self.height {
            return;
        }
        self.blocks[((y * self.height + z) * self.width + x) as usize] = tile_type as u8;
        self.calc_light_depths(x, z, 1, 1);
        for level_listener in &self.level_listeners {
            level_listener.borrow_mut().tile_changed(x, y, z);
        }
    }
}
