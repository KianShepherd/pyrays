use std::rc::Rc;

use crate::aabb::AABB;
use crate::hittables::HittableObject;
use crate::ray::Ray;
use glam::Vec3A;

const MAX_IN_OCTREE: usize = 12;
const MAX_DEPTH: usize = 10;

#[derive(Debug, Clone)]
pub struct OcTree {
    bounding_box: AABB,
    hittables: Vec<HittableObject>,
    sub_boxes: Vec<OcTree>,
    is_leaf: bool,
}

#[derive(Debug, Clone)]
pub struct OcTreeBuilder {
    bounding_box: AABB,
    hittables: Vec<Rc<HittableObject>>,
    sub_boxes: Vec<OcTreeBuilder>,
    is_leaf: bool,
}

impl OcTreeBuilder {
    pub fn new(objs: Vec<Rc<HittableObject>>) -> OcTree {
        let mut min = Vec3A::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
        let mut max = Vec3A::new(
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
        );
        objs.iter().for_each(|o| match *o.as_ref() {
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

        let mut builder = Self::internal_new(AABB::new(min - 0.1, max), objs, 0);
        OcTree::new(&mut builder)
    }

    fn internal_new(bbox: AABB, objs: Vec<Rc<HittableObject>>, depth: usize) -> Self {
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
                    arr.push(OcTreeBuilder::internal_new(
                        *sub_aabb,
                        objs.iter().fold(vec![], |mut new_objs, o| {
                            match *o.as_ref() {
                                HittableObject::SphereObj(s) => {
                                    if sub_aabb.overlaps(&s.get_aabb()) {
                                        new_objs.push(o.clone());
                                    }
                                }
                                HittableObject::TriangleObj(t) => {
                                    if sub_aabb.overlaps(&t.get_aabb()) {
                                        new_objs.push(o.clone());
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
}

impl OcTree {
    pub fn new(ot: &mut OcTreeBuilder) -> Self {
        if ot.is_leaf {
            let leafs = {
                let mut _leafs = vec![];
                for i in 0..(ot.hittables.len()) {
                    _leafs.push(*ot.hittables[i].to_owned());
                }
                ot.hittables.clear();
                _leafs
            };
            Self {
                bounding_box: ot.bounding_box,
                hittables: leafs,
                sub_boxes: vec![],
                is_leaf: true,
            }
        } else {
            Self {
                bounding_box: ot.bounding_box,
                hittables: vec![],
                sub_boxes: ot.sub_boxes.iter_mut().map(|x| OcTree::new(x)).collect(),
                is_leaf: false,
            }
        }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Vec<HittableObject> {
        if self.bounding_box.hit(ray, t_min, t_max) {
            if self.is_leaf {
                return self.hittables.clone();
            } else {
                return self.sub_boxes.iter().fold(vec![], |mut arr, b| {
                    arr.extend(b.hit(ray, t_min, t_max));
                    arr
                });
            }
        }
        vec![]
    }
}
