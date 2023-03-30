use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittables::HittableObject;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::ptr;

const MAX_IN_OCTREE: usize = 10;

#[derive(Debug)]
pub struct OcTree {
    base: *mut OcNode,
}

#[derive(Debug)]
struct OcNode {
    bounding_box: AABB,
    is_leaf: bool,
    hittables: Vec<HittableObject>,
    parent: *mut OcNode,
    sub_boxes: [*mut OcNode; 8],
}

unsafe impl Sync for OcTree {}

impl OcTree {
    pub fn new(objs: Vec<HittableObject>) -> Self {
        let mut min = Vec3::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
        let mut max = Vec3::new(
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
            std::f32::NEG_INFINITY,
        );
        for i in 0..objs.len() {
            match &objs[i] {
                HittableObject::SphereObj(s) => {
                    for a in 0..3 {
                        if s.get_aabb().min.get_idx(a) < min.get_idx(a) {
                            min.set_idx(a, s.get_aabb().min.get_idx(a));
                        }
                        if s.get_aabb().max.get_idx(a) > max.get_idx(a) {
                            max.set_idx(a, s.get_aabb().max.get_idx(a));
                        }
                    }
                }
                HittableObject::TriangleObj(t) => {
                    for a in 0..3 {
                        if t.get_aabb().min.get_idx(a) < min.get_idx(a) {
                            min.set_idx(a, t.get_aabb().min.get_idx(a));
                        }
                        if t.get_aabb().max.get_idx(a) > max.get_idx(a) {
                            max.set_idx(a, t.get_aabb().max.get_idx(a));
                        }
                    }
                }
            }
        }
        let bounding_box = AABB::new(min, max);
        let mut base = OcNode::new(bounding_box);
        for obj in objs {
            base.insert_obj(obj);
        }
        Self {
            base: Box::<OcNode>::into_raw(Box::new(base)),
        }
    }

    pub fn hit(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> Option<Vec<HittableObject>> {
        unsafe { self.base.as_ref()?.hit(ray, t_min, t_max, rec) }
    }
}

impl OcNode {
    fn new(bb: AABB) -> Self {
        Self {
            bounding_box: bb,
            is_leaf: true,
            hittables: vec![],
            parent: ptr::null_mut(),
            sub_boxes: [ptr::null_mut(); 8],
        }
    }

    fn insert_obj(&mut self, obj: HittableObject) {
        stacker::maybe_grow(128 * 1024, 1024 * 1024, || {
            if self.is_leaf {
                self.hittables.push(obj);
                if self.hittables.len() > MAX_IN_OCTREE {
                    self.is_leaf = false;
                    self.subdivide();

                    self.hittables
                        .clone()
                        .iter()
                        .for_each(|o| self.insert_obj(o.clone()));
                    self.hittables = vec![];
                }
            } else {
                self.sub_boxes.iter().for_each(|sb| unsafe {
                    match &obj {
                        HittableObject::SphereObj(s) => {
                            if s.get_aabb().overlaps(&(*(*sb)).bounding_box) {
                                sb.as_mut()
                                    .expect("failed to get mut sub box in insert obj")
                                    .insert_obj(obj.clone());
                            }
                        }
                        HittableObject::TriangleObj(t) => {
                            if t.get_aabb().overlaps(&(*(*sb)).bounding_box) {
                                sb.as_mut()
                                    .expect("failed to get mut sub box in insert obj")
                                    .insert_obj(obj.clone());
                            }
                        }
                    }
                });
            }
        });
    }

    fn subdivide(&mut self) {
        let min = self.bounding_box.min;
        let max = self.bounding_box.max;
        let midpoint = &min + &(&(&max - &min) / 2.0);
        self.setnode(
            0,
            node(AABB::new(
                Vec3::new(min.x(), min.y(), min.z()),
                Vec3::new(midpoint.x(), midpoint.y(), midpoint.z()),
            )),
        );
        self.setnode(
            1,
            node(AABB::new(
                Vec3::new(min.x(), midpoint.y(), min.z()),
                Vec3::new(midpoint.x(), max.y(), midpoint.z()),
            )),
        );
        self.setnode(
            2,
            node(AABB::new(
                Vec3::new(midpoint.x(), min.y(), min.z()),
                Vec3::new(max.x(), midpoint.y(), midpoint.z()),
            )),
        );
        self.setnode(
            3,
            node(AABB::new(
                Vec3::new(midpoint.x(), midpoint.y(), min.z()),
                Vec3::new(max.x(), max.y(), midpoint.z()),
            )),
        );
        self.setnode(
            4,
            node(AABB::new(
                Vec3::new(min.x(), min.y(), midpoint.z()),
                Vec3::new(midpoint.x(), midpoint.y(), max.z()),
            )),
        );
        self.setnode(
            5,
            node(AABB::new(
                Vec3::new(min.x(), midpoint.y(), midpoint.z()),
                Vec3::new(midpoint.x(), max.y(), max.z()),
            )),
        );
        self.setnode(
            6,
            node(AABB::new(
                Vec3::new(midpoint.x(), min.y(), midpoint.z()),
                Vec3::new(max.x(), midpoint.y(), max.z()),
            )),
        );
        self.setnode(
            7,
            node(AABB::new(
                Vec3::new(midpoint.x(), midpoint.y(), midpoint.z()),
                Vec3::new(max.x(), max.y(), max.z()),
            )),
        );
    }

    fn setnode(&mut self, node_idx: usize, mut node: Box<OcNode>) {
        node.parent = self;
        self.sub_boxes[node_idx] = Box::<OcNode>::into_raw(node);
    }

    pub fn hit(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut HitRecord,
    ) -> Option<Vec<HittableObject>> {
        unsafe {
            if self.bounding_box.hit(ray, t_min, t_max) {
                if self.is_leaf {
                    return Some(self.hittables.clone());
                } else {
                    let v = self.sub_boxes.as_slice().iter().fold(vec![], |mut arr, b| {
                        match b.as_ref().unwrap().hit(ray, t_min, t_max, rec) {
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
}

fn node(aabb: AABB) -> Box<OcNode> {
    Box::new(OcNode::new(aabb))
}
