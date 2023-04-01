use crate::hittable::{self, HitRecord};
use crate::material::Material;
use crate::ray::Ray;
use crate::{aabb::AABB, hittable::set_face_normal};
use glam::Vec3A;
use std::intrinsics::{fadd_fast, fdiv_fast, fmul_fast};

#[derive(Debug, Clone)]
pub struct Triangle {
    points: [Vec3A; 3],
    normal: Vec3A,
    material: Material,
    culling: bool,
    aabb: Option<AABB>,
}

impl Triangle {
    pub fn new(
        point1: Vec3A,
        point2: Vec3A,
        point3: Vec3A,
        mat: Material,
        cull_back_face: bool,
    ) -> Triangle {
        let points_ = [point1, point2, point3];
        let normal_ = {
            let a = point2 - point1;
            let b = point3 - point1;
            a.cross(b).normalize()
        };

        let mut t = Triangle {
            points: points_,
            normal: normal_,
            material: mat,
            culling: cull_back_face,
            aabb: None,
        };
        t.aabb = Some(t.get_aabb());
        t
    }

    pub fn get_aabb(&self) -> AABB {
        match self.aabb {
            Some(a) => a,
            None => {
                let mut min =
                    Vec3A::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
                let mut max = Vec3A::new(
                    std::f32::NEG_INFINITY,
                    std::f32::NEG_INFINITY,
                    std::f32::NEG_INFINITY,
                );
                let mut a = 0;
                let mut b = 0;
                while a < 3 {
                    while b < 3 {
                        if self.points[b][a] < min[a] {
                            min[a] = self.points[b][a]
                        }
                        if self.points[b][a] > max[a] {
                            max[a] = self.points[b][a]
                        }
                        b += 1;
                    }
                    b = 0;
                    a += 1;
                }
                AABB::new(min, max)
            }
        }
    }
}

impl hittable::Hittable for Triangle {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        unsafe {
            let vertex0 = self.points[0];
            let vertex1 = self.points[1];
            let vertex2 = self.points[2];

            let edge1 = vertex1 - vertex0;
            let edge2 = vertex2 - vertex0;

            let h = ray.direction().cross(edge2);
            let a = edge1.dot(h);
            if self.culling && a < t_min {
                return None;
            }

            let f = fdiv_fast(1.0, a);
            let s = ray.origin() - vertex0;
            let u = fmul_fast(f, s.dot(h));
            if !(0.0..=1.0).contains(&u) {
                return None;
            }

            let q = s.cross(edge1);
            let v = fmul_fast(f, ray.direction().dot(q));
            if v < 0.0 || fadd_fast(u, v) > 1.0 {
                return None;
            }

            let t = fmul_fast(f, edge2.dot(q));
            if t > t_max || t < t_min {
                return None;
            }
            let intersection_point = ray.origin() + ray.direction() * t;

            let (front_face, normal) = set_face_normal(ray, self.normal);

            Some(HitRecord {
                p: intersection_point,
                normal,
                t,
                material: self.material,
                front_face,
            })
        }
    }
}
