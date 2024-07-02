mod color;
mod config;
mod ray;
mod vec3;

use color::write_color;
use image::{ImageBuffer, RgbImage}; //接收render传回来的图片，在main中文件输出
use indicatif::ProgressBar;
use std::fs::File;
use crate::config::Configuration;
use crate::vec3::Vec3;
use crate::ray::Ray;

const AUTHOR: &str = "张仁浩";

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn main() {
    let config = Configuration::new(
        400,
        2.0,
        16.0 / 9.0,
        1.0,
        Vec3::new(0.0, 0.0, 0.0),
    );

    let path = "output/test.jpg";
    let quality = 60;
    let bar: ProgressBar = if is_ci() {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((config.image_height * config.image_width) as u64)
    };

    let mut img: RgbImage = ImageBuffer::new(config.image_width as u32, config.image_height as u32);

    // 以下是write color和process bar的示例代码
    // let pixel_color = [255u8; 3];
    for j in 0..config.image_height {
        for i in 0..config.image_width {
            let pixel_center = config.pixel00_loc + config.pixel_delta_u * (i as f64) + config.pixel_delta_v * (j as f64);
            let ray = Ray::new(config.camera_center, pixel_center - config.camera_center);
            let color = ray.color();
            write_color(color.rgb(), &mut img, i, j);
            bar.inc(1);
        }
    }
    bar.finish();

    println!("Ouput image as \"{}\"\n Author: {}", path, AUTHOR);
    let output_image: image::DynamicImage = image::DynamicImage::ImageRgb8(img);
    let mut output_file: File = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("Outputting image fails."),
    }
}
