use crate::colour_map::ColourMap;
use crate::hittable::Hittable;
use crate::hittables::{HittableObject, Hittables};
use crate::material::Material::Lambertian;
use crate::noise::Noise;
use crate::triangle::Triangle;
use glam::Vec3A;
use std::ops::Add;

pub(crate) struct Terrain {
    ground_points: Vec<Vec3A>,
    vertex_resolution: usize,
}

impl Terrain {
    pub fn new(width: f32, depth: f32, res: usize) -> Self {
        let verts = {
            let r1 = res + 1;
            let mut verts_ = vec![];
            let mut loc = Vec3A::new(-width / 2.0, 0.0, -depth / 2.0);
            for _i in 0..r1 {
                for _j in 0..r1 {
                    verts_.push(loc);
                    loc = loc.add(Vec3A::new(width / res as f32, 0.0, 0.0));
                }
                loc = Vec3A::new(-width / 2.0, 0.0, loc.z + (depth / (res as f32)));
            }
            verts_
        };

        Terrain {
            ground_points: verts,
            vertex_resolution: res,
        }
    }

    pub fn get_triangles(
        &mut self,
        noise: Option<Noise>,
        colour_map: Option<ColourMap>,
        height_scale: f32,
    ) -> Vec<HittableObject> {
        match noise {
            Some(noise_) => {
                for i in 0..self.ground_points.len() {
                    self.ground_points[i] = Vec3A::new(
                        self.ground_points[i].x + 0.1,
                        noise_.noise_map[i] * height_scale,
                        self.ground_points[i].z + 0.1,
                    );
                }
            }
            None => {}
        }

        let hittables_: Vec<HittableObject> = {
            let r1 = &self.vertex_resolution + 1;
            let mut hittables: Vec<HittableObject> = vec![];
            for i in 0..self.vertex_resolution {
                for j in 0..self.vertex_resolution {
                    let i0j0 = self.ground_points[(((i + 0) * r1) + (j + 0)) as usize];
                    let i0j1 = self.ground_points[(((i + 0) * r1) + (j + 1)) as usize];
                    let i1j0 = self.ground_points[(((i + 1) * r1) + (j + 0)) as usize];
                    let i1j1 = self.ground_points[(((i + 1) * r1) + (j + 1)) as usize];
                    let color1: Vec3A;
                    let color2: Vec3A;

                    match &colour_map {
                        Some(colour_map_) => {
                            let height1 = (i0j1.y / height_scale
                                + i0j0.y / height_scale
                                + i1j0.y / height_scale)
                                / 3.0;
                            let height2 = (i1j0.y / height_scale
                                + i1j1.y / height_scale
                                + i0j1.y / height_scale)
                                / 3.0;
                            color1 = colour_map_.to_colour(height1);
                            color2 = colour_map_.to_colour(height2);
                        }
                        None => {
                            color1 = Vec3A::new(0.2, 0.8, 0.4);
                            color2 = Vec3A::new(0.2, 0.8, 0.4);
                        }
                    }

                    hittables.push(HittableObject::TriangleObj(Triangle::new(
                        i0j1,
                        i0j0,
                        i1j0,
                        Lambertian(color1),
                        false,
                    )));
                    hittables.push(HittableObject::TriangleObj(Triangle::new(
                        i1j0,
                        i1j1,
                        i0j1,
                        Lambertian(color2),
                        false,
                    )));
                }
            }
            hittables
        };

        hittables_
    }
}
