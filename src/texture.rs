use std::rc::Rc;

use image::{ImageBuffer, Rgb};
use rand::prelude::ThreadRng;

use crate::{color::Color, perlin::Perlin, vec3::Point};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> Self {
        Self {
            odd: Rc::new(SolidColor::new(c1)),
            even: Rc::new(SolidColor::new(c2)),
        }
    }

    pub fn new_with_texture(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(rng: &mut ThreadRng, scale: f64) -> Self {
        Self {
            noise: Perlin::new(rng),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        //Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(&(self.scale * p)))
        //Color::new(1.0, 1.0, 1.0) * self.noise.turb(&(self.scale * p), 7)
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Option<ImageBuffer<Rgb<f32>, Vec<f32>>>,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Option<Self> {
        image::io::Reader::open(filename)
            .ok()
            .and_then(|f| f.decode().ok())
            .map(|img| Self {
                data: Some(img.to_rgb32f()),
            })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        match self.data.as_ref() {
            None => Color::new(0.0, 1.0, 1.0),
            Some(data) => {
                let u = u.clamp(0.0, 1.0);
                let v = 1.0 - v.clamp(0.0, 1.0);
                let i = ((u * (data.width() as f64)) as u32).min(data.width() - 1);
                let j = ((v * (data.height() as f64)) as u32).min(data.height() - 1);
                let pixel = data.get_pixel(i, j);

                Color::new(pixel.0[0] as f64, pixel.0[1] as f64, pixel.0[2] as f64)
            }
        }
    }
}
