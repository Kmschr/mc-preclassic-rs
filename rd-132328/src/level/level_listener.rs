pub trait LevelListener {
    fn tile_changed(&mut self, x: i32, y: i32, z: i32);
    fn light_column_changed(&mut self, x: i32, z: i32, y0: i32, y1: i32);
    fn all_changed(&mut self);
}
