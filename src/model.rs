use vec::{Vec3, Ray};
use rand::random;

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

pub struct Scatter {
    pub color: Vec3,
    pub ray: Option<Ray>
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &Hit) -> Scatter;
}

pub struct Lambertian {
    pub albedo: Vec3
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * random::<Vec3>() - Vec3(1.0, 1.0, 1.0);
        if p.dot(p) < 1.0 {
            return p;
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &Hit) -> Scatter {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        Scatter {
            color: self.albedo,
            ray: Some(Ray(hit.p, target - hit.p))
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub struct Metal {
    pub albedo: Vec3
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        Scatter {
            color: self.albedo,
            ray: Some(Ray(hit.p, reflect(r_in.direction(), hit.normal)))
        }
    }
}
