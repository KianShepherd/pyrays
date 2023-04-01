use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Vec3A,
    pub normal: Vec3A,
    pub t: f32,
    pub material: Material,
    pub front_face: bool,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn get_p(&self) -> Vec3A {
        self.p
    }
    pub fn get_t(&self) -> f32 {
        self.t
    }
    pub fn get_normal(&self) -> Vec3A {
        self.normal
    }
    pub fn get_front_face(&self) -> bool {
        self.front_face
    }

    pub fn set_rec(&mut self, r: &HitRecord) {
        self.p = r.p;
        self.t = r.t;
        self.normal = r.normal;
        self.front_face = r.front_face;
        self.material = r.material;
    }
}

pub fn set_face_normal(ray: &Ray, outward_normal: Vec3A) -> (bool, Vec3A) {
    let front_face = ray.direction().dot(outward_normal) < 0.0;
    let normal = if front_face {
        outward_normal
    } else {
        -outward_normal
    };
    (front_face, normal)
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
