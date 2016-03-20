use vec::{Vec3, Ray};

use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov_degrees: f32, aspect: f32) -> Camera {
        let theta = vfov_degrees * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).to_unit_vector();
        let u = vup.cross(w).to_unit_vector();
        let v = w.cross(u);
        Camera {
            origin: lookfrom,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            lower_left_corner: lookfrom - half_width * u - half_height * v - w
       }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
