use crate::color::write_color;
use crate::utils::{self, v3};
use crate::utils::{rand01, Interval, Vec3};
use crate::hittable::Hittable;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use rayon::prelude::*;

pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3,
    pub cnt: usize,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time:f64, cnt: usize) -> Self {
        Self {
            ori: origin,
            dir: direction,
            cnt,
            time,
        }
    }

    pub fn update(&self, origin: Vec3, direction: Vec3) -> Self {
        Self {
            ori: origin,
            dir: direction,
            cnt: self.cnt - 1,
            time: self.time,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + self.dir * t
    }

    pub fn color<T: Hittable>(&self, world: &T, background: &Vec3) -> Vec3 {
        if self.cnt == 0 {
            return Vec3::zero();
        }
        let t = Interval::new(0.001, f64::INFINITY);
        match world.hit(&self, &t) {
            Some(rec) => {
                rec.mat.emitted(rec.u, rec.v, &rec.p) + match rec.mat.scatter(self, &rec) {
                    Some(scatter_rec) => scatter_rec.attenuation * scatter_rec.scattered.color(world, background),
                    None => Vec3::zero(),
                }
            }
            None => background.clone(),
        }
    }
}


pub struct CameraConfig {
    pub img_width: usize,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64, // This argument is in degrees
    pub ratio: f64,
    pub sample_times: usize,
    pub reflect_depth: usize,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Vec3,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            img_width: 100,
            ratio: 1.0,
            sample_times: 10,
            reflect_depth: 10,
            vfov: 90.0,
            lookfrom: Vec3::new(0.0, 0.0, 0.0),
            lookat: Vec3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            focus_dist: 10.0,
            defocus_angle: 0.0,
            background: v3(1.0, 1.0, 1.0),
        }
    }
}

pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    pub aspect_ratio: f64,
    pub view_height: f64,
    pub view_width: f64,
    pub view_u: Vec3,
    pub view_v: Vec3,
    pub view_upper_left: Vec3,
    pub sample_times: usize,
    pub reflect_depth: usize,
    pub vfov: f64, // This argument is in degrees
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub background: Vec3,

    camera_center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    sample_scale: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(cfg: CameraConfig) -> Self {
        let image_width = cfg.img_width;
        let image_height = ((cfg.img_width as f64 / cfg.ratio) as usize).max(1);

        let theta = cfg.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let camera_center = cfg.lookfrom;
        // let focal_length =  (cfg.lookat - cfg.lookfrom).norm();
        let view_height = 2.0 * h * cfg.focus_dist;
        let view_width = view_height * (image_width as f64 / image_height as f64);

        let w = (cfg.lookfrom - cfg.lookat).normalize();
        let u = cfg.vup.cross(&w).normalize();
        let v = w.cross(&u);

        let (view_u, view_v) = (view_width * u, view_height * -v);

        let (pixel_delta_u, pixel_delta_v) = (
            view_u / (image_width as f64),
            view_v / (image_height as f64),
        );

        let view_upper_left = camera_center - cfg.focus_dist * w - view_u / 2.0 - view_v / 2.0;
        let pixel00_loc = view_upper_left + pixel_delta_u + pixel_delta_v;
        let sample_scale = 1.0 / (cfg.sample_times as f64);

        let defocus_radius = cfg.focus_dist * (cfg.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            image_width,
            image_height,
            aspect_ratio: cfg.ratio,
            view_height,
            view_width,
            camera_center,
            view_u,
            view_v,
            pixel_delta_u,
            pixel_delta_v,
            view_upper_left,
            pixel00_loc,
            sample_times: cfg.sample_times,
            reflect_depth: cfg.reflect_depth,
            vfov: cfg.vfov,
            sample_scale,
            lookfrom: cfg.lookfrom,
            lookat: cfg.lookat,
            vup: cfg.vup,
            u,
            v,
            w,
            defocus_angle: cfg.defocus_angle,
            focus_dist: cfg.focus_dist,
            defocus_disk_u,
            defocus_disk_v,
            background: cfg.background,
        }
    }

    fn sample_square() -> Vec3 {
        Vec3::new(rand01() - 0.5, rand01() - 0.5, 0.0)
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((u + offset.x) * self.pixel_delta_u)
            + ((v + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.defocus_disk_sample()
        };
        let ray_time = rand01();
        Ray::new(ray_origin, pixel_sample - ray_origin, ray_time, self.reflect_depth)
    }

    pub fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.camera_center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    pub fn render<T: Hittable + Send + Sync>(&self, world: &T) -> RgbImage {
        let bar: ProgressBar = if utils::is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let mut img: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let color: Vec3 = (0..self.sample_times)
                    .into_par_iter()
                    .map(|_| self.get_ray(i as f64, j as f64).color(world, &self.background))
                    .sum::<Vec3>()
                    * self.sample_scale;
                write_color(color.rgb(), &mut img, i, j);
                bar.inc(1);
            }
        }
        bar.finish();
        img
    }
}
