mod color;
mod utils;
mod ray;
mod aabb;
mod hittable;
mod material;
mod texture;
mod sphere;
mod quad;

use crate::hittable::{HittableList, BVHNode};
use crate::utils::{Vec3, rand01, v3};
use crate::material::*;
use crate::texture::*;
use crate::ray::{Camera, CameraConfig};
use crate::sphere::Sphere;
use crate::quad::Quad;
use std::fs::File;
use std::sync::Arc;

const AUTHOR: &str = "张仁浩";

fn bouncing_spheres() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::create(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));
    let ground_material = Arc::new(Lambertian::new(checker.clone()));
    world.add(Arc::new(Sphere::stable_new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand01();
            let center = Vec3::new(a as f64 + 0.9 * rand01(), 0.2, b as f64 + 0.9 * rand01());

            if (center - Vec3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_xyz01() * Vec3::random_xyz01();
                    let sphere_material = Arc::new(Lambertian::from(albedo));
                    let velocity = Vec3::new(0.0, rand01()/2.0, 0.0);
                    world.add(Arc::new(Sphere::new(center, velocity, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_in(0.5, 1.0);
                    let fuzz  = rand01()/2.0;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::stable_new(center, 0.2, sphere_material.clone())));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::stable_new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::stable_new(Vec3::new(0.0, 1.0, 0.0), 0.5, material1.clone())));

    let material2 = Arc::new(Lambertian::from(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::stable_new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2.clone())));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::stable_new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3.clone())));

    let world = BVHNode::from(world);

    let camera = Camera::new(CameraConfig {
        ratio: 16.0 / 9.0,
        img_width: 400,
        sample_times: 100,
        reflect_depth: 50,
        vfov: 20.0,
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.6,
    });

    camera.render(&world)
}

fn checkered_spheres() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::create(0.32, Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9)));

    world.add(Arc::new(Sphere::stable_new(Vec3::new(0.0, -10.0, 0.0), 10.0, Arc::new(Lambertian::new(checker.clone())))));
    world.add(Arc::new(Sphere::stable_new(Vec3::new(0.0, 10.0, 0.0), 10.0, Arc::new(Lambertian::new(checker.clone())))));

    let cam = Camera::new(CameraConfig {
        ratio: 16.0 / 9.0,
        img_width: 400,
        sample_times: 100,
        reflect_depth: 50,
        vfov: 20.0,
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.0,
    });

    cam.render(&world)
}

fn cowball() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let texture = Arc::new(ImageTexture::load("image/earthmap.jpeg").unwrap());
    let sufrace = Arc::new(Lambertian::new(texture.clone()));
    let ball = Arc::new(Sphere::stable_new(Vec3::new(0.0, 0.0, 0.0), 2.0, sufrace.clone()));

    let cam = Camera::new(CameraConfig {
        ratio: 16.0 / 9.0,
        img_width: 400,
        sample_times: 100,
        reflect_depth: 50,
        vfov: 20.0,
        lookfrom: Vec3::new(0.0, 0.0, 12.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.0,
    });

    let mut world = HittableList::new();
    world.add(ball.clone());
    cam.render(&world)
}

fn noise_sphere() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::stable_new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(pertext.clone())))));
    world.add(Arc::new(Sphere::stable_new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::new(pertext.clone())))));
    let cam = Camera::new(CameraConfig {
        ratio: 16.0 / 9.0,
        img_width: 400,
        sample_times: 100,
        reflect_depth: 50,
        vfov: 20.0,
        lookfrom: Vec3::new(13.0, 2.0, 3.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.0,
    });
    cam.render(&world)
}

fn quads() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::from(v3(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from(v3(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from(v3(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from(v3(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from(v3(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        v3(-3.0, -2.0, 5.0),
        v3(0.0, 0.0, -4.0),
        v3(0.0, 4.0, 0.0),
        left_red.clone(),
    )));

    world.add(Arc::new(Quad::new(
        v3(-2.0, -2.0, 0.0),
        v3(4.0, 0.0, 0.0),
        v3(0.0, 4.0, 0.0),
        back_green.clone(),
    )));

    world.add(Arc::new(Quad::new(
        v3(3.0, -2.0, 1.0),
        v3(0.0, 0.0, 4.0),
        v3(0.0, 4.0, 0.0),
        right_blue.clone(),
    )));

    world.add(Arc::new(Quad::new(
        v3(-2.0, 3.0, 1.0),
        v3(4.0, 0.0, 0.0),
        v3(0.0, 0.0, 4.0),
        upper_orange.clone(),
    )));

    world.add(Arc::new(Quad::new(
        v3(-2.0, -3.0, 5.0),
        v3(4.0, 0.0, 0.0),
        v3(0.0, 0.0, -4.0),
        lower_teal.clone(),
    )));

    let r = ray::Ray::new(v3(0.0, 0.0, 9.0), v3(0.0, -1.0, -2.0), 0.0, 100);
    let co = r.color(&world);
    println!("{}, {}, {}", co.x, co.y, co.z);

    let cam = Camera::new(CameraConfig {
        ratio: 1.0,
        img_width: 400,
        sample_times: 100,
        reflect_depth: 100,
        vfov: 80.0,
        lookfrom: Vec3::new(0.0, 0.0, 9.0),
        lookat: Vec3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.0,
    });

    cam.render(&world)
}

fn main() {
    const SCENE: i32 = 5;

    let path = "output/test.jpg";
    let quality = 60;

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(
        match SCENE {
            1 => bouncing_spheres(),
            2 => checkered_spheres(),
            3 => cowball(),
            4 => noise_sphere(),
            5 => quads(),
            _ => bouncing_spheres(),
        }
    );
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
