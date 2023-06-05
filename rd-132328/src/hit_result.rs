#[derive(Debug)]
pub struct HitResult {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub o: i32,
    pub f: i32,
}

impl HitResult {
    pub fn new(x: i32, y: i32, z: i32, o: i32, f: i32) -> HitResult {
        HitResult { x, y, z, o, f }
    }
}
