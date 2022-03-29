use rand::{rngs::ThreadRng, Rng};

use crate::vec3::{Point, Vec3};

pub struct Perlin {
    ranfloat: [Vec3; Self::POINT_COUNT],
    perm_x: [u16; Self::POINT_COUNT],
    perm_y: [u16; Self::POINT_COUNT],
    perm_z: [u16; Self::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut s = Self {
            ranfloat: [Vec3::default(); Self::POINT_COUNT],
            perm_x: [0; Self::POINT_COUNT],
            perm_y: [0; Self::POINT_COUNT],
            perm_z: [0; Self::POINT_COUNT],
        };

        for x in s.ranfloat.iter_mut() {
            *x = Vec3::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize()
        }
        Self::generate_perm(rng, &mut s.perm_x);
        Self::generate_perm(rng, &mut s.perm_y);
        Self::generate_perm(rng, &mut s.perm_z);

        s
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let mut u = p.x - p.x.floor();
        let mut v = p.y - p.y.floor();
        let mut w = p.z - p.z.floor();

        u = u.powi(2) * u.mul_add(-2.0, 3.0);
        v = v.powi(2) * v.mul_add(-2.0, 3.0);
        w = w.powi(2) * w.mul_add(-2.0, 3.0);

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for (di, ri) in c.iter_mut().enumerate() {
            for (dj, rj) in ri.iter_mut().enumerate() {
                for (dk, rk) in rj.iter_mut().enumerate() {
                    *rk = self.ranfloat[(self.perm_x[((i + (di as i32)) & 255) as usize]
                        ^ self.perm_y[((j + (dj as i32)) & 255) as usize]
                        ^ self.perm_z[((k + (dk as i32)) & 255) as usize])
                        as usize]
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    pub fn turb(&self, p: &Point, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u.powi(2) * u.mul_add(-2.0, 3.0);
        let vv = v.powi(2) * v.mul_add(-2.0, 3.0);
        let ww = w.powi(2) * w.mul_add(-2.0, 3.0);
        let mut accum = 0.0;

        for (i, ri) in c.iter().enumerate() {
            for (j, rj) in ri.iter().enumerate() {
                for (k, rk) in rj.iter().enumerate() {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * rk.dot(weight_v);
                }
            }
        }
        accum
    }

    #[allow(unused)]
    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Note: this code differs a lot from the one in the book.
        let c00 = c[0][0][0] * (1.0 - u) + c[1][0][0] * u;
        let c01 = c[0][0][1] * (1.0 - u) + c[1][0][1] * u;
        let c10 = c[0][1][0] * (1.0 - u) + c[1][1][0] * u;
        let c11 = c[0][1][1] * (1.0 - u) + c[1][1][1] * u;

        let c0 = c00 * (1.0 - v) + c10 * v;
        let c1 = c01 * (1.0 - v) + c11 * v;

        c0 * (1.0 - w) + c1 * w
    }

    pub fn generate_perm(rng: &mut ThreadRng, p: &mut [u16; Self::POINT_COUNT]) {
        for (i, v) in p.iter_mut().enumerate() {
            *v = i as u16;
        }
        Self::permute(rng, p)
    }

    pub fn permute(rng: &mut ThreadRng, p: &mut [u16; Self::POINT_COUNT]) {
        for i in (1..p.len()).rev() {
            let target = rng.gen_range(0..i);
            p.swap(i, target)
        }
    }
}
