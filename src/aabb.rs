use crate::{ray::Ray, vec3::Point};

/// AABB stays for Axis-Aligned Bounding Box.
#[derive(Clone)]
pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl AABB {
    pub fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = r.dir[a].recip();
            let mut t0 = (self.min[a] - r.orig[a]) * inv_d;
            let mut t1 = (self.max[a] - r.orig[a]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0)
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn union(&self, other: &Self) -> Self {
        let small = Point::new(
            f64::min(self.min.x, other.min.x),
            f64::min(self.min.y, other.min.y),
            f64::min(self.min.z, other.min.z),
        );
        let big = Point::new(
            f64::max(self.max.x, other.max.x),
            f64::max(self.max.y, other.max.y),
            f64::max(self.max.z, other.max.z),
        );
        Self {
            min: small,
            max: big,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Point;

    #[test]
    fn union() {
        let a = AABB::new(Point::new(-1.0, 0.0, 1.0), Point::new(0.0, 5.0, 1.5));
        let b = AABB::new(Point::new(-0.5, -2.0, 1.2), Point::new(0.8, 10.0, 1.3));
        let c = AABB::union(&a, &b);

        assert!((c.min - Point::new(-1.0, -2.0, 1.0)).near_zero());
        assert!((c.max - Point::new(0.8, 10.0, 1.5)).near_zero())
    }
}
