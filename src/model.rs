use vec::{Vec3, Ray};
use materials::Material;

#[derive(Clone, Copy)]
pub struct Hit<'obj> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'obj Material
}

pub trait HitTest {
    fn hit(&self, r: &Ray) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<Material>
}

impl HitTest for Sphere {
    fn hit<'a>(&'a self, r: &Ray) -> Option<Hit<'a>> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = 2.0 * oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let d = 1.0 / (2.0 * a);
            // Assuming r.origin is outside the sphere, we only need to consider
            // this hit and not the other hit (with the positive square root)
            let t = (-b - discriminant.sqrt()) * d;
            if t <= 0.0 {
                None
            } else {
                let p = r.point_at_parameter(t);
                Some(Hit {
                    t: t,
                    p: p,
                    normal: (p - self.center) / self.radius,
                    material: &*self.material
                })
            }
        } else {
            None
        }
    }
}

impl HitTest for Vec<Box<HitTest>> {
    fn hit(&self, r: &Ray) -> Option<Hit> {
        let mut best = None;
        for child in self {
            if let Some(hit) = child.hit(r) {
                match best {
                    None => best = Some(hit),
                    Some(prev) => if hit.t < prev.t {
                        best = Some(hit)
                    }
                }
            }
        }
        best
    }
}
