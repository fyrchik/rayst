use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

use rand::distributions::Uniform;
use rand::{distributions::Distribution, Rng};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn new_eq(x: f64) -> Self {
        Self::new(x, x, x)
    }

    pub fn x(x: f64) -> Self {
        Self::new(x, 0.0, 0.0)
    }

    pub fn y(y: f64) -> Self {
        Self::new(0.0, y, 0.0)
    }

    pub fn z(z: f64) -> Self {
        Self::new(0.0, 0.0, z)
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline]
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    pub fn normalize(self) -> Vec3 {
        self / self.length()
    }

    #[inline]
    pub fn random<R: Rng, D: Distribution<f64>>(rng: &mut R, dist: &D) -> Vec3 {
        Self::new(dist.sample(rng), dist.sample(rng), dist.sample(rng))
    }

    pub fn near_zero(&self) -> bool {
        const EPSILON: f64 = 1e-8;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }

    pub fn reflect(&self, v: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(v) * v
    }

    pub fn refract(&self, normal: Vec3, eta1_over_eta2: f64) -> Vec3 {
        let cos_theta = (-self.dot(normal)).min(1.0);
        let r_out_perp = eta1_over_eta2 * (*self + cos_theta * normal);
        let r_out_para = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_para
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let uni = Uniform::<f64>::new(-1.0, 1.0);
    loop {
        let p = Vec3::random(&mut rng, &uni);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let p = random_in_unit_sphere();
    if p.dot(normal) > 0.0 {
        p
    } else {
        -p
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, c: f64) -> Vec3 {
        Vec3 {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, c: f64) {
        self.x *= c;
        self.y *= c;
        self.z *= c;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, c: f64) -> Vec3 {
        Vec3 {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, c: f64) {
        self.x /= c;
        self.y /= c;
        self.z /= c;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn operations() {
        const EPSILON: f64 = 0.00000001;

        let mut a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        a += b;
        assert!((a.x - 5.0).abs() < EPSILON);
        assert!((a.y - 7.0).abs() < EPSILON);
        assert!((a.z - 9.0).abs() < EPSILON);

        a /= 2.0;
        assert!((a.x - 2.5).abs() < EPSILON);
        assert!((a.y - 3.5).abs() < EPSILON);
        assert!((a.z - 4.5).abs() < EPSILON);

        a *= 4.0;
        assert!((a.x - 10.0).abs() < EPSILON);
        assert!((a.y - 14.0).abs() < EPSILON);
        assert!((a.z - 18.0).abs() < EPSILON);

        a = -a;
        assert!((a.x + 10.0).abs() < EPSILON);
        assert!((a.y + 14.0).abs() < EPSILON);
        assert!((a.z + 18.0).abs() < EPSILON);

        assert_eq!(a.x, a[0]);
        assert_eq!(a.y, a[1]);
        assert_eq!(a.z, a[2]);

        assert!((a.length_squared() - 620.0) < EPSILON);
        assert!((a.length() - 24.899799196) < EPSILON);

        let x = Vec3::new(2.0, 2.0, 2.0);
        let n = x.normalize();
        println!("{:?}", n);
        assert!((n.x - 0.577350269).abs() < EPSILON);
        assert!((n.y - 0.577350269).abs() < EPSILON);
        assert!((n.z - 0.577350269).abs() < EPSILON);
        assert!((n.length() - 1.0).abs() < EPSILON);
    }
}
