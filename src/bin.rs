use std::error::Error;
use std::io::{self, Write};
use std::rc::Rc;

use rayst::{
    color::Color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    vec3::{Point, Vec3},
};

fn main() -> Result<(), Box<dyn Error>> {
    // Image.
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // World.
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point::z(-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera.
    let viewport_height: f64 = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::default();
    let horizontal = Vec3::x(viewport_width);
    let vertical = Vec3::y(viewport_height);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::z(focal_length);

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
            let c = ray_color(&r, &world);

            println!("{}", c);
        }
    }

    write!(&mut stderr, "\nDone.\n")?;

    Ok(())
}

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (Color::new_from_vec3(rec.normal + Vec3::new_eq(1.0)));
    }

    let dir = r.dir.normalize();
    let t = 0.5 * (dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
