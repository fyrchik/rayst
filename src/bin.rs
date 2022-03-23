use std::error::Error;
use std::io::{self, Write};
use std::rc::Rc;

use rayst::{
    camera::Camera,
    color::Color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    vec3::{Point, Vec3},
};

use rand::{distributions::Uniform, Rng};

fn main() -> Result<(), Box<dyn Error>> {
    // Image.
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;

    // World.
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point::z(-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera.
    let cam = Camera::default();

    let mut stderr = io::stderr();
    let mut rng = rand::thread_rng();
    let uni = Uniform::<f64>::new(0.0, 1.0);

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        write!(&mut stderr, "\rScanlines remaining: {}", j)?;

        for i in 0..image_width {
            let mut c = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.sample(uni)) / (image_width - 1) as f64;
                let v = (j as f64 + rng.sample(uni)) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                c += ray_color(&r, &world);
            }
            println!("{}", c.adjust_and_format(samples_per_pixel));
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
