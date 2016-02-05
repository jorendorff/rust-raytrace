mod vec;

use vec::{Vec3, Ray};


fn hit_sphere(center: Vec3, radius: f32, r: Ray) -> bool {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn color(r: Ray) -> Vec3 {
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    const SKY_BLUE: Vec3 = Vec3(0.5, 0.7, 1.0);
    const RED: Vec3 = Vec3(1.0, 0.0, 0.0);

    if hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, r) {
        return RED;
    }
    let unit_direction = r.direction().to_unit_vector();
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
    let origin = Vec3(0.0, 0.0, 0.0);

    // direction vectors defining the view
    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 2.0, 0.0);

    for y in 0 .. HEIGHT {
        let j = HEIGHT - 1 - y;
        for i in 0 .. WIDTH {
            let u = i as f32 / WIDTH as f32;
            let v = j as f32 / HEIGHT as f32;
            let r = Ray(origin, lower_left_corner + u * horizontal + v * vertical);
            let rgb = color(r).to_u8();
            println!("{} {} {}", rgb[0], rgb[1], rgb[2]);
        }
    }
}
