use std::error::Error;
use std::rc::Rc;

use rayst::aarect::{XYRect, XZRect, YZRect};
use rayst::box3d::Box3D;
use rayst::bvh::BVHNode;
use rayst::constant_medium::ConstantMedium;
use rayst::hittable::{RotateY, Translate};
use rayst::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use rayst::moving_sphere::MovingSphere;
use rayst::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use rayst::vec3::Vec3;
use rayst::{
    camera::Camera, color::Color, hittable::Hittable, hittable_list::HittableList, ray::Ray,
    sphere::Sphere, vec3::Point,
};

use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    // Image.
    let mut aspect_ratio: f64 = 16.0 / 9.0;
    let mut image_width: u32 = 400;
    let mut samples_per_pixel: u32 = 100;
    let max_depth = 50;

    // Random number generator.
    let mut rng = rayst::thread_rng();

    // World.
    let mut look_from = Point::new(13.0, 2.0, 3.0);
    let mut look_at = Point::default();
    let mut vfov = 20.0;
    let mut background = Color::default();
    let mut aperture = 0.0;
    let scene = match 0 {
        1 => {
            background = Color::new(0.7, 0.8, 1.0);
            aperture = 0.1;
            random_scene()
        }
        2 => {
            background = Color::new(0.7, 0.8, 1.0);
            two_spheres()
        }
        3 => {
            background = Color::new(0.7, 0.8, 1.0);
            two_perlin_spheres(&mut rng)
        }
        4 => {
            background = Color::new(0.7, 0.8, 1.0);
            earth()
        }
        5 => {
            samples_per_pixel = 400;
            look_from = Point::new(26.0, 3.0, 6.0);
            look_at = Point::y(2.0);
            simple_light(&mut rng)
        }
        6 => {
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 400;
            look_from = Point::new(278.0, 278.0, -800.0);
            look_at = Point::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            cornell_box()
        }
        _ => {
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            look_from = Point::new(278.0, 278.0, -800.0);
            look_at = Point::new(278.0, 278.0, 0.0);
            vfov = 40.0;
            cornell_smoke()
        }
    };

    let image_height = (image_width as f64 / aspect_ratio) as u32;
    //let world = BVHNode::from_hittable_list(scene, 0.0, 1.0);
    let world = scene;

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

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let mut c = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = cam.get_ray(&mut rng, u, v);
                c += ray_color(&mut rng, &r, background, &world, max_depth);
            }
            println!("{}", c.adjust_and_format(samples_per_pixel));
        }
    }

    eprintln!("\nDone.");

    Ok(())
}

fn ray_color(
    rng: &mut rayst::Rng,
    r: &Ray,
    background: Color,
    world: &impl Hittable,
    depth: i32,
) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = rec.material.scatter(rng, r, &rec) {
            return attenuation * ray_color(rng, &scattered, background, world, depth - 1);
        }
        return rec.material.emitted(rec.u, rec.v, &rec.p);
    }

    background
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

fn two_perlin_spheres(rng: &mut rayst::Rng) -> HittableList {
    let mut world = HittableList::default();
    let pertext = Rc::new(NoiseTexture::new(rng, 4.0));

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

fn earth() -> HittableList {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg").unwrap());
    let earth_surface = Rc::new(Lambertian::new_with_texture(earth_texture));
    let globe = Rc::new(Sphere::new(Point::default(), 2.0, earth_surface));

    let mut world = HittableList::default();
    world.add(globe);

    world
}

fn simple_light(rng: &mut rayst::Rng) -> HittableList {
    let mut world = HittableList::default();

    let pertext = Rc::new(NoiseTexture::new(rng, 4.0));
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

    let difflight = Rc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Rc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    world
}

fn cornell_box() -> HittableList {
    let mut world = HittableList::default();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Rc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Rc<dyn Hittable> = Rc::new(Box3D::new(
        Point::default(),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let mut box2: Rc<dyn Hittable> = Rc::new(Box3D::new(
        Point::default(),
        Point::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    world
}

fn cornell_smoke() -> HittableList {
    let mut world = HittableList::default();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    world.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Rc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    world.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Rc<dyn Hittable> = Rc::new(Box3D::new(
        Point::default(),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    box1 = Rc::new(ConstantMedium::new(box1, 0.01, Color::new(0.0, 0.0, 0.0)));
    world.add(box1);

    let mut box2: Rc<dyn Hittable> = Rc::new(Box3D::new(
        Point::default(),
        Point::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    box2 = Rc::new(ConstantMedium::new(box2, 0.01, Color::new(1.0, 1.0, 1.0)));
    world.add(box2);

    world
}
