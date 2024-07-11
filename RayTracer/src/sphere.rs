use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::utils::{Interval, Vec3};
use std::sync::Arc;

pub struct Sphere {
    p: Vec3,
    v: Vec3,
    radius: f64,
    mat: Arc<dyn Material + Sync + Send>,
    bbox: AABB,
}

impl Sphere {
    pub fn stable_new(center: Vec3, radius: f64, mat: Arc<dyn Material + Sync + Send>) -> Self {
        let p = Vec3::new(radius, radius, radius);
        Self {
            p: center,
            v: Vec3::zero(),
            radius: radius.max(0.0),
            mat,
            bbox: AABB::by_two_points(center - p, center + p),
        }
    }

    pub fn new(p: Vec3, v: Vec3, radius: f64, mat: Arc<dyn Material + Sync + Send>) -> Self {
        let r = Vec3::new(radius, radius, radius);
        let bbox1 = AABB::by_two_points(p - r, p + r);
        let bbox2 = AABB::by_two_points(p+v-r, p+v+r);
        Self {
            p,
            v,
            radius,
            mat,
            bbox: bbox1.combine(&bbox2),
        }
    }

    pub fn center(&self, t: f64) -> Vec3 {
        self.p + t * self.v
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, r_t: &Interval) -> Option<HitRecord> {
        let center = self.center(r.time);
        let oc = center - r.ori;
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
                (r.at(root) - center) / self.radius,
                r,
                self.mat.clone(),
            ))
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}
