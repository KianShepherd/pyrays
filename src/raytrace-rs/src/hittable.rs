use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub p: Option<Vec3>,
    pub normal: Option<Vec3>,
    pub t: Option<f64>,
    pub material: Option<Material>,
    front_face: Option<bool>,
}

#[allow(dead_code)]
impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: None,
            normal: None,
            t: None,
            front_face: None,
            material: None,
        }
    }
    pub fn get_p(&self) -> Option<Vec3> {
        self.p
    }
    pub fn get_t(&self) -> Option<f64> {
        self.t
    }
    pub fn get_normal(&self) -> Option<Vec3> {
        self.normal
    }
    pub fn get_front_face(&self) -> Option<bool> {
        self.front_face
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = Some(ray.direction().dot(outward_normal) < 0.0);
        if self.front_face.unwrap() {
            self.normal = Some(outward_normal);
        } else {
            self.normal = Some(-outward_normal);
        }
    }

    pub fn set_rec(&mut self, r: &HitRecord) {
        self.p = r.p;
        self.t = r.t;
        self.normal = r.normal;
        self.front_face = r.front_face;
        self.material = r.material;
    }
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hitrecord() -> Result<(), String> {
        let mut hitrec = HitRecord {
            p: Some(Vec3::new(1.0, 1.0, 1.0)),
            normal: Some(Vec3::new(1.0, 1.0, 1.0)),
            t: Some(1.0),
            front_face: Some(true),
            material: None,
        };
        assert_eq!(hitrec.get_p(), Some(Vec3::new(1.0, 1.0, 1.0)));
        assert_eq!(hitrec.get_normal(), Some(Vec3::new(1.0, 1.0, 1.0)));
        assert_eq!(hitrec.get_t(), Some(1.0));
        assert_eq!(hitrec.get_front_face(), Some(true));
        Ok(())
    }
}
