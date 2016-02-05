mod vec;

use vec::Vec3;

fn main() {
    const WIDTH: usize = 200;
    const HEIGHT: usize = 100;

    println!("P3");
    println!("{} {}", WIDTH, HEIGHT);
    println!("255");
    for y in 0 .. HEIGHT {
        let j = HEIGHT - 1 - y;
        for i in 0 .. WIDTH {
            let rgb = Vec3 (
                i as f32 / WIDTH as f32,
                j as f32 / HEIGHT as f32,
                0.2).to_u8();
            println!("{} {} {}", rgb[0], rgb[1], rgb[2]);
        }
    }
}
