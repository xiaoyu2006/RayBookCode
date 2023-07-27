use crate::{vec3::Vec3, util::clamp};

pub type Color = Vec3;

impl Color {
    pub fn write(&self, samples: i32) -> String {
        let scale = 1.0 / (samples as f64);
        let normalized = self * scale;

        format!(
            "{} {} {}",
            (256.0 * clamp(normalized.x, 0.0, 0.999)) as i32,
            (256.0 * clamp(normalized.y, 0.0, 0.999)) as i32,
            (256.0 * clamp(normalized.z, 0.0, 0.999)) as i32,
        )
    }
}
