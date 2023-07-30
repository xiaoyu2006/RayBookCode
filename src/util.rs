use crate::vec3::dot;
use crate::vec3::Vec3;

pub fn random_double() -> f64 {
    fastrand::f64()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp<T: std::cmp::PartialOrd>(x: T, min: T, max: T) -> T {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(&(n * dot(&v, &n)) * 2.0)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-uv, n);
    let r_out_parallel = &(uv + &(n * cos_theta)) * etai_over_etat;
    let r_out_perp = n * -(1.0 - r_out_parallel.length_squared()).sqrt();
    &r_out_parallel + &r_out_perp
}

pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
