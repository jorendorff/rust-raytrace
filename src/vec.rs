use std::ops::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn x(&self) -> f32 { self.0 }
    pub fn y(&self) -> f32 { self.1 }
    pub fn z(&self) -> f32 { self.2 }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(self.1 * other.2 - self.2 * other.1,
             -(self.0 * other.2 - self.2 * other.0),
             self.0 * other.1 - self.1 * other.0)
    }

    pub fn squared_length(self) -> f32 { self.dot(self) }

    pub fn to_u8(&self) -> [u8;3] {
        let u = |f| (f * 255.9) as i32 as u8;
        [u(self.0), u(self.1), u(self.2)]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
