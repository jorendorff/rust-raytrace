use rand::random;
use vec::{Vec3, Ray};
use model::Hit;

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
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Scatter {
        let reflected = reflect(r_in.direction(), hit.normal);
        let scattered = Ray(hit.p, reflected + self.fuzz * random_in_unit_sphere());

        Scatter {
            color: self.albedo,
            ray: if scattered.direction().dot(hit.normal) <= 0.0 {
                None
            } else {
                Some(scattered)
            }
        }
    }
}

