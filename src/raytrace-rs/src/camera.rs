use crate::random_f32;
use crate::ray::Ray;
use glam::Vec3A;

fn random_in_unit_disk() -> Vec3A {
    loop {
        let p = Vec3A::new(random_f32(-1.0, 1.0), random_f32(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

#[allow(dead_code)]
pub struct Camera {
    origin: Vec3A,
    horizontal: Vec3A,
    vertical: Vec3A,
    lower_left_corner: Vec3A,
    w: Vec3A,
    u: Vec3A,
    v: Vec3A,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3A,
        look_at: Vec3A,
        v_up: Vec3A,
        v_fov: f32,
        aspect_ratio: f32,
        aperature: f32,
        focus_dist: f32,
    ) -> Camera {
        let theta = (v_fov * std::f32::consts::PI) / 180.0;
        let h = (theta / 2.0).tan();

        let _viewport_height = 2.0 * h;
        let _viewport_width = aspect_ratio * _viewport_height;
        let _focal_length = 1.0;

        let w = (look_from - look_at).normalize();
        let u = v_up.cross(w).normalize();
        let v = w.cross(u);

        let _origin = look_from;
        let _horizontal = u * focus_dist * _viewport_width;
        let _vertical = v * focus_dist * _viewport_height;
        let _lower_left_corner = _origin - _horizontal * 0.5 - _vertical * 0.5 - w * focus_dist;

        let lens_radius = aperature / 2.0;

        Camera {
            origin: _origin,
            horizontal: _horizontal,
            vertical: _vertical,
            lower_left_corner: _lower_left_corner,
            w,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            ((self.lower_left_corner + ((self.horizontal * s) + (self.vertical * t)))
                - self.origin)
                - offset,
        )
    }
}
