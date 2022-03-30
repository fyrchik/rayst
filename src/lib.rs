use rand::SeedableRng;

pub mod aabb;
pub mod aarect;
pub mod box3d;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod constant_medium;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod moving_sphere;
pub mod perlin;
pub mod ray;
pub mod sphere;
pub mod texture;
pub mod vec3;

/// Rng is a concrete type for a random number generator used across the crate.
/// It is here to easily switch between generators.
pub type Rng = rand_xoshiro::Xoshiro256PlusPlus;

pub fn thread_rng() -> Rng {
    rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(42)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
