use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::utils::{rand01, Vec3};

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
            ray_in.update(hit_record.p, scatter_dir),
        ))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self { albedo, fuzz, }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRec> {
        let reflected = ray_in.dir.reflect(&hit_record.normal).normalize() + self.fuzz * Vec3::random_unit();
        if reflected.dot(&hit_record.normal) > 0.0 {
            Some(ScatterRec::new(
                self.albedo,
                ray_in.update(hit_record.p, reflected),
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self {
            refraction_index: ri,
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0*r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<crate::ScatterRec> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if hit_record.face_out {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_dir = ray_in.dir.normalize();

        let cos_theta = (-unit_dir.dot(&hit_record.normal)).min(1.0);
        let sin_theta = (1.0-cos_theta*cos_theta).sqrt();

        let direction = if ri*sin_theta > 1.0 || Self::reflectance(cos_theta, ri) > rand01() {
            unit_dir.reflect(&hit_record.normal)
        } else {
            unit_dir.refract(&hit_record.normal, ri)
        };
        Some(ScatterRec::new(
            attenuation,
            ray_in.update(hit_record.p, direction),
        ))
    }
}