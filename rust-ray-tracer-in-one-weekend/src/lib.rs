mod vector; 
mod color;
mod ray; 
mod model;
mod sphere;
mod camera;

use std::rc::Rc;

use color::Color3;
use sphere::Sphere;
use vector::Vec3;
use ray::Ray;
use model::Hittable;

pub mod rt {
    use crate::vector;

    pub const INFINITY: f64 = f64::INFINITY;
    pub const PI: f64 = std::f64::consts::PI;

    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }

    pub fn random_double() -> vector::fsize {
        // Returns a random real in [0, 1)
        rand::random::<vector::fsize>()
    }

    pub fn random_double_between(min: vector::fsize, max: vector::fsize) -> vector::fsize {
        min + (max - min) * random_double()
    }

    pub fn clamp(value: vector::fsize, min: vector::fsize, max: vector::fsize) -> vector::fsize {
        if value < min {
            return min
        }
        if value > max {
            return max
        }
        value
    }
}

pub fn ray_color(ray: &Ray, world: &model::HitList) -> Color3 {
    if let Some(hit) = world.hit(ray, 0.0, rt::INFINITY) {
        return 0.5 * (hit.normal_as_color() + Color3(1.0, 1.0, 1.0))
    }

    // Get unit vector of direction to map to distance of project plane 
    let unit_direction = ray.direction().normalize();

    // Graphics trick to scale time: 0.0 <= time <= 1.0
    let time = 0.5*(unit_direction.y() + 1.0);

    // Colors of gradient
    let white_color = Color3(1.0, 1.0, 1.0);
    let blue_color = Color3(0.5, 0.7, 1.0);

    // Create linear gradient of blue to white (top down)
    time * blue_color + (1.0 - time) * white_color
}

pub fn render_ppm(image_width: u32) {
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 25;

    // Models
    let sphere = Rc::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5));
    let big_sphere = Rc::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0));

    // World
    let mut world = model::HitList::new();
    world.add(sphere);
    world.add(big_sphere);


    // Camera 
    let camera = camera::Camera::new(aspect_ratio);

    // Render 
    // PPM Header 
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");


    // Image 
    for row in (0..image_height).rev() {
        for column in 0..image_width {
            let mut pixel = Color3(0.0, 0.0, 0.0);

            for sample in (0..samples_per_pixel) {
                let horizontal_line = (column as f64 + rt::random_double()) / ((image_width - 1) as f64);
                let vertical_line = (row as f64 + rt::random_double()) / ((image_height - 1) as f64);

                let ray = camera.get_ray(horizontal_line, vertical_line);

                pixel = pixel + ray_color(&ray, &world)
            }

            pixel.average_samples(samples_per_pixel);
            print!("{} ", pixel);
        }
        println!("");
    } 
}

pub fn write_test_ppm(image_width: u32, image_height: u32) {
    // PPM Header 
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    // Image 
    for row in (0..image_height).rev() {
        for column in 0..image_width {
            let color = Color3(
                column as f64 / ((image_width - 1) as f64),
                row as f64 / ((image_height - 1) as f64),
                0.25,
            );

            print!("{} ", color);
        }
        println!("");
    } 
}