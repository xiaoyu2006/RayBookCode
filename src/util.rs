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
