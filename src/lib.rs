pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod moving_sphere;
pub mod ray;
pub mod sphere;
pub mod vec3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
