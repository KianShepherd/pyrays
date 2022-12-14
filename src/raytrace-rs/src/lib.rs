use crate::camera::Camera;
use crate::configuration::RonObject;
use crate::hittable::Hittable;
use crate::hittables::Hittables;
use crate::sphere::Sphere;
use crate::triangle::Triangle;
use crate::vec3::Vec3;
use material::Material;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod camera;
mod configuration;
mod hittable;
mod hittables;
mod material;
mod ray;
mod sphere;
mod triangle;
mod vec3;

fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}
fn random_f64(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
#[allow(dead_code)]
fn random_vec3(min: f64, max: f64) -> vec3::Vec3 {
    vec3::Vec3::new(
        random_f64(min, max),
        random_f64(min, max),
        random_f64(min, max),
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

fn ray_color(ray: ray::Ray, world: &hittables::Hittables, depth: i32) -> vec3::Vec3 {
    let mut hit_rec = hittable::HitRecord::new();
    let bias = 0.01;

    if depth <= 0 {
        return vec3::Vec3::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_rec) {
        let color = &mut vec3::Vec3::new(0.0, 0.0, 0.0);
        let res = material::scatter(ray, hit_rec, color, hit_rec.material.unwrap());
        match res {
            Some(result) => {
                *color
                    * ray_color(result, world, depth - 1)
                    * (0..world.lights.len()).into_iter().fold(
                        vec3::Vec3::new(1.0, 1.0, 1.0),
                        |mut in_shadow, i| {
                            let light_direction =
                                (world.lights[i] - hit_rec.p.unwrap()).unit_vector();
                            let point_of_intersection =
                                hit_rec.p.unwrap() + (light_direction * bias);
                            let max_dist = (point_of_intersection - world.lights[i]).length();
                            if world.hit(
                                ray::Ray::new(point_of_intersection, light_direction),
                                0.01,
                                max_dist / 2.0,
                                &mut hittable::HitRecord::new(),
                            ) {
                                in_shadow = in_shadow * vec3::Vec3::new(0.3, 0.3, 0.3);
                            }
                            in_shadow
                        },
                    )
            }
            None => vec3::Vec3::new(0.0, 0.0, 0.0),
        }
    } else {
        let unit_dir = ray.direction().unit_vector();
        let t = 0.5 * (unit_dir.y() + 1.0);
        let one = vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t);
        let two = vec3::Vec3::new(0.5, 0.7, 1.0) * t;
        one + two
    }
}

#[derive(Debug, Clone)]
struct Work {
    x: usize,
    y: usize,
    colour: Vec<u8>,
}

fn create_work_list(image_width: i32, image_height: i32, num_cpu: usize) -> Vec<Vec<Vec<usize>>> {
    let rows = ((image_height / num_cpu as i32) + 1) as usize;
    (0..num_cpu).into_iter().fold(vec![], |mut work_list, i| {
        work_list.push(((i * rows)..((i + 1) * rows)).into_iter().fold(
            vec![],
            |mut work_for_cpu, y| {
                (0..image_width).into_iter().for_each(|x| {
                    if y < image_height as usize {
                        work_for_cpu.push(vec![x as usize, y as usize]);
                    }
                });
                work_for_cpu
            },
        ));
        work_list
    })
}

fn sample_pixel(
    samples_per_pixel: usize,
    coord: Vec<f64>,
    image_width: i32,
    image_height: i32,
    max_depth: i32,
    camera: &Camera,
    world: &Hittables,
) -> Vec3 {
    (0..samples_per_pixel)
        .into_iter()
        .fold(Vec3::new(0.0, 0.0, 0.0), |pixel_color, _| {
            let ray = {
                let u = (coord[0] + random()) / (image_width - 1) as f64;
                let v = ((image_height as f64 - (coord[1] + 1.0)) + random())
                    / (image_height - 1) as f64;
                camera.get_ray(u, v)
            };
            pixel_color + ray_color(ray, world, max_depth)
        })
}

fn conv_py_vec(vector: Vec<f64>) -> Vec3 {
    vec3::Vec3::new(vector[0], vector[1], vector[2])
}

fn parse_ron_material(mat: Vec<String>) -> Material {
    let material_type = &mat[0];
    match &material_type[..] {
        "Lambertian" => material::Material::Lambertian(vec3::Vec3::new(
            mat[1].parse::<f64>().unwrap(),
            mat[2].parse::<f64>().unwrap(),
            mat[3].parse::<f64>().unwrap(),
        )),
        "Metal" => material::Material::Metal(
            vec3::Vec3::new(
                mat[1].parse::<f64>().unwrap(),
                mat[2].parse::<f64>().unwrap(),
                mat[3].parse::<f64>().unwrap(),
            ),
            mat[4].parse::<f64>().unwrap(),
        ),
        "Mirror" => material::Material::Mirror,
        "Dielectric" => material::Material::Dielectric(mat[1].parse::<f64>().unwrap()),
        &_ => {
            panic!("Unknown material found")
        }
    }
}

fn parse_ron_object(obj: RonObject) -> Box<dyn Hittable + Send + Sync + 'static> {
    match &*obj.objtype {
        "Sphere" => Box::new(Sphere::new(
            conv_py_vec(obj.vectors[0].clone()),
            obj.scalars[0],
            parse_ron_material(obj.material),
        )),
        "Triangle" => Box::new(Triangle::new(
            conv_py_vec(obj.vectors[0].clone()),
            conv_py_vec(obj.vectors[1].clone()),
            conv_py_vec(obj.vectors[2].clone()),
            parse_ron_material(obj.material),
            obj.scalars[0] != 0.0,
        )),
        _ => panic!("unknown ron object type."),
    }
}

pub fn create_image(ron_string: String) -> Vec<Vec<Vec<u8>>> {
    let settings = configuration::RaytracerScene::from_ron(ron_string);

    let camera = camera::Camera::new(
        conv_py_vec(settings.camera_pos.clone()),
        conv_py_vec(settings.camera_dir.clone()),
        conv_py_vec(settings.camera_up.clone()),
        settings.v_fov,
        settings.aspect_ratio,
        settings.aperture,
        settings.focal_distance,
    );

    let world = Hittables {
        lights: settings.lights.iter().fold(vec![], |mut objs, obj| {
            objs.push(conv_py_vec(obj.clone()));
            objs
        }),
        hittables: settings.objects.iter().fold(vec![], |mut objs, obj| {
            objs.push(parse_ron_object(obj.clone()));
            objs
        }),
    };

    let now = Instant::now();
    let image = if settings.multithreading {
        let image_ = Arc::new(Mutex::new({
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
        }));

        let world_ = Arc::new(world);
        let camera_ = Arc::new(camera);
        let settings_ = Arc::new(settings);

        let cpu_count = num_cpus::get();
        let work_list = Arc::new(create_work_list(
            settings_.image_width,
            settings_.image_height,
            cpu_count,
        ));

        let mut task_list = vec![];
        (0..cpu_count).into_iter().for_each(|cpu| {
            let scoped_image = image_.clone();
            let scoped_world = world_.clone();
            let scoped_camera = camera_.clone();
            let scoped_work_list = work_list.clone();
            let scoped_settings = settings_.clone();

            task_list.push(thread::spawn(move || {
                let work_list_for_cpu = scoped_work_list.get(cpu).unwrap();
                let mut inner_work_vec = Vec::with_capacity(work_list_for_cpu.len());

                work_list_for_cpu.iter().for_each(|work| {
                    inner_work_vec.push(Work {
                        x: work[0],
                        y: work[1],
                        colour: sample_pixel(
                            scoped_settings.samples_per_pixel,
                            vec![work[0] as f64, work[1] as f64],
                            scoped_settings.image_width,
                            scoped_settings.image_height,
                            scoped_settings.max_depth,
                            &scoped_camera,
                            &scoped_world,
                        )
                        .to_rgb(scoped_settings.samples_per_pixel),
                    });
                });

                let mut image_data = scoped_image.lock().unwrap();
                inner_work_vec.iter().for_each(|work| {
                    image_data[work.y as usize][work.x as usize] = work.colour.clone();
                });

                println!("Cpu {} done out of {}.", cpu + 1, cpu_count);
            }));
        });

        for task in task_list {
            let _ = task.join();
        }

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
        let progress_prints = settings.image_width as f64 / 16.0;
        (0..settings.image_height).into_iter().for_each(|y| {
            if y % ((settings.image_height as f64 / progress_prints) as i32) == 0 {
                eprintln!(
                    "{:.2}% Done",
                    (y as f64 / settings.image_height as f64) * 100.0
                );
            }
            (0..settings.image_width).into_iter().for_each(|x| {
                image_[y as usize][x as usize] = sample_pixel(
                    settings.samples_per_pixel,
                    vec![x as f64, y as f64],
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
    eprintln!(
        "100.00% Done\n\nTime taken: {}h : {}m : {}s\n\n",
        hours, minutes, seconds
    );

    image
}

#[cfg(test)]
mod tests {
    use super::*;

    fn similarity(a: Vec<Vec<Vec<u8>>>, b: Vec<Vec<Vec<u8>>>) -> f64 {
        let mut total_simi = 0.0;
        let mut point_simi;
        for y in 0..a.len() {
            for x in 0..a[0].len() {
                point_simi = 0.0;
                for v in 0..3 {
                    point_simi += (255.0 - (a[y][x][v] as f64 - b[y][x][v] as f64).abs()) / 255.0;
                }
                total_simi += point_simi / 3.0;
            }
        }

        total_simi / (a.len() * a[0].len()) as f64
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
