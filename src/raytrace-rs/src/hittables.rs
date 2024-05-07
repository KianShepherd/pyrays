use crate::hittable::HitRecord;
use crate::octree::OcTree;
use crate::ray::Ray;
use crate::Sphere;
use crate::Triangle;
use glam::Vec3A;

#[derive(Debug, Copy, Clone)]
pub enum HittableObject {
    SphereObj(Sphere),
    TriangleObj(Triangle),
}

pub struct Hittables<'a> {
    pub lights: Vec<Vec3A>,
    hittables: OcTree<'a>,
}

fn conv_py_vec(vector: Vec<f32>) -> Vec3A {
    Vec3A::new(vector[0], vector[1], vector[2])
}

#[allow(dead_code)]
impl<'a> Hittables<'a> {
    pub fn new<'b>(lights: &[Vec<f32>], objects: &'b Vec<HittableObject>) -> Self
    where
        'b: 'a,
    {
        let mut _lights = vec![];
        lights.iter().for_each(|obj| {
            _lights.push(conv_py_vec(obj.clone()));
        });

        Self {
            lights: _lights,
            hittables: OcTree::new(objects),
        }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hittables.hit(ray, t_min, t_max)
    }
}
