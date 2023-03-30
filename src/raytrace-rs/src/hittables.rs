use crate::configuration::RonObject;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::octree::OcTree;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::Sphere;
use crate::Triangle;

#[derive(Debug, Clone)]
pub enum HittableObject {
    SphereObj(Sphere),
    TriangleObj(Triangle),
}

pub struct Hittables {
    pub lights: Vec<Vec3>,
    hittables: OcTree,
}

fn conv_py_vec(vector: Vec<f32>) -> Vec3 {
    Vec3::new(vector[0], vector[1], vector[2])
}

fn parse_ron_material(mat: Vec<String>) -> Material {
    let material_type = &mat[0];
    match &material_type[..] {
        "Lambertian" => Material::Lambertian(Vec3::new(
            mat[1].parse::<f32>().unwrap(),
            mat[2].parse::<f32>().unwrap(),
            mat[3].parse::<f32>().unwrap(),
        )),
        "Metal" => Material::Metal(
            Vec3::new(
                mat[1].parse::<f32>().unwrap(),
                mat[2].parse::<f32>().unwrap(),
                mat[3].parse::<f32>().unwrap(),
            ),
            mat[4].parse::<f32>().unwrap(),
        ),
        "Mirror" => Material::Mirror,
        "Dielectric" => Material::Dielectric(mat[1].parse::<f32>().unwrap()),
        &_ => {
            panic!("Unknown material found")
        }
    }
}

fn parse_ron_sphere(obj: RonObject) -> Sphere {
    return Sphere::new(
        conv_py_vec(obj.vectors[0].clone()),
        obj.scalars[0],
        parse_ron_material(obj.material),
    );
}
fn parse_ron_triangle(obj: RonObject) -> Triangle {
    return Triangle::new(
        conv_py_vec(obj.vectors[0].clone()),
        conv_py_vec(obj.vectors[1].clone()),
        conv_py_vec(obj.vectors[2].clone()),
        parse_ron_material(obj.material),
        obj.scalars[0] != 0.0,
    );
}

#[allow(dead_code)]
impl Hittables {
    pub fn new(lights: &Vec<Vec<f32>>, objects: &Vec<RonObject>) -> Self {
        let mut _lights = vec![];
        lights.iter().for_each(|obj| {
            _lights.push(conv_py_vec(obj.clone()));
        });
        let mut _objects = vec![];
        objects.iter().for_each(|obj| {
            match &*obj.objtype {
                "Sphere" => _objects.push(HittableObject::SphereObj(parse_ron_sphere(obj.clone()))),
                "Triangle" => {
                    _objects.push(HittableObject::TriangleObj(parse_ron_triangle(obj.clone())))
                }
                _ => panic!("unknown ron object type."),
            };
        });

        Self {
            lights: _lights,
            hittables: OcTree::new(_objects),
        }
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut temp_rec = HitRecord::new();
        let mut closest = t_max;

        match self.hittables.hit(ray, t_min, t_max, rec) {
            Some(hs) => {
                hs.iter().for_each(|hittable| match hittable {
                    HittableObject::SphereObj(s) => {
                        if s.hit(ray, t_min, closest, &mut temp_rec) {
                            hit_anything = true;
                            closest = temp_rec.get_t().unwrap();
                            rec.set_rec(&temp_rec);
                        }
                    }
                    HittableObject::TriangleObj(t) => {
                        if t.hit(ray, t_min, closest, &mut temp_rec) {
                            hit_anything = true;
                            closest = temp_rec.get_t().unwrap();
                            rec.set_rec(&temp_rec);
                        }
                    }
                });
            }
            None => {}
        };
        hit_anything
    }
}
