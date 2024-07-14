mod aabb;
mod color;
mod hittable;
mod material;
mod quad;
mod ray;
mod sphere;
mod medium;
mod texture;
mod utils;

use hittable::{RotationY, Translation};
use medium::ConstantMedium;
use quad::makebox;

use crate::hittable::{BVHNode, HittableList};
use crate::material::*;
use crate::quad::Quad;
use crate::ray::{Camera, CameraConfig};
use crate::sphere::Sphere;
use crate::texture::*;
use crate::utils::{rand01, v3, Vec3};
use std::fs::File;
use std::sync::Arc;

const AUTHOR: &str = "张仁浩";

fn bouncing_spheres() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::create(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new(checker.clone()));
    world.add(Arc::new(Sphere::stable_new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand01();
            let center = Vec3::new(a as f64 + 0.9 * rand01(), 0.2, b as f64 + 0.9 * rand01());

            if (center - Vec3::new(4.0, 0.2, 0.0)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vec3::random_xyz01() * Vec3::random_xyz01();
                    let sphere_material = Arc::new(Lambertian::from(albedo));
                    let velocity = Vec3::new(0.0, rand01() / 2.0, 0.0);
                    world.add(Arc::new(Sphere::new(
                        center,
                        velocity,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Vec3::random_in(0.5, 1.0);
                    let fuzz = rand01() / 2.0;
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::stable_new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::stable_new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::stable_new(
        Vec3::new(0.0, 1.0, 0.0),
        0.5,
        material1.clone(),
    )));

    let material2 = Arc::new(Lambertian::from(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::stable_new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));

    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::stable_new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));

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
        background: v3(0.7, 0.8, 1.0),
    });

    camera.render(&world)
}

fn checkered_spheres() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::create(
        0.32,
        Vec3::new(0.2, 0.3, 0.1),
        Vec3::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::stable_new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::stable_new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));

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
        background: v3(0.7, 0.8, 1.0),
    });

    cam.render(&world)
}

fn cowball() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let texture = Arc::new(ImageTexture::load("image/earthmap.jpeg").unwrap());
    let sufrace = Arc::new(Lambertian::new(texture.clone()));
    let ball = Arc::new(Sphere::stable_new(
        Vec3::new(0.0, 0.0, 0.0),
        2.0,
        sufrace.clone(),
    ));

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
        background: v3(0.7, 0.8, 1.0),
    });

    let mut world = HittableList::new();
    world.add(ball.clone());
    cam.render(&world)
}

fn noise_sphere() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::stable_new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::stable_new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
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
        background: v3(0.7, 0.8, 1.0),
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
        background: v3(0.7, 0.8, 1.0),
    });

    cam.render(&world)
}

fn simple_light() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::stable_new(
        v3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::stable_new(
        v3(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    let difflight = Arc::new(DiffuseLight::from(v3(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::stable_new(
        v3(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(3.0, 1.0, -2.0),
        v3(2.0, 0.0, 0.0),
        v3(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    let cam = Camera::new(CameraConfig {
        ratio: 16.0 / 9.0,
        img_width: 400,
        sample_times: 100,
        reflect_depth: 50,
        vfov: 20.0,
        lookfrom: Vec3::new(26.0, 3.0, 6.0),
        lookat: Vec3::new(0.0, 2.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.0,
        background: v3(0.0, 0.0, 0.0),
    });

    cam.render(&world)
}

fn cornell_box() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from(v3(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(v3(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(v3(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(v3(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        v3(555.0, 0.0, 0.0),
        v3(0.0, 555.0, 0.0),
        v3(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(0.0, 0.0, 0.0),
        v3(0.0, 555.0, 0.0),
        v3(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(343.0, 554.0, 332.0),
        v3(-130.0, 0.0, 0.0),
        v3(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(0.0, 0.0, 0.0),
        v3(555.0, 0.0, 0.0),
        v3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(555.0, 555.0, 555.0),
        v3(-555.0, 0.0, 0.0),
        v3(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(0.0, 0.0, 555.0),
        v3(555.0, 0.0, 0.0),
        v3(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = makebox(&v3(0.0, 0.0, 0.0), &v3(165.0, 330.0, 165.0), white.clone());
    world.add(Arc::new(Translation::new(
        Arc::new(RotationY::new(box1.clone(), 15.0)),
        v3(265.0, 0.0, 295.0),
    )));

    let box2 = makebox(&v3(0.0, 0.0, 0.0), &v3(165.0, 165.0, 165.0), white.clone());
    world.add(Arc::new(Translation::new(
        Arc::new(RotationY::new(box2.clone(), -18.0)),
        v3(130.0, 0.0, 65.0),
    )));

    let cam = Camera::new(CameraConfig {
        ratio: 1.0,
        img_width: 600,
        sample_times: 200,
        reflect_depth: 50,
        background: v3(0.0, 0.0, 0.0),
        vfov: 40.0,
        lookfrom: Vec3::new(278.0, 278.0, -800.0),
        lookat: Vec3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.0,
    });

    cam.render(&world)
}

fn cornell_smoke() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from(v3(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from(v3(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from(v3(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from(v3(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        v3(555.0, 0.0, 0.0),
        v3(0.0, 555.0, 0.0),
        v3(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(0.0, 0.0, 0.0),
        v3(0.0, 555.0, 0.0),
        v3(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(113.0, 554.0, 127.0),
        v3(330.0, 0.0, 0.0),
        v3(0.0, 0.0, 305.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(0.0, 0.0, 0.0),
        v3(555.0, 0.0, 0.0),
        v3(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(555.0, 555.0, 555.0),
        v3(-555.0, 0.0, 0.0),
        v3(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        v3(0.0, 0.0, 555.0),
        v3(555.0, 0.0, 0.0),
        v3(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = makebox(&v3(0.0, 0.0, 0.0), &v3(165.0, 330.0, 165.0), white.clone());
    let box1 = Arc::new(Translation::new(
        Arc::new(RotationY::new(box1.clone(), 15.0)),
        v3(265.0, 0.0, 295.0),
    ));

    let box2 = makebox(&v3(0.0, 0.0, 0.0), &v3(165.0, 165.0, 165.0), white.clone());
    let box2 = Arc::new(Translation::new(
        Arc::new(RotationY::new(box2.clone(), -18.0)),
        v3(130.0, 0.0, 65.0),
    ));

    world.add(Arc::new(ConstantMedium::create(box1, 0.01, v3(0.0, 0.0, 0.0))));
    world.add(Arc::new(ConstantMedium::create(box2, 0.01, v3(1.0, 1.0, 1.0))));

    let cam = Camera::new(CameraConfig {
        ratio: 1.0,
        img_width: 600,
        sample_times: 200,
        reflect_depth: 50,
        background: v3(0.0, 0.0, 0.0),
        vfov: 40.0,
        lookfrom: Vec3::new(278.0, 278.0, -800.0),
        lookat: Vec3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        focus_dist: 10.0,
        defocus_angle: 0.0,
    });

    cam.render(&world)
}

fn main() {
    const SCENE: i32 = 8;

    let path = "output/test.jpg";
    let quality = 60;

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(match SCENE {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => cowball(),
        4 => noise_sphere(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        _ => bouncing_spheres(),
    });
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
