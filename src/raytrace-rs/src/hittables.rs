use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Hittables {
    pub lights: Vec<Vec3>,
    pub hittables: Vec<Box<dyn Hittable + Send + Sync>>,
}

#[allow(dead_code)]
impl Hittables {
    pub fn push(&mut self, hittable_: Box<dyn Hittable + Send + Sync>) {
        let _ = &self.hittables.push(hittable_);
    }

    pub fn push_light(&mut self, light_position: Vec3) {
        let _ = &self.lights.push(light_position);
    }

    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut temp_rec = HitRecord::new();
        let mut closest = t_max;

        self.hittables.iter().for_each(|hittable| {
            if hittable.hit(ray, t_min, closest, &mut temp_rec) {
                hit_anything = true;
                closest = temp_rec.get_t().unwrap();
                rec.set_rec(&temp_rec);
            }
        });
        hit_anything
    }
}
