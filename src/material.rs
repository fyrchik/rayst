use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::random_in_unit_sphere};

use rand::{rngs::ThreadRng, Rng};

pub trait Material {
    fn scatter(&self, rng: &mut ThreadRng, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut ThreadRng, _: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_in_unit_sphere(rng);
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((Ray::new(rec.p, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, rng: &mut ThreadRng, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.dir.normalize().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere(rng));
        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            ri: refraction_index,
        }
    }

    fn reflectance(cosine: f64, rr: f64) -> f64 {
        // Use Schlick's approximation.
        let r0 = (1.0 - rr) / (1.0 + rr);
        let r1 = r0.powi(2);
        r1 + (1.0 - r1) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, rng: &mut ThreadRng, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if rec.front_face {
            self.ri.recip()
        } else {
            self.ri
        };
        let unit_direction = r_in.dir.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };
        Some((Ray::new(rec.p, direction), Color::new(1.0, 1.0, 1.0)))
    }
}
