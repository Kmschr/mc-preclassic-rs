#[derive(Debug)]
pub struct AABB {
    epsilon: f32,
    pub x0: f32,
    pub y0: f32,
    pub z0: f32,
    pub x1: f32,
    pub y1: f32,
    pub z1: f32,
}

impl AABB {
    pub fn new(x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) -> AABB {
        AABB {
            epsilon: 0.0,
            x0,
            y0,
            z0,
            x1,
            y1,
            z1,
        }
    }

    pub fn expand(&self, xa: f32, ya: f32, za: f32) -> AABB {
        let mut _x0 = self.x0;
        let mut _y0 = self.y0;
        let mut _z0 = self.z0;
        let mut _x1 = self.x1;
        let mut _y1 = self.y1;
        let mut _z1 = self.z1;
        if xa < 0.0 {
            _x0 += xa;
        }
        if xa > 0.0 {
            _x1 += xa;
        }
        if ya < 0.0 {
            _y0 += ya;
        }
        if ya > 0.0 {
            _y1 += ya;
        }
        if za < 0.0 {
            _z0 += za;
        }
        if za > 0.0 {
            _z1 += za;
        }
        AABB::new(_x0, _y0, _z0, _x1, _y1, _z1)
    }

    pub fn grow(&self, xa: f32, ya: f32, za: f32) -> AABB {
        let mut _x0 = self.x0 - xa;
        let mut _y0 = self.y0 - ya;
        let mut _z0 = self.z0 - za;
        let mut _x1 = self.x1 + xa;
        let mut _y1 = self.y1 + ya;
        let mut _z1 = self.z1 + za;
        AABB::new(_x0, _y0, _z0, _x1, _y1, _z1)
    }

    pub fn clip_x_collide(&self, c: &AABB, xa: f32) -> f32 {
        if c.y1 <= self.y0 || c.y0 >= self.y1 {
            return xa;
        }
        if c.z1 <= self.z0 || c.z0 >= self.z1 {
            return xa;
        }
        let mut xa = xa;
        let max = self.x0 - c.x1 - self.epsilon;
        if xa > 0.0 && c.x1 <= self.x0 && max < xa {
            xa = max;
        }
        let max = self.x1 - c.x0 + self.epsilon;
        if xa < 0.0 && c.x0 >= self.x1 && max > xa {
            xa = max;
        }
        xa
    }

    pub fn clip_y_collide(&self, c: &AABB, ya: f32) -> f32 {
        if c.x1 <= self.x0 || c.x0 >= self.x1 {
            return ya;
        }
        if c.z1 <= self.z0 || c.z0 >= self.z1 {
            return ya;
        }
        let mut ya = ya;
        let max = self.y0 - c.y1 - self.epsilon;
        if ya > 0.0 && c.y1 <= self.y0 && max < ya {
            ya = max;
        }
        let max = self.y1 - c.y0 + self.epsilon;
        if ya < 0.0 && c.y0 >= self.y1 && max > ya {
            ya = max;
        }
        ya
    }

    pub fn clip_z_collide(&self, c: &AABB, za: f32) -> f32 {
        if c.x1 <= self.x0 || c.x0 >= self.x1 {
            return za;
        }
        if c.y1 <= self.y0 || c.y0 >= self.y1 {
            return za;
        }
        let mut za = za;
        let max = self.z0 - c.z1 - self.epsilon;
        if za > 0.0 && c.z1 <= self.z0 && max < za {
            za = max;
        }
        let max = self.z1 - c.z0 + self.epsilon;
        if za < 0.0 && c.z0 >= self.z1 && max > za {
            za = max;
        }
        za
    }

    pub fn move_(&mut self, xa: f32, ya: f32, za: f32) {
        self.x0 += xa;
        self.y0 += ya;
        self.z0 += za;
        self.x1 += xa;
        self.y1 += ya;
        self.z1 += za;
    }
}
