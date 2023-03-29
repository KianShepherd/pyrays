use crate::hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::intrinsics::{fadd_fast, fdiv_fast, fmul_fast};

pub struct Triangle {
    points: Vec<Vec3>,
    normal: Vec3,
    material: Material,
    culling: bool,
}

impl Triangle {
    pub fn new(
        point1: Vec3,
        point2: Vec3,
        point3: Vec3,
        mat: Material,
        cull_back_face: bool,
    ) -> Triangle {
        let points_ = vec![point1, point2, point3];
        let normal_ = {
            let a = &point2 - &point1;
            let b = &point3 - &point1;
            a.cross(&b).unit_vector()
        };

        Triangle {
            points: points_,
            normal: normal_,
            material: mat,
            culling: cull_back_face,
        }
    }
}

impl hittable::Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut hittable::HitRecord) -> bool {
        unsafe {
            let vertex0 = *self.points.get(0).unwrap();
            let vertex1 = *self.points.get(1).unwrap();
            let vertex2 = *self.points.get(2).unwrap();

            let edge1 = &vertex1 - &vertex0;
            let edge2 = &vertex2 - &vertex0;

            let h = ray.direction().cross(&edge2);
            let a = edge1.dot(&h);
            if self.culling && a < t_min {
                return false;
            }

            let f = fdiv_fast(1.0, a);
            let s = &ray.origin() - &vertex0;
            let u = fmul_fast(f, s.dot(&h));
            if !(0.0..=1.0).contains(&u) {
                return false;
            }

            let q = s.cross(&edge1);
            let v = fmul_fast(f, ray.direction().dot(&q));
            if v < 0.0 || fadd_fast(u, v) > 1.0 {
                return false;
            }

            let t = fmul_fast(f, edge2.dot(&q));
            if t > t_max || t < t_min {
                return false;
            }
            let intersection_point = &ray.origin() + &(&ray.direction() * t);

            rec.t = Some(t);
            rec.p = Some(intersection_point);
            rec.set_face_normal(ray, &self.normal);
            rec.material = Some(self.material);

            true
        }
    }
}
