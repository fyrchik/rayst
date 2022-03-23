use std::error::Error;
use std::io::{self, Write};

use rayst::{
    color::Color,
    ray::Ray,
    vec3::{Point, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Image.
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera.
    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut stderr = io::stderr();

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        write!(&mut stderr, "\rScanlines remaining: {}", j)?;

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let c = ray_color(&r);

            println!("{}", c);
        }
    }

    write!(&mut stderr, "\nDone.\n")?;

    Ok(())
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> bool {
    let oc = r.orig - *center;
    let a = r.dir.dot(r.dir);
    let b = 2.0 * oc.dot(r.dir);
    let c = oc.dot(oc) - radius * radius;
    let d = b * b - 4.0 * a * c;
    d > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let dir = r.dir.normalize();
    let t = 0.5 * (dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
