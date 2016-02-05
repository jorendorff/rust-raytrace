mod vec;

use vec::{Vec3, Ray};

struct Hit {
    t: f32,
    p: Vec3,
    normal: Vec3
}

trait HitTest {
    fn hit(&self, r: &Ray) -> Option<Hit>;
}

struct Sphere {
    center: Vec3,
    radius: f32
}

impl HitTest for Sphere {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = 2.0 * oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let d = 1.0 / (2.0 * a);
            // Assuming r.origin() is outside the sphere, we only need to consider
            // this hit and not the other hit (with the positive square root)
            let t = (-b - discriminant.sqrt()) * d;
            if t <= 0.0 {
                None
            } else {
                let p = r.point_at_parameter(t);
                Some(Hit {
                    t: t,
                    p: p,
                    normal: p - self.center
                })
            }
        } else {
            None
        }
    }
}



fn color(r: Ray) -> Vec3 {
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    const SKY_BLUE: Vec3 = Vec3(0.5, 0.7, 1.0);
    const RED: Vec3 = Vec3(1.0, 0.0, 0.0);
    const GLANCING: Vec3 = Vec3(0.3, 0.0, 0.5);

    let sphere = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.8
    };

    let unit_direction = r.direction().to_unit_vector();
    if let Some(hit) = sphere.hit(&r) {
        let d = hit.normal.to_unit_vector().dot(unit_direction).abs();  // directness
        return (1.0 - d) * GLANCING + d * RED;
    }
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * WHITE + t * SKY_BLUE
}


fn main() {
    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;

    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");

    // camera position (a point)
    let origin = Vec3(0.0, 0.0, 0.0);

    // direction vectors defining the view
    let lower_left_corner = Vec3(-2.0, -1.5, -1.0);
    let horizontal = Vec3(4.0, 0.0, 0.0);
    let vertical = Vec3(0.0, 3.0, 0.0);

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
