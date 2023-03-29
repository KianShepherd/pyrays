use crate::hittable;
use crate::material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::intrinsics::{fadd_fast, fdiv_fast, fmul_fast, fsub_fast};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: material::Material,
}

#[allow(dead_code)]
impl Sphere {
    pub fn new(cen: Vec3, rad: f32, mat: material::Material) -> Sphere {
        Sphere {
            center: cen,
            radius: rad,
            material: mat,
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut hittable::HitRecord) -> bool {
        unsafe {
            let oc: Vec3 = &ray.origin() - &self.center;
            let a = ray.direction().length_squared();
            let half_b = oc.dot(&ray.direction());
            let c = oc.length_squared() - fmul_fast(self.radius, self.radius);
            let discriminant = fsub_fast(fmul_fast(half_b, half_b), fmul_fast(a, c));
            if discriminant > 0.0 {
                let root = discriminant.sqrt();

                let temp1 = fdiv_fast(fsub_fast(-half_b, root), a);
                let temp2 = fdiv_fast(fadd_fast(-half_b, root), a);

                if (temp1 < t_max && temp1 > t_min) || (temp2 < t_max && temp2 > t_min) {
                    rec.t = Some(temp1);
                    rec.p = Some(ray.at(rec.t.unwrap()));
                    let outward_normal =
                        &(&rec.p.unwrap() - &self.center) * fdiv_fast(1.0, self.radius);
                    rec.set_face_normal(ray, &outward_normal);
                    rec.material = Some(self.material);

                    return true;
                }
            }

            false
        }
    }
}
