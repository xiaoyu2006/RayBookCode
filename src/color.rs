use crate::{util::clamp, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn write(&self, gamma: f64, samples: i32) -> String {
        let scale = 1.0 / (samples as f64);
        let normalized = self * scale;
        let normalized = Vec3 {
            x: normalized.x.powf(1.0 / gamma),
            y: normalized.y.powf(1.0 / gamma),
            z: normalized.z.powf(1.0 / gamma),
        };

        format!(
            "{} {} {}",
            (256.0 * clamp(normalized.x, 0.0, 0.999)) as i32,
            (256.0 * clamp(normalized.y, 0.0, 0.999)) as i32,
            (256.0 * clamp(normalized.z, 0.0, 0.999)) as i32,
        )
    }
}
