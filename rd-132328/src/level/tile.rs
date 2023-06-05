use std::sync::Mutex;

use super::{level::Level, tesselator::Tesselator};

pub static ROCK: Mutex<Tile> = Mutex::new(Tile::new(0));
pub static GRASS: Mutex<Tile> = Mutex::new(Tile::new(1));

pub struct Tile {
    tex: i32,
}

impl Tile {
    pub const fn new(tex: i32) -> Tile {
        Tile { tex }
    }

    pub fn render(
        &mut self,
        t: &mut Tesselator,
        level: &Level,
        layer: i32,
        x: i32,
        y: i32,
        z: i32,
    ) {
        let u0 = self.tex as f32 / 16.0;
        let u1 = u0 + (1.0 / 16.0);
        let v0 = 0.0;
        let v1 = v0 + (1.0 / 16.0);
        let c1 = 1.0;
        let c2 = 0.8;
        let c3 = 0.6;
        let x0 = x as f32;
        let x1 = x as f32 + 1.0;
        let y0 = y as f32;
        let y1 = y as f32 + 1.0;
        let z0 = z as f32;
        let z1 = z as f32 + 1.0;
        let br = level.get_brightness(x, y - 1, z) * c1;
        if !level.is_solid_tile(x, y - 1, z) && ((br == c1) ^ (layer == 1)) {
            t.color(br, br, br);
            t.tex(u0, v1);
            t.vertex(x0, y0, z1);
            t.tex(u0, v0);
            t.vertex(x0, y0, z0);
            t.tex(u1, v0);
            t.vertex(x1, y0, z0);
            t.tex(u1, v1);
            t.vertex(x1, y0, z1);
        }

        let br = level.get_brightness(x, y + 1, z) * c1;
        if !level.is_solid_tile(x, y + 1, z) && ((br == c1) ^ (layer == 1)) {
            t.color(br, br, br);
            t.tex(u1, v1);
            t.vertex(x1, y1, z1);
            t.tex(u1, v0);
            t.vertex(x1, y1, z0);
            t.tex(u0, v0);
            t.vertex(x0, y1, z0);
            t.tex(u0, v1);
            t.vertex(x0, y1, z1);
        }

        let br = level.get_brightness(x, y, z - 1) * c2;
        if !level.is_solid_tile(x, y, z - 1) && ((br == c2) ^ (layer == 1)) {
            t.color(br, br, br);
            t.tex(u1, v0);
            t.vertex(x0, y1, z0);
            t.tex(u0, v0);
            t.vertex(x1, y1, z0);
            t.tex(u0, v1);
            t.vertex(x1, y0, z0);
            t.tex(u1, v1);
            t.vertex(x0, y0, z0);
        }

        let br = level.get_brightness(x, y, z + 1) * c2;
        if !level.is_solid_tile(x, y, z + 1) && ((br == c2) ^ (layer == 1)) {
            t.color(br, br, br);
            t.tex(u0, v0);
            t.vertex(x0, y1, z1);
            t.tex(u0, v1);
            t.vertex(x0, y0, z1);
            t.tex(u1, v1);
            t.vertex(x1, y0, z1);
            t.tex(u1, v0);
            t.vertex(x1, y1, z1);
        }

        let br = level.get_brightness(x - 1, y, z) * c3;
        if !level.is_solid_tile(x - 1, y, z) && ((br == c3) ^ (layer == 1)) {
            t.color(br, br, br);
            t.tex(u1, v0);
            t.vertex(x0, y1, z1);
            t.tex(u0, v0);
            t.vertex(x0, y1, z0);
            t.tex(u0, v1);
            t.vertex(x0, y0, z0);
            t.tex(u1, v1);
            t.vertex(x0, y0, z1);
        }

        let br = level.get_brightness(x + 1, y, z) * c3;
        if !level.is_solid_tile(x + 1, y, z) && ((br == c3) ^ (layer == 1)) {
            t.color(br, br, br);
            t.tex(u0, v1);
            t.vertex(x1, y0, z1);
            t.tex(u1, v1);
            t.vertex(x1, y0, z0);
            t.tex(u1, v0);
            t.vertex(x1, y1, z0);
            t.tex(u0, v0);
            t.vertex(x1, y1, z1);
        }
    }

    pub fn render_face(&self, t: &mut Tesselator, x: i32, y: i32, z: i32, face: i32) {
        let x0 = x as f32 + 0.0;
        let x1 = x as f32 + 1.0;
        let y0 = y as f32 + 0.0;
        let y1 = y as f32 + 1.0;
        let z0 = z as f32 + 0.0;
        let z1 = z as f32 + 1.0;
        if face == 0 {
            t.vertex(x0, y0, z1);
            t.vertex(x0, y0, z0);
            t.vertex(x1, y0, z0);
            t.vertex(x1, y0, z1);
        }
        if face == 1 {
            t.vertex(x1, y1, z1);
            t.vertex(x1, y1, z0);
            t.vertex(x0, y1, z0);
            t.vertex(x0, y1, z1);
        }
        if face == 2 {
            t.vertex(x0, y1, z0);
            t.vertex(x1, y1, z0);
            t.vertex(x1, y0, z0);
            t.vertex(x0, y0, z0);
        }
        if face == 3 {
            t.vertex(x0, y1, z1);
            t.vertex(x0, y0, z1);
            t.vertex(x1, y0, z1);
            t.vertex(x1, y1, z1);
        }
        if face == 4 {
            t.vertex(x0, y1, z1);
            t.vertex(x0, y1, z0);
            t.vertex(x0, y0, z0);
            t.vertex(x0, y0, z1);
        }
        if face == 5 {
            t.vertex(x1, y0, z1);
            t.vertex(x1, y0, z0);
            t.vertex(x1, y1, z0);
            t.vertex(x1, y1, z1);
        }
    }
}
