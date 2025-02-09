use crate::hittables::HittableObject;
use crate::ray::Ray;
use glam::Vec3A;
use std::intrinsics::{fdiv_fast, fmul_fast, fsub_fast};

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone)]
pub struct AABB {
    pub min: Vec3A,
    pub max: Vec3A,
}

fn overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
    (min1 <= min2 && max1 >= max2)
        || (min2 <= min1 && max2 >= max1)
        || (max1 >= min2 && max1 <= max2)
        || (max2 >= min1 && max2 <= max1)
}

#[allow(dead_code)]
impl AABB {
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> bool {
        unsafe {
            let mut t_min2 = t_min;
            let mut t_max2 = t_max;
            let mut res = true;
            let mut a = 0;
            while a < 3 {
                let inv_d = fdiv_fast(1.0, ray.direction()[a]);
                let t0 = fmul_fast(fsub_fast(self.min[a], ray.origin()[a]), inv_d);
                let t1 = fmul_fast(fsub_fast(self.max[a], ray.origin()[a]), inv_d);
                let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };
                t_min2 = if t0 > t_min2 { t0 } else { t_min2 };
                t_max2 = if t1 < t_max2 { t1 } else { t_max2 };
                if t_max2 <= t_min2 {
                    res = false;
                    break;
                }
                a += 1;
            }
            if t_min2 < t_min || t_max2 > t_max {
                false
            } else {
                res
            }
        }
    }

    pub fn overlaps(&self, other: &AABB) -> bool {
        let mut a = 0;
        while a < 3 {
            if !overlap(self.min[a], self.max[a], other.min[a], other.max[a]) {
                return false;
            }
            a += 1;
        }
        true
    }

    pub fn inside(&self, obj: HittableObject) -> bool {
        match obj {
            HittableObject::SphereObj(s) => s.get_aabb().overlaps(self),
            HittableObject::TriangleObj(t) => t.get_aabb().overlaps(self),
        }
    }
}
