use crate::vector;
use crate::ray::Ray;

pub struct Camera {
    origin: vector::Vec3,
    horizontal: vector::Vec3,
    vertical: vector::Vec3,
    lower_left_corner: vector::Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: vector::fsize) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = vector::Vec3(0.0, 0.0, 0.0);
        let horizontal = vector::Vec3(viewport_width, 0.0, 0.0);
        let vertical = vector::Vec3(0.0, viewport_height, 0.0);
        let lower_left_corner = &origin - (&horizontal / 2.0) - &vertical / 2.0 - vector::Vec3(0.0, 0.0, focal_length);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner, 
        }
    }

    pub fn get_ray(&self, horizontal_line: vector::fsize, vertical_line: vector::fsize) -> Ray {
        Ray::new(
            self.origin.clone(),
            &self.lower_left_corner + horizontal_line * &self.horizontal + (&(vertical_line * &self.vertical) - &self.origin),
        )
    }
}