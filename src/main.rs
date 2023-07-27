pub mod vec3;
pub mod color;
pub mod ray;
pub mod hit;
pub mod camera;
pub mod util;

use vec3::Vec3;
use color::Color;
use ray::Ray;
use hit::{sphere::Sphere, HittableList};

use crate::{camera::Camera, util::random_double};

fn ray_trace(ray: &Ray, world: &HittableList) -> Color {
    let rec = world.hit(ray, 0.0, f64::INFINITY);

    if let Some(rec) = rec {
        return &(&rec.normal + &Color { x: 1.0, y: 1.0, z: 1.0 }) * 0.5;
    }

    // Gradient background
    let unit = ray.direction.unit_vec();
    let t = 0.5 * (unit.y + 1.0);
    &(&Color { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t)) + &(&Color { x: 0.5, y: 0.7, z: 1.0 } * t)
}

fn render(img_width: i32, img_height: i32, camera: Camera, world: HittableList, samples: i32) {
    println!("P3");
    println!("{} {}", img_width, img_height);
    println!("255");

    for j in (0..img_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..img_width {
            let mut color = Color { x: 0.0, y: 0.0, z: 0.0 };
            for _ in 0..samples {
                let u = ( i as f64 + random_double() ) / (img_width - 1) as f64;
                let v = ( j as f64 + random_double() ) / (img_height - 1) as f64;
                let r = camera.get_ray(u, v);
                color = &color + &ray_trace(&r, &world);
            }
            println!("{}", color.write(samples));
        }
    }

    eprintln!("\nDone.");
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 50;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    // let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    // let horizontal = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    // let vertical = Vec3 { x: 0.0, y: viewport_height, z: 0.0 };
    // let lower_left_corner = &(&(&origin - &(&horizontal / 2.0)) - &(&vertical / 2.0)) - &Vec3 { x: 0.0, y: 0.0, z: focal_length };
    let camera = Camera::new(viewport_height, viewport_width, focal_length);

    // Objects
    let mut world = hit::HittableList::new();
    world.add(Box::new(Sphere { center: Vec3 { x: 0.0, y: 0.0, z: -1.0 }, radius: 0.5 }));
    world.add(Box::new(Sphere { center: Vec3 { x: 0.0, y: -100.5, z: -1.0 }, radius: 100.0 }));

    // Render
    render(IMAGE_WIDTH, IMAGE_HEIGHT, camera, world, SAMPLES_PER_PIXEL);
}

