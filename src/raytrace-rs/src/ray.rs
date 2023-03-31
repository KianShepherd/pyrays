use glam::Vec3A;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vec3A,
    direction: Vec3A,
}

impl Ray {
    pub fn new(orig: Vec3A, dir: Vec3A) -> Ray {
        Ray {
            origin: orig,
            direction: dir,
        }
    }
    pub fn origin(&self) -> Vec3A {
        self.origin
    }
    pub fn direction(&self) -> Vec3A {
        self.direction
    }
    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + self.direction * t
    }
}
