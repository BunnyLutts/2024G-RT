use crate::ray::{HitRecord, Hittable, Ray};
use crate::utils::{Interval, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
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
            ))
        }
    }
}
