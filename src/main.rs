use crate::render::render;
use std::rc::Rc;

use camera::Camera;
use color::Color;
use hit::HittableList;
use object::{Dielectric, Lambertian, Metal, Sphere};
use util::{random_double, random_double_range};
use vec3::Vec3;

pub mod camera;
pub mod color;
pub mod hit;
pub mod material;
pub mod object;
pub mod ray;
pub mod render;
pub mod util;
pub mod vec3;

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian {
        albedo: Color {
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
    });
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Vec3 {
                x: a as f64 + 0.9 * random_double(),
                y: 0.2,
                z: b as f64 + 0.9 * random_double(),
            };

            if (&center
                - &Vec3 {
                    x: 4.0,
                    y: 0.2,
                    z: 0.0,
                })
                .length()
                > 0.9
            {
                let sphere_material: Rc<dyn material::Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = &Vec3::random() * &Vec3::random();
                    sphere_material = Rc::new(Lambertian { albedo });
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal { albedo, fuzz });
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric { ir: 1.5 });
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material,
                    }));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric { ir: 1.5 });
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material1,
    }));

    let material2 = Rc::new(Lambertian {
        albedo: Color {
            x: 0.4,
            y: 0.2,
            z: 0.1,
        },
    });
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material2,
    }));

    let material3 = Rc::new(Metal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });
    world.add(Box::new(Sphere {
        center: Vec3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material3,
    }));

    world
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let img_width = 1200;
    let img_height = (img_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
    let gamma = 2.0;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Vec3 {
        x: 13.0,
        y: 2.0,
        z: 3.0,
    };
    let lookat = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vup = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    render(
        img_width,
        img_height,
        camera,
        world,
        gamma,
        samples_per_pixel,
    );
}
