use std::collections::HashMap;
use std::sync::Mutex;

use crate::glu::*;

use image::io::Reader as ImageReader;

lazy_static! {
    static ref ID_MAP: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

pub fn load_texture(resource_name: &str, mode: i32) -> i32 {
    if ID_MAP.lock().unwrap().contains_key(resource_name) {
        return *ID_MAP.lock().unwrap().get(resource_name).unwrap();
    }
    let mut ib: [GLuint; 1] = [0; 1];
    unsafe {
        glGenTextures(1, ib.as_mut_ptr());
    }
    let id = ib[0];
    ID_MAP
        .lock()
        .unwrap()
        .insert(resource_name.to_string(), id as i32);

    println!("{} -> {}", resource_name, id);

    unsafe {
        glBindTexture(GL_TEXTURE_2D, id);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, mode);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, mode);
        let img = ImageReader::open(resource_name).unwrap().decode().unwrap();
        let img = img.to_rgba8();

        let (w, h) = img.dimensions();
        let pixels = img.as_raw().clone();

        gluBuild2DMipmaps(
            GL_TEXTURE_2D,
            GL_RGBA as GLint,
            w as GLint,
            h as GLint,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            pixels.as_ptr() as *const GLvoid,
        );
    }

    id as i32
}
