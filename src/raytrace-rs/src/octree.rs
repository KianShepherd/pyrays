use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittables::HittableObject;
use crate::ray::Ray;
use glam::Vec3A;

const MAX_IN_OCTREE: usize = 20;
const MAX_DEPTH: usize = 10;

#[derive(Debug, Clone)]
pub struct OcTree<'a> {
    bounding_box: AABB,
    hittables: Vec<&'a HittableObject>,
    sub_boxes: Vec<OcTree<'a>>,
    is_leaf: bool,
}

impl<'a> OcTree<'a> {
    pub fn new<'b>(objs: &'a Vec<HittableObject>) -> OcTree<'b>
    where
        'a: 'b,
    {
        let mut min = Vec3A::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
        let mut max = Vec3A::new(
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
        );
        objs.iter().for_each(|o| match o {
            HittableObject::SphereObj(s) => {
                for a in 0..3 {
                    if s.get_aabb().min[a] < min[a] {
                        min[a] = s.get_aabb().min[a];
                    }
                    if s.get_aabb().max[a] > max[a] {
                        max[a] = s.get_aabb().max[a];
                    }
                }
            }
            HittableObject::TriangleObj(t) => {
                for a in 0..3 {
                    if t.get_aabb().min[a] < min[a] {
                        min[a] = t.get_aabb().min[a];
                    }
                    if t.get_aabb().max[a] > max[a] {
                        max[a] = t.get_aabb().max[a];
                    }
                }
            }
        });

        Self::internal_new(
            AABB::new(min - 0.1, max),
            objs.iter().map(|x| x).collect(),
            0,
        )
    }

    fn internal_new<'b>(bbox: AABB, objs: Vec<&'b HittableObject>, depth: usize) -> Self
    where
        'a: 'b,
        'b: 'a,
    {
        let diff = bbox.max - bbox.min;
        if objs.len() > MAX_IN_OCTREE
            && ((diff.x > 1.0) || (diff.y > 1.0) || (diff.z > 1.0))
            && bbox.min.distance(bbox.max) > 1.0
            && depth < MAX_DEPTH
        {
            let min = bbox.min;
            let max = bbox.max;
            let midpoint = Vec3A::new(
                (min.x + max.x) / 2.0,
                (min.y + max.y) / 2.0,
                (min.z + max.z) / 2.0,
            );
            let sub_boxes_aabb = vec![
                AABB::new(
                    Vec3A::new(min.x, min.y, min.z),
                    Vec3A::new(midpoint.x, midpoint.y, midpoint.z),
                ),
                AABB::new(
                    Vec3A::new(min.x, midpoint.y, min.z),
                    Vec3A::new(midpoint.x, max.y, midpoint.z),
                ),
                AABB::new(
                    Vec3A::new(midpoint.x, min.y, min.z),
                    Vec3A::new(max.x, midpoint.y, midpoint.z),
                ),
                AABB::new(
                    Vec3A::new(midpoint.x, midpoint.y, min.z),
                    Vec3A::new(max.x, max.y, midpoint.z),
                ),
                AABB::new(
                    Vec3A::new(min.x, min.y, midpoint.z),
                    Vec3A::new(midpoint.x, midpoint.y, max.z),
                ),
                AABB::new(
                    Vec3A::new(min.x, midpoint.y, midpoint.z),
                    Vec3A::new(midpoint.x, max.y, max.z),
                ),
                AABB::new(
                    Vec3A::new(midpoint.x, min.y, midpoint.z),
                    Vec3A::new(max.x, midpoint.y, max.z),
                ),
                AABB::new(
                    Vec3A::new(midpoint.x, midpoint.y, midpoint.z),
                    Vec3A::new(max.x, max.y, max.z),
                ),
            ];
            Self {
                bounding_box: bbox,
                hittables: vec![],
                sub_boxes: sub_boxes_aabb.iter().fold(vec![], |mut arr, sub_aabb| {
                    arr.push(OcTree::internal_new(
                        *sub_aabb,
                        objs.iter().fold(vec![], |mut new_objs, o| {
                            match o {
                                HittableObject::SphereObj(s) => {
                                    if sub_aabb.overlaps(&s.get_aabb()) {
                                        new_objs.push(o);
                                    }
                                }
                                HittableObject::TriangleObj(t) => {
                                    if sub_aabb.overlaps(&t.get_aabb()) {
                                        new_objs.push(o);
                                    }
                                }
                            }
                            new_objs
                        }),
                        depth + 1,
                    ));
                    arr
                }),
                is_leaf: false,
            }
        } else {
            Self {
                bounding_box: bbox,
                hittables: objs,
                sub_boxes: vec![],
                is_leaf: true,
            }
        }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bounding_box.hit(ray, t_min, t_max) {
            if self.is_leaf {
                let mut rec = None;
                let mut closest = t_max;
                self.hittables.iter().for_each(|hittable| match hittable {
                    HittableObject::SphereObj(s) => {
                        if let Some(r) = s.hit(ray, t_min, closest) {
                            closest = r.get_t();
                            rec = Some(r);
                        }
                    }
                    HittableObject::TriangleObj(t) => {
                        if let Some(r) = t.hit(ray, t_min, closest) {
                            closest = r.get_t();
                            rec = Some(r);
                        }
                    }
                });
                rec
            } else {
                let mut rec = None;
                let mut closest = t_max;
                self.sub_boxes.iter().for_each(|b| {
                    if let Some(r) = b.hit(ray, t_min, closest) {
                        closest = r.get_t();
                        rec = Some(r);
                    }
                });
                rec
            }
        } else {
            None
        }
    }
}
