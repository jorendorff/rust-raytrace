mod vec;
mod model;

use vec::{Vec3, Ray};
use model::{HitTest, Sphere};

fn color<T: HitTest>(r: Ray, model: &T) -> Vec3 {
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    const SKY_BLUE: Vec3 = Vec3(0.5, 0.7, 1.0);

    let unit_direction = r.direction().to_unit_vector();
    if let Some(hit) = model.hit(&r) {
        return 0.5 * Vec3(hit.normal.x() + 1.0,
                          hit.normal.y() + 1.0,
                          hit.normal.z() + 1.0);
    }
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * WHITE + t * SKY_BLUE
}

fn main() {
    const WIDTH: usize = 200;
    const HEIGHT: usize = 100;

    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    // camera position (a point)
    let origin = Vec3(0.0, 0.0, 1.0);

    // direction vectors defining the view
    let lower_left_corner = Vec3(-2.0, -1.5, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);

    // model
    let spheres: Vec<Box<HitTest>> = vec![
        Box::new(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5
        }),
        Box::new(Sphere {
            center: Vec3(0.0, -100.5, -1.0),
            radius: 100.0
        })
    ];

    for y in 0 .. HEIGHT {
        let j = HEIGHT - 1 - y;
        for i in 0 .. WIDTH {
            let u = i as f32 / WIDTH as f32;
            let v = j as f32 / HEIGHT as f32;
            let r = Ray(origin, lower_left_corner + u * horizontal + v * vertical);
            let rgb = color(r, &spheres).to_u8();
            println!("{} {} {}", rgb[0], rgb[1], rgb[2]);
        }
    }
}
