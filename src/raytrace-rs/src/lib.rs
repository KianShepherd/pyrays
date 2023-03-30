#![feature(core_intrinsics)]
use crate::camera::Camera;
use crate::hittables::Hittables;
use crate::sphere::Sphere;
use crate::triangle::Triangle;
use crate::vec3::Vec3;
use linya::{Bar, Progress};
use rand::Rng;
use rayon::prelude::*;
use std::intrinsics::{fadd_fast, fdiv_fast, fmul_fast, fsub_fast};
use std::sync::Mutex;
use std::time::Instant;

mod aabb;
mod camera;
mod configuration;
mod hittable;
mod hittables;
mod material;
mod octree;
mod ray;
mod sphere;
mod triangle;
mod vec3;

fn random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen::<f32>()
}
fn random_f32(min: f32, max: f32) -> f32 {
    unsafe { fadd_fast(min, fmul_fast(fsub_fast(max, min), random())) }
}
#[allow(dead_code)]
fn random_vec3(min: f32, max: f32) -> vec3::Vec3 {
    vec3::Vec3::new(
        random_f32(min, max),
        random_f32(min, max),
        random_f32(min, max),
    )
}
// Quick Diffusion
#[allow(dead_code)]
fn random_unit_vec3() -> vec3::Vec3 {
    let mut p: vec3::Vec3;
    loop {
        p = random_vec3(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            break;
        }
    }
    p
}

fn ray_color(ray: &ray::Ray, world: &hittables::Hittables, depth: i32) -> vec3::Vec3 {
    unsafe {
        let mut hit_rec = hittable::HitRecord::new();
        let bias = 0.01;

        if depth <= 0 {
            return vec3::Vec3::new(0.0, 0.0, 0.0);
        }

        if world.hit(ray, 0.001, f32::INFINITY, &mut hit_rec) {
            let color = &mut vec3::Vec3::new(0.0, 0.0, 0.0);
            let res = material::scatter(&ray, hit_rec, color, &hit_rec.material.unwrap());
            match res {
                Some(result) => {
                    &(&*color * &ray_color(&result, world, depth - 1))
                        * &((0..world.lights.len()).into_iter().fold(
                            vec3::Vec3::new(1.0, 1.0, 1.0),
                            |mut in_shadow, i| {
                                let light_direction =
                                    (&world.lights[i] - &hit_rec.p.unwrap()).unit_vector();
                                let point_of_intersection =
                                    &hit_rec.p.unwrap() + &(&light_direction * bias);
                                let max_dist = (&point_of_intersection - &world.lights[i]).length();
                                if world.hit(
                                    &ray::Ray::new(point_of_intersection, light_direction),
                                    0.01,
                                    fdiv_fast(max_dist, 2.0),
                                    &mut hittable::HitRecord::new(),
                                ) {
                                    in_shadow = &in_shadow * &vec3::Vec3::new(0.3, 0.3, 0.3);
                                }
                                in_shadow
                            },
                        ))
                }
                None => vec3::Vec3::new(0.0, 0.0, 0.0),
            }
        } else {
            let unit_dir = ray.direction().unit_vector();
            let t = fmul_fast(0.5, fadd_fast(unit_dir.y(), 1.0));
            let one = &vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t);
            let two = &vec3::Vec3::new(0.68, 0.8, 1.0) * t;
            &one + &two
        }
    }
}

#[derive(Debug, Clone)]
struct Work {
    x: usize,
    y: usize,
    colour: Vec<u8>,
}

fn create_work_list(image_width: i32, image_height: i32) -> Vec<Vec<Vec<usize>>> {
    (0..image_height).into_iter().fold(vec![], |mut work, y| {
        work.push((0..image_width).into_iter().fold(vec![], |mut row, x| {
            row.push(vec![x as usize, y as usize]);
            row
        }));
        work
    })
}

fn sample_pixel(
    samples_per_pixel: usize,
    coord: Vec<f32>,
    image_width: i32,
    image_height: i32,
    max_depth: i32,
    camera: &Camera,
    world: &Hittables,
) -> Vec3 {
    unsafe {
        (0..samples_per_pixel)
            .into_iter()
            .fold(Vec3::new(0.0, 0.0, 0.0), |pixel_color, _| {
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
                &pixel_color + &ray_color(&ray, world, max_depth)
            })
    }
}

fn conv_py_vec(vector: Vec<f32>) -> Vec3 {
    vec3::Vec3::new(vector[0], vector[1], vector[2])
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

    eprintln!("Generating BVH.");
    let now_w = Instant::now();
    let world = Hittables::new(&settings.lights, &settings.objects);
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
    let image = if settings.multithreading {
        let image_ = Mutex::new({
            let row = (0..settings.image_width)
                .into_iter()
                .fold(vec![], |mut _row, _| {
                    _row.push(vec![0, 0, 0]);
                    _row
                });
            (0..settings.image_height)
                .into_iter()
                .fold(vec![], |mut _vec, _| {
                    _vec.push(row.clone());
                    _vec
                })
        });

        let work_list = create_work_list(settings.image_width, settings.image_height);
        let progress = Mutex::new(Progress::new());
        let progress_bar: Bar = progress
            .lock()
            .unwrap()
            .bar(work_list.len(), format!("Scan lines completed"));

        work_list.par_iter().for_each(|row| {
            let mut inner_work_vec = vec![];
            row.iter().for_each(|work| {
                inner_work_vec.push(Work {
                    x: work[0],
                    y: work[1],
                    colour: sample_pixel(
                        settings.samples_per_pixel,
                        vec![work[0] as f32, work[1] as f32],
                        settings.image_width,
                        settings.image_height,
                        settings.max_depth,
                        &camera,
                        &world,
                    )
                    .to_rgb(settings.samples_per_pixel),
                });
            });

            let mut image_data = image_.lock().unwrap();
            inner_work_vec.iter().for_each(|work| {
                image_data[work.y as usize][work.x as usize] = work.colour.clone();
            });
            progress.lock().unwrap().inc_and_draw(&progress_bar, 1);
        });

        let final_val = match image_.lock() {
            Ok(x) => x.clone(),
            Err(_) => vec![],
        };
        final_val
    } else {
        // Single Thread
        let mut image_ = {
            let row = (0..settings.image_width as usize)
                .into_iter()
                .fold(vec![], |mut row, _| {
                    row.push(vec![0, 0, 0]);
                    row
                });
            (0..settings.image_height as usize)
                .into_iter()
                .fold(vec![], |mut _vec, _| {
                    _vec.push(row.clone());
                    _vec
                })
        };
        let progress_prints = settings.image_width as f32 / 16.0;
        (0..settings.image_height).into_iter().for_each(|y| {
            if y % ((settings.image_height as f32 / progress_prints) as i32) == 0 {
                eprintln!(
                    "{:.2}% Done",
                    (y as f32 / settings.image_height as f32) * 100.0
                );
            }
            (0..settings.image_width).into_iter().for_each(|x| {
                image_[y as usize][x as usize] = sample_pixel(
                    settings.samples_per_pixel,
                    vec![x as f32, y as f32],
                    settings.image_width,
                    settings.image_height,
                    settings.max_depth,
                    &camera,
                    &world,
                )
                .to_rgb(settings.samples_per_pixel);
            });
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
