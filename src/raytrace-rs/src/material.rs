use crate::ray::Ray;
use crate::{hittable::HitRecord, random_f32, random_unit_vec3};
use glam::Vec3A;
use std::intrinsics::{fadd_fast, fdiv_fast, fmul_fast, fsub_fast, powf32};

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Material {
    Lambertian(Vec3A),
    Metal(Vec3A, f32),
    Dielectric(f32),
    Mirror,
}

pub fn scatter(ray: &Ray, rec: HitRecord, color: &mut Vec3A, material: &Material) -> Option<Ray> {
    match material {
        Material::Lambertian(col) => lambertian_scatter(ray, rec, color, col),
        Material::Metal(col, fuzz) => metal_scatter(ray, rec, color, col, *fuzz),
        Material::Dielectric(refractive_index) => {
            dielectric_scatter(ray, rec, color, *refractive_index)
        }
        Material::Mirror => mirror_scatter(ray, rec, color),
    }
}

fn reflect(v: Vec3A, n: Vec3A) -> Vec3A {
    unsafe { v - (n * fmul_fast(2.0, v.dot(n))) }
}

fn refract(uv: Vec3A, n: Vec3A, etai_over_etat: f32) -> Option<Vec3A> {
    unsafe {
        let uv_ = uv.normalize();
        let dt = uv_.dot(n);
        let discriminant = fsub_fast(
            1.0,
            fmul_fast(
                etai_over_etat,
                fmul_fast(etai_over_etat, fsub_fast(1.0, fmul_fast(dt, dt))),
            ),
        );
        if discriminant > 0.0 {
            Some(((uv_ - (n * dt)) * etai_over_etat) - (n * discriminant.sqrt()))
        } else {
            None
        }
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    unsafe {
        let r0 = {
            let a = fadd_fast(fsub_fast(1.0, ref_idx), fadd_fast(1.0, ref_idx));
            fmul_fast(a, a)
        };
        fadd_fast(
            r0,
            powf32(fmul_fast(fsub_fast(1.0, r0), fsub_fast(1.0, cosine)), 5.0),
        )
    }
}

fn lambertian_scatter(
    _ray: &Ray,
    rec: HitRecord,
    color: &mut Vec3A,
    material_color: &Vec3A,
) -> Option<Ray> {
    let scatter_direction = rec.normal + random_unit_vec3();
    color.clone_from(material_color);
    Some(Ray::new(rec.p, scatter_direction))
}

fn metal_scatter(
    ray: &Ray,
    rec: HitRecord,
    color: &mut Vec3A,
    material_color: &Vec3A,
    fuzz: f32,
) -> Option<Ray> {
    let reflected = reflect(ray.direction().normalize(), rec.normal);
    let scattered = Ray::new(rec.p, reflected + (random_unit_vec3() * fuzz));
    color.clone_from(material_color);
    if scattered.direction().dot(rec.normal) > 0.0 {
        Some(scattered)
    } else {
        None
    }
}

fn mirror_scatter(ray: &Ray, rec: HitRecord, color: &mut Vec3A) -> Option<Ray> {
    let reflected = Ray::new(rec.p, reflect(ray.direction().normalize(), rec.normal));
    color.clone_from(&Vec3A::new(1.0, 1.0, 1.0));
    if reflected.direction().dot(rec.normal) > 0.0 {
        Some(reflected)
    } else {
        None
    }
}

fn dielectric_scatter(
    ray: &Ray,
    rec: HitRecord,
    color: &mut Vec3A,
    refractive_index: f32,
) -> Option<Ray> {
    unsafe {
        color.clone_from(&Vec3A::new(1.0, 1.0, 1.0));
        let reflected = reflect(ray.direction().normalize(), rec.normal);
        let outward_normal: Vec3A;
        let ni_over_nt: f32;
        let cosine: f32;

        if ray.direction().normalize().dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = refractive_index;
            cosine = fdiv_fast(
                fmul_fast(ray.direction().dot(rec.normal), refractive_index),
                ray.direction().length(),
            );
            //cosine = (1.0 - refractive_index * refractive_index * (1.0 - cosine * cosine)).sqrt();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = fdiv_fast(1.0, refractive_index);
            cosine = fdiv_fast(-ray.direction().dot(rec.normal), ray.direction().length());
        }

        match refract(ray.direction(), outward_normal, ni_over_nt) {
            Some(ray) => {
                if random_f32(0.0, 1.0) > schlick(cosine, refractive_index) {
                    return Some(Ray::new(rec.p, ray));
                }
            }
            None => {}
        }

        Some(Ray::new(rec.p, reflected))
    }
}
