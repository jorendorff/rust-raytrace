extern crate rand;
extern crate lodepng;

mod vec;
mod materials;
mod model;
mod camera;
mod render;

use rand::random;
use vec::{Vec3, random_in_unit_disc};
use model::{Model, Sphere};
use materials::{Material, Lambertian, Metal, Dielectric};
use camera::Camera;

fn random_scene() -> Box<Model> {
    // model
    let mut spheres: Vec<Sphere> = vec![
        Sphere {
            center: Vec3(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Box::new(Lambertian {
                albedo: Vec3(1.0, 0.6, 0.5)
            })
        },
        Sphere {
            center: Vec3(-4.0, 2.0, 0.0),
            radius: 2.0,
            material: Box::new(Lambertian {
                albedo: Vec3(0.6, 0.2, 0.2)
            })
        },
        Sphere {
            center: Vec3(0.0, 2.0, 0.0),
            radius: 2.0,
            material: Box::new(Dielectric {
                index: 1.5
            })
        },
        Sphere {
            center: Vec3(4.0, 2.0, 0.0),
            radius: 2.0,
            material: Box::new(Metal {
                albedo: Vec3(0.85, 0.9, 0.7),
                fuzz: 0.0,
            })
        }
    ];

    fn random_material() -> Box<Material> {
        match random() {
            0.0 ... 0.7 =>
                Box::new(Lambertian {
                    albedo: random()
                }),
            0.7 ... 0.9 =>
                Box::new(Metal {
                    albedo: Vec3(0.5, 0.5, 0.5) + 0.5 * random::<Vec3>(),
                    fuzz: 0.5 * random::<f32>()
                }),
            _ =>
                Box::new(Dielectric {
                    index: 1.5
                })
        }
    }

    for _ in 0..500 {
        let r = 0.4;
        let Vec3(x, z, _) = random_in_unit_disc();
        let pos = 20.0 * Vec3(x, 0.0, z) + Vec3(0.0, r, 0.0);
        if spheres.iter().all(|s| (s.center - pos).length() >= s.radius + r) {
            spheres.push(Sphere {
                center: pos,
                radius: r,
                material: random_material()
            });
        }
    }

    let world: Vec<Box<Model>> = spheres.into_iter().map(|s| Box::new(s) as Box<Model>).collect();
    Box::new(world)
}

fn main() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 400;

    const NSAMPLES: usize = 100;

    let scene = random_scene();

    let lookfrom = Vec3(16.0, 3.5, 12.0);
    let lookat = Vec3(0.0, 1.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let focus_distance = (lookfrom - lookat).length();
    let aperture = 0.3;
    let camera = Camera::new(lookfrom, lookat, vup, 20.0, WIDTH as f32 / HEIGHT as f32,
                             aperture, focus_distance);

    let pixels = render::render(&*scene, &camera, WIDTH, HEIGHT, NSAMPLES);

    let filename = "out.png";
    match lodepng::encode24_file(filename, &pixels, WIDTH, HEIGHT) {
        Ok(()) => {}
        Err(err) => println!("Error writing file \"{}\": {}", filename, err)
    }
}
