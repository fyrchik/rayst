use std::error::Error;
use std::io::{self, Write};
use std::rc::Rc;

use rayst::bvh::BVHNode;
use rayst::material::{Dielectric, Lambertian, Metal};
use rayst::moving_sphere::MovingSphere;
use rayst::texture::{CheckerTexture, NoiseTexture};
use rayst::vec3::Vec3;
use rayst::{
    camera::Camera, color::Color, hittable::Hittable, hittable_list::HittableList, ray::Ray,
    sphere::Sphere, vec3::Point,
};

use rand::{rngs::ThreadRng, Rng};

fn main() -> Result<(), Box<dyn Error>> {
    // Image.
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel: u32 = 100;
    let max_depth = 50;

    // World.
    let look_from;
    let look_at;
    let vfov;
    let mut aperture = 0.0;
    let scene = match 0 {
        1 => {
            look_from = Point::new(13.0, 2.0, 3.0);
            look_at = Point::default();
            vfov = 20.0;
            aperture = 0.1;
            random_scene()
        }
        2 => {
            look_from = Point::new(13.0, 2.0, 3.0);
            look_at = Point::default();
            vfov = 20.0;
            two_spheres()
        }
        _ => {
            look_from = Point::new(13.0, 2.0, 3.0);
            look_at = Point::default();
            vfov = 20.0;
            two_perlin_spheres()
        }
    };
    let world = BVHNode::from_hittable_list(scene, 0.0, 1.0);

    // Camera.
    let vup = Vec3::y(1.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0..1.0,
    );

    let mut stderr = io::stderr();
    let mut rng = rand::thread_rng();

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        write!(&mut stderr, "\rScanlines remaining: {}", j)?;

        for i in 0..image_width {
            let mut c = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = cam.get_ray(&mut rng, u, v);
                c += ray_color(&mut rng, &r, &world, max_depth);
            }
            println!("{}", c.adjust_and_format(samples_per_pixel));
        }
    }

    write!(&mut stderr, "\nDone.\n")?;

    Ok(())
}

fn ray_color(rng: &mut ThreadRng, r: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(rng, r, &rec) {
            return attenuation * ray_color(rng, &scattered, world, depth - 1);
        }
        return Color::default();
    }

    let dir = r.dir.normalize();
    let t = 0.5 * (dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let checker = Rc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new(
        Point::y(-1000.0),
        1000.0,
        Rc::new(Lambertian::new_with_texture(checker)),
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = (&mut rng).gen();
            let center = Point::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let object: Rc<dyn Hittable> = if choose_mat < 0.8 {
                    let albedo: Color = rng.gen::<Color>(); //+ rng.gen::<Color>();
                    let center2 = center + Vec3::y(rng.gen_range(0.0..0.5));
                    Rc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0..1.0,
                        0.2,
                        Rc::new(Lambertian::new(albedo)),
                    ))
                } else if choose_mat < 0.95 {
                    let albedo: Color = Color::new(
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                        rng.gen_range(0.5..1.0),
                    );
                    Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(albedo, rng.gen_range(0.0..0.5))),
                    ))
                } else {
                    Rc::new(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5))))
                };

                world.add(object);
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point::y(1.0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn two_spheres() -> HittableList {
    let mut world = HittableList::default();
    let checker = Rc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new(
        Point::y(-10.0),
        10.0,
        Rc::new(Lambertian::new_with_texture(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point::y(10.0),
        10.0,
        Rc::new(Lambertian::new_with_texture(checker)),
    )));
    world
}

fn two_perlin_spheres() -> HittableList {
    let mut rng = rand::thread_rng();
    let mut world = HittableList::default();
    let pertext = Rc::new(NoiseTexture::new(&mut rng, 4.0));

    world.add(Rc::new(Sphere::new(
        Point::y(-1000.0),
        1000.0,
        Rc::new(Lambertian::new_with_texture(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point::y(2.0),
        2.0,
        Rc::new(Lambertian::new_with_texture(pertext)),
    )));
    world
}
