use std::rc::Rc;

use crate::color::Color;
use crate::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::util::reflect;
use crate::vec3::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub material: Rc<dyn Material>,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = &ray.origin - &self.center;
        let a = dot(&ray.direction, &ray.direction);
        let half_b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a; // Try the other root.
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = &(&p - &self.center) / self.radius;

        Some(HitRecord::from_outward_normal(
            ray,
            p,
            self.material.clone(),
            outward_normal,
            root,
        ))
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = (&rec.normal + &Vec3::random_unit_vector()).unit_vec();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.unit_vec();
        }

        let scattered = Ray {
            origin: rec.p.clone(),
            direction: scatter_direction,
        };
        let attenuation = self.albedo.clone();
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&ray.direction.unit_vec(), &rec.normal);
        let scattered = Ray {
            origin: rec.p.clone(),
            direction: &reflected + &(&Vec3::random_in_unit_sphere() * self.fuzz),
        };
        let attenuation = self.albedo.clone();
        if dot(&scattered.direction, &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
