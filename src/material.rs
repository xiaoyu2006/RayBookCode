use crate::{color::Color, hit::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}
