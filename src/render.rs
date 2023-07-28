use crate::{
    camera::Camera, color::Color, hit::HittableList, ray::Ray, util::random_double, vec3::Vec3,
};

pub fn ray_trace(ray: &Ray, world: &HittableList, depth: i64) -> Color {
    const BLACK: Vec3 = Color {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    if depth <= 0 {
        return BLACK;
    }

    let rec = world.hit(ray, 0.001, f64::INFINITY);

    if let Some(rec) = rec {
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            return &ray_trace(&scattered, world, depth - 1) * &attenuation;
        } else {
            return BLACK;
        }
    }

    // Gradient background
    let unit = ray.direction.unit_vec();
    let t = 0.5 * (unit.y + 1.0);
    &(&Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    } * (1.0 - t))
        + &(&Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        } * t)
}

pub fn render(
    img_width: i32,
    img_height: i32,
    camera: Camera,
    world: HittableList,
    gamma: f64,
    samples: i32,
) {
    const MAX_DEPTH: i64 = 50;

    println!("P3");
    println!("{} {}", img_width, img_height);
    println!("255");

    for j in (0..img_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..img_width {
            let mut color = Color {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            for _ in 0..samples {
                let u = (i as f64 + random_double()) / (img_width - 1) as f64;
                let v = (j as f64 + random_double()) / (img_height - 1) as f64;
                let r = camera.get_ray(u, v);
                color = &color + &ray_trace(&r, &world, MAX_DEPTH);
            }
            println!("{}", color.write(gamma, samples));
        }
    }

    eprintln!("\nDone.");
}
