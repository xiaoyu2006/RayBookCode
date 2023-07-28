pub mod camera;
pub mod color;
pub mod hit;
pub mod material;
pub mod object;
pub mod ray;
pub mod render;
pub mod util;
pub mod vec3;

use std::rc::Rc;

use camera::Camera;
use color::Color;
use object::{Lambertian, Metal, Sphere};
use ray::Ray;
use render::render;
use vec3::Vec3;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const GAMMA: f64 = 1.8;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let camera = Camera::new(viewport_height, viewport_width, focal_length);

    // Objects
    let mut world = hit::HittableList::new();

    let material_ground = Lambertian {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.0,
        },
    };
    let material_center = Lambertian {
        albedo: Color {
            x: 0.7,
            y: 0.3,
            z: 0.3,
        },
    };
    let material_left = Metal {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
        fuzz: 0.3,
    };
    let material_right = Metal {
        albedo: Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        fuzz: 1.0,
    };

    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -100.5,
            z: -1.0,
        },
        radius: 100.0,
        material: Rc::new(material_ground),
    }));
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Rc::new(material_center),
    }));
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: -1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Rc::new(material_left),
    }));
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 1.0,
            y: 0.0,
            z: -1.0,
        },
        radius: 0.5,
        material: Rc::new(material_right),
    }));

    // Render
    render(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        camera,
        world,
        GAMMA,
        SAMPLES_PER_PIXEL,
    );
}
