use crate::color::write_color;
use crate::material::Material;
use crate::utils::{rand01, Interval, Vec3};
use crate::utils;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::sync::Arc;

pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3,
    pub cnt: usize,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, cnt: usize) -> Self {
        Self {
            ori: origin,
            dir: direction,
            cnt,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + self.dir * t
    }

    pub fn color<T: Hittable>(&self, world: &T) -> Vec3 {
        if self.cnt == 0 {
            return Vec3::zero();
        }
        let t = Interval::new(0.001, std::f64::INFINITY);
        if let Some(rec) = world.hit(&self, &t) {
            return match rec.mat.scatter(self, &rec) {
                Some(scatter_rec) => scatter_rec.attenuation * scatter_rec.scattered.color(world),
                None => Vec3::zero(),
            };
        }

        let blue = Vec3::new(0.5, 0.7, 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let a = 0.5 * (self.dir.normalize().y + 1.0);
        blue * a + white * (1.0 - a)
    }
}

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face_out: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Vec3, t: f64, out_normal: Vec3, r: &Ray, mat: Arc<dyn Material>) -> Self {
        let face_out = r.dir.dot(&out_normal) < 0.0;
        let normal = if face_out { out_normal } else { -out_normal };
        Self {
            p,
            normal,
            t,
            face_out,
            mat,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, r_t: &Interval) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, r_t: &Interval) -> Option<HitRecord> {
        let mut now_t = r_t.clone();
        let mut hit_record = None;

        for obj in &self.objects {
            if let Some(rec) = obj.hit(r, &now_t) {
                now_t.max = rec.t;
                hit_record = Some(rec);
            }
        }

        hit_record
    }
}

pub struct CameraConfig {
    pub img_width: usize,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f64,              // This argument is in degrees
    pub ratio: f64,
    pub sample_times: usize,
    pub reflect_depth: usize,
    pub defocus_angle: f64,
    pub focus_dist: f64,
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
    pub vfov: f64,              // This argument is in degrees
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

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
        let h = (theta/2.0).tan();

        let camera_center = cfg.lookfrom;
        // let focal_length =  (cfg.lookat - cfg.lookfrom).norm();
        let view_height = 2.0 * h * cfg.focus_dist;
        let view_width = view_height * (image_width as f64 / image_height as f64);

        let w = (cfg.lookfrom-cfg.lookat).normalize();
        let u = cfg.vup.cross(&w).normalize();
        let v = w.cross(&u);

        let (view_u, view_v) = (
            view_width * u,
            view_height * -v,
        );

        let (pixel_delta_u, pixel_delta_v) = (
            view_u / (image_width as f64),
            view_v / (image_height as f64),
        );

        let view_upper_left =
            camera_center - cfg.focus_dist * w - view_u / 2.0 - view_v / 2.0;
        let pixel00_loc = view_upper_left + pixel_delta_u + pixel_delta_v;
        let sample_scale = 1.0 / (cfg.sample_times as f64);

        let defocus_radius = cfg.focus_dist * (cfg.defocus_angle/2.0).to_radians().tan();
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
        let ray_origin = if self.defocus_angle <= 0.0 {self.camera_center} else {self.defocus_disk_sample()};
        Ray::new(
            ray_origin,
            pixel_sample - ray_origin,
            self.reflect_depth,
        )
    }

    pub fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.camera_center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }

    pub fn render(&self, world: &HittableList) -> RgbImage {
        let bar: ProgressBar = if utils::is_ci() {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        let mut img: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_times {
                    let ray = self.get_ray(i as f64, j as f64);
                    color += ray.color(world);
                }
                color = color * self.sample_scale;
                write_color(color.rgb(), &mut img, i, j);
                bar.inc(1);
            }
        }
        bar.finish();
        img
    }
}