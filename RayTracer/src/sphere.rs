use std::sync::Arc;
use crate::ray::{HitRecord, Hittable, Ray};
use crate::utils::{Interval, Vec3};
use crate::material::Material;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    mat: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, mat: Arc<dyn Material + Sync + Send>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, r_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - r.ori;
        let a = r.dir.squared_length();
        let h = r.dir.dot(&oc);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            None
        } else {
            let sqrtd = discriminant.sqrt();
            let mut root = (h - sqrtd) / a;
            if !r_t.surrounds(root) {
                root = (h + sqrtd) / a;
                if !r_t.surrounds(root) {
                    return None;
                }
            }
            Some(HitRecord::new(
                r.at(root),
                root,
                (r.at(root) - self.center) / self.radius,
                r,
                self.mat.clone(),
            ))
        }
    }
}
