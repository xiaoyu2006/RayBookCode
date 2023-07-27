pub mod sphere;

use crate::{Vec3, Ray};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    fn from_outward_normal(ray: &Ray, p: Vec3, outward_normal: Vec3, t: f64) -> Self {
        let front_face = &ray.direction * &outward_normal < 0.0;
        let normal = if front_face { outward_normal } else { -&outward_normal };
        Self { p, normal, t, front_face }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;  // Objects overlaps
        let mut hit_record = None;

        for object in &self.objects {
            if let Some(temp_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_record.t;
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }
}
