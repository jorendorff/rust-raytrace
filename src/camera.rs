use vec::{Vec3, Ray};

use std::f32::consts::PI;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(vfov_degrees: f32, aspect: f32) -> Camera {
        let theta = vfov_degrees * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            origin: Vec3(0.0, 0.0, 0.0),
            vertical: Vec3(0.0, 2.0 * half_height, 0.0),
            horizontal: Vec3(2.0 * half_width, 0.0, 0.0),
            lower_left_corner: Vec3(-half_width, -half_height, -1.0)
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
