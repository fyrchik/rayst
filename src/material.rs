use std::rc::Rc;

use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::{random_in_unit_sphere, Point},
};

use rand::Rng;

pub trait Material {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        Color::default()
    }
    fn scatter(&self, rng: &mut crate::Rng, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new_with_texture(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut crate::Rng, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + random_in_unit_sphere(rng);
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        Some((
            Ray::new(rec.p, scatter_direction, r_in.time),
            self.albedo.value(rec.u, rec.v, &rec.p),
        ))
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
    fn scatter(&self, rng: &mut crate::Rng, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = r_in.dir.normalize().reflect(rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(rng),
            r_in.time,
        );
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
    fn scatter(&self, rng: &mut crate::Rng, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
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
        Some((
            Ray::new(rec.p, direction, r_in.time),
            Color::new(1.0, 1.0, 1.0),
        ))
    }
}

pub struct DiffuseLight {
    pub emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(c: Color) -> Self {
        Self {
            emit: Rc::new(SolidColor::new(c)),
        }
    }

    pub fn new_with_texture(texture: Rc<dyn Texture>) -> Self {
        Self { emit: texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _rng: &mut crate::Rng,
        _r_in: &Ray,
        _rec: &HitRecord,
    ) -> Option<(Ray, Color)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &Point) -> Color {
        self.emit.value(u, v, p)
    }
}

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(c: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(c)),
        }
    }

    pub fn new_with_texture(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, rng: &mut crate::Rng, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        Some((
            Ray::new(rec.p, random_in_unit_sphere(rng), r_in.time),
            self.albedo.value(rec.u, rec.v, &rec.p),
        ))
    }
}
