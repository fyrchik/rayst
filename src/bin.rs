use std::error::Error;
use std::io::{self, Write};
use std::rc::Rc;

use rayst::vec3;
use rayst::{
    camera::Camera,
    color::Color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    sphere::Sphere,
    vec3::Point,
};

use rand::{distributions::Uniform, Rng};

fn main() -> Result<(), Box<dyn Error>> {
    // Image.
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

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
                c += ray_color(&r, &world, max_depth);
            }
            println!("{}", c.adjust_and_format(samples_per_pixel));
        }
    }

    write!(&mut stderr, "\nDone.\n")?;

    Ok(())
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::default();
    }
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut rng = rand::thread_rng();
        let uni = Uniform::new(-1.0, 1.0);
        let target = rec.p + vec3::random_in_hemisphere(&mut rng, &uni, rec.normal);
        return 0.5 * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let dir = r.dir.normalize();
    let t = 0.5 * (dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
