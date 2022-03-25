use crate::{
    ray::Ray,
    vec3::{random_in_unit_disc, Point, Vec3},
};

use rand::rngs::ThreadRng;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point::new_eq(0.0),
            Point::z(-1.0),
            Vec3::y(1.0),
            90.0,
            16.0 / 9.0,
            2.0,
            1.0,
        )
    }
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        theta: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let h = (theta.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;
        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, r: &mut ThreadRng, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disc(r);
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin
                - offset,
        }
    }
}
