extern crate rand;
extern crate lodepng;

use lodepng::RGB;

mod vec;
mod materials;
mod model;
mod camera;

use rand::random;
use vec::{Vec3, Ray};
use model::{HitTest, Sphere};
use materials::{Lambertian, Metal};
use camera::Camera;

fn color<T: HitTest>(mut r: Ray, model: &T) -> Vec3 {
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    const SKY_BLUE: Vec3 = Vec3(0.5, 0.7, 1.0);

    let mut attenuation = WHITE;
    let mut depth = 0;
    while let Some(hit) = model.hit(&r) {
        let scattered = hit.material.scatter(&r, &hit);
        attenuation = attenuation * scattered.color;
        if let Some(bounce) = scattered.ray {
            r = bounce;
        } else {
            break;
        }

        depth += 1;
        if depth >= 50 {
            break;
        }
    }
    let unit_direction = r.direction.to_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let orig_color = (1.0 - t) * WHITE + t * SKY_BLUE;
    orig_color * attenuation
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;

    const NSAMPLES: usize = 100;

    let mut pixels: Vec<RGB<u8>> = Vec::with_capacity(WIDTH * HEIGHT);

    // model
    let spheres: Vec<Box<HitTest>> = vec![
        Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Lambertian {
                albedo: Vec3(0.8, 0.3, 0.3)
            })
        }),
        Box::new(Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Box::new(Lambertian {
                albedo: Vec3(0.8, 0.8, 0.3)
            })
        }),
        Box::new(Sphere {
            center: Vec3(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Metal {
                albedo: Vec3(0.8, 0.5, 0.2),
                fuzz: 0.3,
            })
        }),
        Box::new(Sphere {
            center: Vec3(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(Metal {
                albedo: Vec3(0.8, 0.8, 0.8),
                fuzz: 1.0
            })
        })
    ];

    let cam = Camera::new();
    for y in 0 .. HEIGHT {
        let j = HEIGHT - 1 - y;
        for i in 0 .. WIDTH {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0 .. NSAMPLES {
                let u = (i as f32 + random::<f32>()) / WIDTH as f32;
                let v = (j as f32 + random::<f32>()) / HEIGHT as f32;

                let r = cam.get_ray(u, v);
                col = col + color(r, &spheres);
            }
            col = col / NSAMPLES as f32;
            col = Vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let rgb = col.to_u8();
            pixels.push(RGB { r: rgb[0], g: rgb[1], b: rgb[2] });
        }
    }

    let filename = "pretty.png";
    match lodepng::encode24_file(filename, &pixels, WIDTH, HEIGHT) {
        Ok(()) => {}
        Err(err) => println!("Error writing file \"{}\": {}", filename, err)
    }
}
