#![feature(core_intrinsics, arc_unwrap_or_clone)]
use crate::camera::Camera;
use crate::colour_map::{ColourData, ColourMap};
use crate::hittables::{HittableObject, Hittables};
use crate::noise::Noise;
use crate::sphere::Sphere;
use crate::terrain::Terrain;
use crate::triangle::Triangle;
use configuration::RonObject;
use glam::Vec3A;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use material::Material;
use rand::Rng;
use rayon::prelude::*;
use std::fmt::Write;
use std::intrinsics::{fadd_fast, fdiv_fast, fmul_fast, fsub_fast, maxnumf32, minnumf32};
use std::time::Instant;

mod aabb;
mod camera;
mod colour_map;
mod configuration;
mod hittable;
mod hittables;
mod material;
mod noise;
mod octree;
mod ray;
mod sphere;
mod terrain;
mod triangle;

fn random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}
fn random_f32(min: f32, max: f32) -> f32 {
    unsafe { fadd_fast(min, fmul_fast(fsub_fast(max, min), random())) }
}
#[allow(dead_code)]
fn random_vec3(min: f32, max: f32) -> Vec3A {
    Vec3A::new(
        random_f32(min, max),
        random_f32(min, max),
        random_f32(min, max),
    )
}
// Quick Diffusion
#[allow(dead_code)]
fn random_unit_vec3() -> Vec3A {
    let mut p: Vec3A;
    loop {
        p = random_vec3(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break;
        }
    }
    p
}
fn clamp(val: f32, min: f32, max: f32) -> f32 {
    maxnumf32(minnumf32(val, max), min)
}

fn to_rgb(colour: Vec3A, samples_per_pixel: usize) -> Vec<u8> {
    unsafe {
        let scale = fdiv_fast(1.0, samples_per_pixel as f32);
        let r = fmul_fast(256.0, clamp(fmul_fast(colour.x, scale).sqrt(), 0.0, 0.999)) as u8;
        let g = fmul_fast(256.0, clamp(fmul_fast(colour.y, scale).sqrt(), 0.0, 0.999)) as u8;
        let b = fmul_fast(256.0, clamp(fmul_fast(colour.z, scale).sqrt(), 0.0, 0.999)) as u8;

        vec![r, g, b]
    }
}

fn ray_color(ray: ray::Ray, world: &hittables::Hittables, depth: i32) -> Vec3A {
    let bias = 0.01;

    if depth <= 0 {
        return Vec3A::new(0.0, 0.0, 0.0);
    }

    match world.hit(ray, 0.001, f32::INFINITY) {
        Some(hit_rec) => {
            let color = &mut Vec3A::new(0.0, 0.0, 0.0);
            match material::scatter(ray, hit_rec, color, &hit_rec.material) {
                Some(result) => {
                    (*color * ray_color(result, world, depth - 1))
                        * ((0..world.lights.len()).fold(
                            Vec3A::new(1.0, 1.0, 1.0),
                            |in_shadow, i| {
                                let _light_direction = (world.lights[i] - hit_rec.p).normalize();
                                let point_of_intersection = hit_rec.p + (_light_direction * bias);
                                let light_direction = _light_direction + random_unit_vec3() / 6.0;
                                let max_dist = (point_of_intersection - world.lights[i]).length();
                                if let Some(_h) = world.hit(
                                    ray::Ray::new(point_of_intersection, light_direction),
                                    0.01,
                                    unsafe { fdiv_fast(max_dist, 2.0) },
                                ) {
                                    in_shadow * Vec3A::new(0.05, 0.05, 0.05)
                                } else {
                                    let angle_of_incedence = {
                                        let mut a_o_i =
                                            _light_direction.dot(hit_rec.get_normal().normalize());
                                        if a_o_i < 0.0 {
                                            a_o_i = 0.0;
                                        }
                                        a_o_i
                                    };
                                    in_shadow * angle_of_incedence
                                }
                            },
                        ))
                }
                None => Vec3A::new(0.0, 0.0, 0.0),
            }
        }
        None => {
            let unit_dir = ray.direction().normalize();
            let t = unsafe { fmul_fast(0.5, fadd_fast(unit_dir.y, 1.0)) };
            let one = Vec3A::new(1.0, 1.0, 1.0) * (1.0 - t);
            let two = Vec3A::new(0.68, 0.8, 1.0) * t;
            one + two
        }
    }
}

fn create_work_list(image_width: i32, image_height: i32) -> Vec<Vec<Vec<usize>>> {
    (0..image_height).fold(vec![], |mut work, y| {
        work.push((0..image_width).fold(vec![], |mut row, x| {
            row.push(vec![x as usize, y as usize]);
            row
        }));
        work
    })
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
fn sample_pixel(
    samples_per_pixel: usize,
    coord: Vec<f32>,
    image_width: i32,
    image_height: i32,
    max_depth: i32,
    camera: &Camera,
    world: &Hittables,
) -> Vec3A {
    unsafe {
        (0..samples_per_pixel).fold(Vec3A::new(0.0, 0.0, 0.0), |pixel_color, _| {
            let ray = {
                let u = fdiv_fast(fadd_fast(coord[0], random()), (image_width - 1) as f32);
                let v = fdiv_fast(
                    fadd_fast(
                        fsub_fast(image_height as f32, fadd_fast(coord[1], 1.0)),
                        random(),
                    ),
                    (image_height - 1) as f32,
                );
                camera.get_ray(u, v)
            };
            pixel_color + ray_color(ray, world, max_depth)
        })
    }
}

fn conv_py_vec(vector: Vec<f32>) -> Vec3A {
    Vec3A::new(vector[0], vector[1], vector[2])
}

pub fn create_image(ron_string: String) -> Vec<Vec<Vec<u8>>> {
    let settings = configuration::RaytracerScene::from_ron(ron_string);
    eprintln!("Loaded scene config into raytracer.\n");

    let camera = camera::Camera::new(
        conv_py_vec(settings.camera_pos.clone()),
        conv_py_vec(settings.camera_dir.clone()),
        conv_py_vec(settings.camera_up.clone()),
        settings.v_fov,
        settings.aspect_ratio,
        settings.aperture,
        settings.focal_distance,
    );

    eprintln!("Generating Procedural Terrain.");
    let now_p = Instant::now();
    let mut _objects = vec![];
    if settings.has_terrain != 0 {
        let mut proc_t = Terrain::new(
            settings.terrain.p2[0] - settings.terrain.p1[0],
            settings.terrain.p2[2] - settings.terrain.p1[2],
            settings.terrain.resolution,
        );
        let noise = Noise::new(
            settings.terrain.resolution,
            settings.terrain.octaves,
            settings.terrain.frequency,
            settings.terrain.lacunarity,
            settings.terrain.seed_value,
            settings.terrain.persistence,
            settings.terrain.erosion_factor,
        );
        let colour_map = {
            let mut _col_map = vec![];
            for i in 0..settings.terrain.map_cutoff.len() {
                _col_map.push(ColourData {
                    cutoff: settings.terrain.map_cutoff[i],
                    colour: Vec3A::new(
                        settings.terrain.map_value[i][0],
                        settings.terrain.map_value[i][1],
                        settings.terrain.map_value[i][2],
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
                settings.terrain.fuzz,
            )),
            settings.terrain.magnitude,
        ));
    }
    settings.objects.iter().for_each(|obj| {
        match &*obj.objtype {
            "Sphere" => _objects.push(HittableObject::SphereObj(parse_ron_sphere(obj.clone()))),
            "Triangle" => {
                _objects.push(HittableObject::TriangleObj(parse_ron_triangle(obj.clone())))
            }
            _ => panic!("unknown ron object type."),
        };
    });
    let mut seconds_p = now_p.elapsed().as_secs();
    let mut minutes_p = seconds_p / 60;
    seconds_p %= 60;
    let hours_p = minutes_p / 60;
    minutes_p %= 60;
    eprintln!(
        "Procedural Terrain generation done.\nTime taken: {}h : {}m : {}s\n",
        hours_p, minutes_p, seconds_p
    );
    eprintln!("Generating BVH.");
    let now_w = Instant::now();
    let world = Hittables::new(&settings.lights, &_objects);
    let mut seconds_w = now_w.elapsed().as_secs();
    let mut minutes_w = seconds_w / 60;
    seconds_w %= 60;
    let hours_w = minutes_w / 60;
    minutes_w %= 60;
    eprintln!(
        "BVH generation done.\nTime taken: {}h : {}m : {}s\n",
        hours_w, minutes_w, seconds_w
    );
    eprintln!("Raytracing scene");

    let now = Instant::now();
    let pb = ProgressBar::new(settings.image_height as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("#>-"),
    );

    let image = if settings.multithreading {
        let image = {
            let row = vec![vec![0, 0, 0]; settings.image_width as usize];
            (0..settings.image_height).fold(vec![], |mut _vec, _| {
                _vec.push(row.clone());
                _vec
            })
        };

        let work_list = create_work_list(settings.image_width, settings.image_height);

        work_list.par_iter().for_each(|row| {
            row.iter().for_each(|work| {
                let colour = to_rgb(
                    sample_pixel(
                        settings.samples_per_pixel,
                        vec![work[0] as f32, work[1] as f32],
                        settings.image_width,
                        settings.image_height,
                        settings.max_depth,
                        &camera,
                        &world,
                    ),
                    settings.samples_per_pixel,
                );
                unsafe {
                    let raw_row = image.as_ptr() as *mut Vec<Vec<u8>>;
                    let raw_column = (*raw_row.add(work[1])).as_ptr() as *mut Vec<u8>;
                    let raw_pixel = (*raw_column.add(work[0])).as_ptr() as *mut u8;
                    *raw_pixel.add(0) = colour[0];
                    *raw_pixel.add(1) = colour[1];
                    *raw_pixel.add(2) = colour[2];
                }
            });

            pb.inc(1);
        });
        image.clone()
    } else {
        // Single Thread
        let mut image_ = {
            let row = (0..settings.image_width as usize).fold(vec![], |mut row, _| {
                row.push(vec![0, 0, 0]);
                row
            });
            (0..settings.image_height as usize).fold(vec![], |mut _vec, _| {
                _vec.push(row.clone());
                _vec
            })
        };
        (0..settings.image_height).for_each(|y| {
            (0..settings.image_width).for_each(|x| {
                image_[y as usize][x as usize] = to_rgb(
                    sample_pixel(
                        settings.samples_per_pixel,
                        vec![x as f32, y as f32],
                        settings.image_width,
                        settings.image_height,
                        settings.max_depth,
                        &camera,
                        &world,
                    ),
                    settings.samples_per_pixel,
                );
            });

            pb.inc(1);
        });
        image_
    };

    let mut seconds = now.elapsed().as_secs();
    let mut minutes = seconds / 60;
    seconds %= 60;
    let hours = minutes / 60;
    minutes %= 60;
    eprintln!("Time taken: {}h : {}m : {}s", hours, minutes, seconds);

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    fn similarity(a: Vec<Vec<Vec<u8>>>, b: Vec<Vec<Vec<u8>>>) -> f32 {
        let mut total_simi = 0.0;
        let mut point_simi;
        for y in 0..a.len() {
            for x in 0..a[0].len() {
                point_simi = 0.0;
                for v in 0..3 {
                    point_simi += (255.0 - (a[y][x][v] as f32 - b[y][x][v] as f32).abs()) / 255.0;
                }
                total_simi += point_simi / 3.0;
            }
        }

        total_simi / (a.len() * a[0].len()) as f32
    }

    #[test]
    fn test_lib() -> Result<(), String> {
        let ron_str1 = "RaytracerScene(multithreading: false, aspect_ratio: 1.7751479289940828, image_width: 100, image_height: 67, samples_per_pixel: 100, max_depth: 25,v_fov: 90.0, aperture: 0.01, focal_distance: 3.5, camera_pos: [0.0, 0.0, -3.5], camera_dir: [0.0, 0.0, 0.0], camera_up: [0.0, 1.0, 0.0], objects: [(objtype: \"Sphere\", vectors: [[0.6, 0.0, -1.5]], scalars: [0.5], material: [\"Metal\", \"0.7\", \"0.6\", \"0.2\", \"0.3\"]), (objtype: \"Sphere\", vectors: [[-0.9, -1.0, -1.2]], scalars: [0.5], material: [\"Mirror\"]),(objtype: \"Sphere\", vectors: [[0.7, 0.8, -1.2]], scalars: [0.5], material: [\"Dielectric\", \"0.8\"]), (objtype: \"Sphere\", vectors: [[-0.7, 0.8, -1.2]], scalars: [0.5], material: [\"Lambertian\", \"0.9\", \"0.0\", \"0.8\"]), (objtype: \"Triangle\", vectors: [[2.0, -2.0, 0.0], [-2.0, -2.0, 0.0], [-2.0, 2.0, 0.0]],scalars: [1.0], material: [\"Lambertian\", \"0.0\", \"0.6\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[2.0, -2.0, 0.0], [-2.0, 2.0, 0.0], [2.0, 2.0, 0.0]],scalars: [1.0], material: [\"Lambertian\", \"0.0\", \"0.6\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[-2.0, -2.0, 0.0], [-2.0, -2.0, -2.0], [-2.0, 2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.6\", \"0.0\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[-2.0, -2.0, 0.0], [-2.0, 2.0, -2.0], [-2.0, 2.0, 0.0]],scalars: [1.0], material: [\"Lambertian\", \"0.6\", \"0.0\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[2.0, -2.0, -2.0], [2.0, -2.0, 0.0], [2.0, 2.0, 0.0]],scalars: [1.0], material: [\"Lambertian\", \"0.9\", \"0.9\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[2.0, -2.0, -2.0], [2.0, 2.0, 0.0], [2.0, 2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.9\", \"0.9\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[2.0, 2.0, 0.0], [-2.0, 2.0, 0.0], [-2.0, 2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.0\", \"0.0\", \"0.9\"]), (objtype: \"Triangle\", vectors: [[2.0, 2.0, 0.0], [-2.0, 2.0, -2.0], [2.0, 2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.0\", \"0.0\", \"0.9\"]), (objtype: \"Triangle\", vectors: [[-2.0, -2.0, 0.0], [2.0, -2.0, 0.0], [2.0, -2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.7\", \"0.0\", \"0.9\"]), (objtype: \"Triangle\", vectors: [[-2.0, -2.0, 0.0], [2.0, -2.0, -2.0], [-2.0, -2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.7\", \"0.0\", \"0.9\"])], lights: [[-1.0, 1.5, -3.5]])";
        let ron_str2 = "RaytracerScene(multithreading: false, aspect_ratio: 1.7751479289940828, image_width: 100, image_height: 67, samples_per_pixel: 100, max_depth: 25,v_fov: 90.0, aperture: 0.01, focal_distance: 3.5, camera_pos: [0.0, 0.0, -3.5], camera_dir: [0.0, 0.0, 0.0], camera_up: [0.0, 1.0, 0.0], objects: [(objtype: \"Sphere\", vectors: [[0.6, 0.0, -1.5]], scalars: [0.5], material: [\"Metal\", \"0.7\", \"0.6\", \"0.2\", \"0.3\"]), (objtype: \"Sphere\", vectors: [[-0.9, -1.0, -1.2]], scalars: [0.5], material: [\"Mirror\"]),(objtype: \"Sphere\", vectors: [[0.7, 0.8, -1.2]], scalars: [0.5], material: [\"Dielectric\", \"0.8\"]), (objtype: \"Sphere\", vectors: [[-0.7, 0.8, -1.2]], scalars: [0.5], material: [\"Lambertian\", \"0.9\", \"0.0\", \"0.8\"]), (objtype: \"Triangle\", vectors: [[2.0, -2.0, 0.0], [-2.0, -2.0, 0.0], [-2.0, 2.0, 0.0]],scalars: [1.0], material: [\"Lambertian\", \"0.0\", \"0.6\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[2.0, -2.0, 0.0], [-2.0, 2.0, 0.0], [2.0, 2.0, 0.0]],scalars: [1.0], material: [\"Lambertian\", \"0.0\", \"0.6\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[-2.0, -2.0, 0.0], [-2.0, -2.0, -2.0], [-2.0, 2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.6\", \"0.0\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[-2.0, -2.0, 0.0], [-2.0, 2.0, -2.0], [-2.0, 2.0, 0.0]],scalars: [1.0], material: [\"Lambertian\", \"0.6\", \"0.0\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[2.0, -2.0, -2.0], [2.0, -2.0, 0.0], [2.0, 2.0, 0.0]],scalars: [1.0], material: [\"Lambertian\", \"0.9\", \"0.9\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[2.0, -2.0, -2.0], [2.0, 2.0, 0.0], [2.0, 2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.9\", \"0.9\", \"0.0\"]), (objtype: \"Triangle\", vectors: [[2.0, 2.0, 0.0], [-2.0, 2.0, 0.0], [-2.0, 2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.0\", \"0.0\", \"0.9\"]), (objtype: \"Triangle\", vectors: [[2.0, 2.0, 0.0], [-2.0, 2.0, -2.0], [2.0, 2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.0\", \"0.0\", \"0.9\"]), (objtype: \"Triangle\", vectors: [[-2.0, -2.0, 0.0], [2.0, -2.0, 0.0], [2.0, -2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.7\", \"0.0\", \"0.9\"]), (objtype: \"Triangle\", vectors: [[-2.0, -2.0, 0.0], [2.0, -2.0, -2.0], [-2.0, -2.0, -2.0]],scalars: [1.0], material: [\"Lambertian\", \"0.7\", \"0.0\", \"0.9\"])], lights: [[-1.0, 1.5, -3.5]])";
        let simi = similarity(
            create_image(ron_str1.to_string()),
            create_image(ron_str2.to_string()),
        );
        let settings = configuration::RaytracerScene::from_ron(ron_str1.to_string());
        assert_eq!(false, settings.to_ron().is_empty());
        assert_eq!(simi > 0.95, true);
        Ok(())
    }
}
