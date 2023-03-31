use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittables::HittableObject;
use crate::ray::Ray;
use glam::Vec3A;

const MAX_IN_OCTREE: usize = 20;

#[derive(Debug, Clone)]
pub struct OcTree {
    bounding_box: AABB,
    hittables: Vec<HittableObject>,
    sub_boxes: Vec<OcTree>,
    is_leaf: bool,
}

impl OcTree {
    pub fn new(objs: Vec<HittableObject>) -> Self {
        let mut min = Vec3A::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
        let mut max = Vec3A::new(
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
        );
        for i in 0..objs.len() {
            match &objs[i] {
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
            }
        }
        let bounding_box = AABB::new(min, max);
        if objs.len() > MAX_IN_OCTREE {
            let midpoint = min + ((max - min) / 2.0);
            let mut sub_boxes_aabb = vec![];
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(min.x, min.y, min.z),
                Vec3A::new(midpoint.x, midpoint.y, midpoint.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(min.x, midpoint.y, min.z),
                Vec3A::new(midpoint.x, max.y, midpoint.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(midpoint.x, min.y, min.z),
                Vec3A::new(max.x, midpoint.y, midpoint.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(midpoint.x, midpoint.y, min.z),
                Vec3A::new(max.x, max.y, midpoint.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(min.x, min.y, midpoint.z),
                Vec3A::new(midpoint.x, midpoint.y, max.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(min.x, midpoint.y, midpoint.z),
                Vec3A::new(midpoint.x, max.y, max.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(midpoint.x, min.y, midpoint.z),
                Vec3A::new(max.x, midpoint.y, max.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(midpoint.x, midpoint.y, midpoint.z),
                Vec3A::new(max.x, max.y, max.z),
            ));
            let mut sub_boxes = vec![];
            for i in 0..sub_boxes_aabb.len() {
                let mut tmp_obj_vec = vec![];
                for o in &objs {
                    match o {
                        HittableObject::SphereObj(s) => {
                            if sub_boxes_aabb[i].overlaps(&s.get_aabb()) {
                                tmp_obj_vec.push(o.clone());
                            }
                        }
                        HittableObject::TriangleObj(t) => {
                            if sub_boxes_aabb[i].overlaps(&t.get_aabb()) {
                                tmp_obj_vec.push(o.clone());
                            }
                        }
                    }
                }
                sub_boxes.push(OcTree::internal_new(
                    sub_boxes_aabb[i].clone(),
                    tmp_obj_vec.clone(),
                ));
            }
            Self {
                bounding_box,
                hittables: vec![],
                sub_boxes,
                is_leaf: false,
            }
        } else {
            Self {
                bounding_box,
                hittables: objs,
                sub_boxes: vec![],
                is_leaf: true,
            }
        }
    }

    fn internal_new(bbox: AABB, objs: Vec<HittableObject>) -> Self {
        if objs.len() > MAX_IN_OCTREE {
            let min = bbox.min;
            let max = bbox.max;
            let midpoint = min + ((max - min) / 2.0);
            let mut sub_boxes_aabb = vec![];
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(min.x, min.y, min.z),
                Vec3A::new(midpoint.x, midpoint.y, midpoint.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(min.x, midpoint.y, min.z),
                Vec3A::new(midpoint.x, max.y, midpoint.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(midpoint.x, min.y, min.z),
                Vec3A::new(max.x, midpoint.y, midpoint.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(midpoint.x, midpoint.y, min.z),
                Vec3A::new(max.x, max.y, midpoint.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(min.x, min.y, midpoint.z),
                Vec3A::new(midpoint.x, midpoint.y, max.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(min.x, midpoint.y, midpoint.z),
                Vec3A::new(midpoint.x, max.y, max.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(midpoint.x, min.y, midpoint.z),
                Vec3A::new(max.x, midpoint.y, max.z),
            ));
            sub_boxes_aabb.push(AABB::new(
                Vec3A::new(midpoint.x, midpoint.y, midpoint.z),
                Vec3A::new(max.x, max.y, max.z),
            ));
            let mut sub_boxes = vec![];
            for i in 0..sub_boxes_aabb.len() {
                let mut tmp_obj_vec = vec![];
                for o in &objs {
                    match o {
                        HittableObject::SphereObj(s) => {
                            if sub_boxes_aabb[i].overlaps(&s.get_aabb()) {
                                tmp_obj_vec.push(o.clone());
                            }
                        }
                        HittableObject::TriangleObj(t) => {
                            if sub_boxes_aabb[i].overlaps(&t.get_aabb()) {
                                tmp_obj_vec.push(o.clone());
                            }
                        }
                    }
                }
                sub_boxes.push(OcTree::internal_new(
                    sub_boxes_aabb[i].clone(),
                    tmp_obj_vec.clone(),
                ));
            }
            Self {
                bounding_box: bbox,
                hittables: vec![],
                sub_boxes,
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

    pub fn hit(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> Option<Vec<HittableObject>> {
        if self.bounding_box.hit(ray, t_min, t_max) {
            if self.is_leaf {
                return Some(self.hittables.clone());
            } else {
                let v = self.sub_boxes.iter().fold(vec![], |mut arr, b| {
                    match b.hit(ray, t_min, t_max, rec) {
                        Some(hs) => {
                            arr.extend(hs);
                        }
                        None => {}
                    }
                    arr
                });
                return if v.len() == 0 { None } else { Some(v.clone()) };
            }
        }
        None
    }
}
