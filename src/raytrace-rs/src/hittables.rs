use std::rc::Rc;

use crate::colour_map::{ColourData, ColourMap};
use crate::configuration::{RonObject, RonTerrain};
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::noise::Noise;
use crate::octree::{OcTree, OcTreeBuilder};
use crate::ray::Ray;
use crate::terrain::Terrain;
use crate::Triangle;
use crate::{colour_map, Sphere};
use glam::Vec3A;

#[derive(Debug, Copy, Clone)]
pub enum HittableObject {
    SphereObj(Sphere),
    TriangleObj(Triangle),
}

pub struct Hittables {
    pub lights: Vec<Vec3A>,
    hittables: OcTree,
}

fn conv_py_vec(vector: Vec<f32>) -> Vec3A {
    Vec3A::new(vector[0], vector[1], vector[2])
}

fn parse_ron_material(mat: Vec<String>) -> Material {
    let material_type = &mat[0];
    match &material_type[..] {
        "Lambertian" => Material::Lambertian(Vec3A::new(
            mat[1].parse::<f32>().unwrap(),
            mat[2].parse::<f32>().unwrap(),
            mat[3].parse::<f32>().unwrap(),
        )),
        "Metal" => Material::Metal(
            Vec3A::new(
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
    Sphere::new(
        conv_py_vec(obj.vectors[0].clone()),
        obj.scalars[0],
        parse_ron_material(obj.material),
    )
}
fn parse_ron_triangle(obj: RonObject) -> Triangle {
    Triangle::new(
        conv_py_vec(obj.vectors[0].clone()),
        conv_py_vec(obj.vectors[1].clone()),
        conv_py_vec(obj.vectors[2].clone()),
        parse_ron_material(obj.material),
        obj.scalars[0] != 0.0,
    )
}

#[allow(dead_code)]
impl Hittables {
    pub fn new(
        lights: &[Vec<f32>],
        objects: &[RonObject],
        has_terrain: usize,
        terrain: &RonTerrain,
    ) -> Self {
        let mut _lights = vec![];
        lights.iter().for_each(|obj| {
            _lights.push(conv_py_vec(obj.clone()));
        });

        let mut _objects = vec![];
        if has_terrain != 0 {
            let mut proc_t = Terrain::new(
                terrain.p2[0] - terrain.p1[0],
                terrain.p2[2] - terrain.p1[2],
                terrain.resolution,
            );
            let noise = Noise::new(
                terrain.resolution,
                terrain.octaves,
                terrain.frequency,
                terrain.lacunarity,
                terrain.seed_value,
                terrain.persistence,
            );
            let colour_map = {
                let mut _col_map = vec![];
                for i in 0..terrain.map_cutoff.len() {
                    _col_map.push(ColourData {
                        cutoff: terrain.map_cutoff[i],
                        colour: Vec3A::new(
                            terrain.map_value[i][0],
                            terrain.map_value[i][1],
                            terrain.map_value[i][2],
                        ),
                    });
                }
                _col_map
            };
            _objects.extend(proc_t.get_triangles(
                Some(noise),
                Some(ColourMap::new(
                    colour_map,
                    Vec3A::new(0.0, 0.0, 0.0),
                    terrain.fuzz,
                )),
                terrain.magnitude,
            ));
        }
        objects.iter().for_each(|obj| {
            match &*obj.objtype {
                "Sphere" => _objects.push(Rc::new(HittableObject::SphereObj(parse_ron_sphere(
                    obj.clone(),
                )))),
                "Triangle" => _objects.push(Rc::new(HittableObject::TriangleObj(
                    parse_ron_triangle(obj.clone()),
                ))),
                _ => panic!("unknown ron object type."),
            };
        });

        Self {
            lights: _lights,
            hittables: OcTreeBuilder::new(_objects),
        }
    }

    pub fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.hittables.hit(ray, t_min, t_max)
    }
}
