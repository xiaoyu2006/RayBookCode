use crate::ray::Ray;
use crate::vec3::{cross, Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = crate::util::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&lookfrom - &lookat).unit_vec();
        let u = cross(&vup, &w).unit_vec();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = &(&u * viewport_width) * focus_dist;
        let vertical = &(&v * viewport_height) * focus_dist;
        let lower_left_corner =
            &(&(&origin - &(&horizontal / 2.0)) - &(&vertical / 2.0)) - &(&w * focus_dist);

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = &(&Vec3::random_in_unit_disk() * self.lens_radius) * 2.0;
        let offset = &(&self.u * rd.x) + &(&self.v * rd.y);

        Ray {
            origin: &self.origin + &offset,
            direction: &(&(&self.lower_left_corner
                + &(&(&self.horizontal * s) + &(&self.vertical * t)))
                - &self.origin)
                - &offset,
        }
    }
}
