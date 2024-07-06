use crate::ray::{HitRecord, Ray};
use crate::utils::Vec3;

pub struct ScatterRec {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

impl ScatterRec {
    pub fn new(attenuation: Vec3, scattered: Ray) -> Self {
        Self { attenuation, scattered }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRec>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRec> {
        let mut scatter_dir = hit_record.normal + Vec3::random_unit();
        if scatter_dir.near_zero() {
            scatter_dir = hit_record.normal;
        }
        Some(ScatterRec::new(
            self.albedo,
            Ray::new(hit_record.p, scatter_dir, ray_in.cnt - 1),
        ))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRec> {
        let reflected = ray_in.dir.reflect(hit_record.normal);
        Some(ScatterRec::new(
            self.albedo,
            Ray::new(hit_record.p, reflected, ray_in.cnt - 1),
        ))
    }
}