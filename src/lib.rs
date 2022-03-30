pub mod aabb;
pub mod aarect;
pub mod box3d;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod moving_sphere;
pub mod perlin;
pub mod ray;
pub mod sphere;
pub mod texture;
pub mod vec3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
