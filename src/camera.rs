use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64) -> Self {
        let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
        let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
        let lower_left_corner = &(&(&origin - &(&horizontal / 2.0)) - &(&vertical / 2.0)) - &Vec3 { x: 0.0, y: 0.0, z: focal_length };

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin.clone(),
            direction: &(&self.lower_left_corner + &(&(&self.horizontal * u) + &(&self.vertical * v))) - &self.origin,
        }
    }
}
