use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Mutex;

use crate::glu::*;

use image::io::Reader as ImageReader;

lazy_static! {
    static ref ID_MAP: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

static LAST_ID: AtomicI32 = AtomicI32::new(-9999999);

pub fn load_texture(resource_name: &str, mode: i32) -> i32 {
    if ID_MAP.lock().unwrap().contains_key(resource_name) {
        return *ID_MAP.lock().unwrap().get(resource_name).unwrap();
    }
    let mut ib: [GLuint; 1] = [0; 1];
    unsafe {
        glGenTextures(1, ib.as_mut_ptr());
    }
    let id = ib[0];
    bind(id as GLint);
    unsafe {
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, mode);
        glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, mode);
        let img = ImageReader::open(resource_name).unwrap().decode().unwrap();
        let img = img.as_rgb8().unwrap();
        let (w, h) = img.dimensions();
        let mut pixels = img.as_raw().clone();
        for i in 0..(w * h) {
            pixels.insert(i as usize * 4 + 3, 0);
        }
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

pub fn bind(id: i32) {
    if id != LAST_ID.load(Ordering::SeqCst) {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, id as GLuint);
        }
        LAST_ID.store(id, Ordering::SeqCst)
    }
}
