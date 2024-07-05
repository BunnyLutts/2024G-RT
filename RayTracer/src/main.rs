mod color;
mod utils;
mod ray;
mod sphere;
mod camera;

use crate::ray::{HittableList, Ray};
use crate::sphere::Sphere;
use crate::utils::Vec3;
use ray::Camera;
use std::fs::File;
use std::sync::Arc;

const AUTHOR: &str = "张仁浩";

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> bool {
    let oc = *center - r.ori;
    let q = *center - r.dir;
    let a = oc.dot(&oc);
    let b = -2.0 * oc.dot(&q);
    let c = q.dot(&q) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn main() {
    let path = "output/test.jpg";
    let quality = 60;

    let camera = Camera::new(
        400, 
        2.0,
        16.0 / 9.0,
        1.0,
        Vec3::new(0.0, 0.0, 0.0),
        100,
    );
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));
    let img = camera.render(&world);

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
