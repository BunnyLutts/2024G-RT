use std::sync::Arc;

use crate::aabb::AABB; 
use crate::hittable::{Hittable, HitRecord};
use crate::utils::{Vec3, Interval}; 
use crate::ray::Ray;
use crate::Material;

pub struct Quad {
    // Points are q, q+u, q+v, q+u+v.
    q: Vec3,
    u: Vec3,
    v: Vec3,
    mat: Arc<dyn Material + Send + Sync>,
    normal: Vec3,
    // q dot normal = d
    d: f64,
    bbox: AABB,
    // w = n / |n|^2
    w: Vec3,
}

impl Quad {
    fn get_bounding_box(q: Vec3, u: Vec3, v: Vec3) -> AABB {
        let (bbox1, bbox2) = (
            AABB::by_two_points(q, q+u+v),
            AABB::by_two_points(q+u, q+v),
        );
        bbox1.combine(&bbox2)
    }
    
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material + Send + Sync>) -> Self {
        let n = u.cross(&v);
        let normal = n.normalize();
        let d = q.dot(&normal);
        let w = n / n.squared_length();
        Self {
            q,
            u,
            v,
            mat,
            bbox: Self::get_bounding_box(q, u, v),
            normal,
            d,
            w,
        }
    }

    pub fn is_interior(&self, alpha: f64, beta: f64) -> bool {
        0.0 <= alpha && alpha <= 1.0 && 0.0 <= beta && beta <= 1.0
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &crate::ray::Ray, r_t: &crate::utils::Interval) -> Option<HitRecord> {
        const EPSILON: f64 = 1e-8;
        let denom = self.normal.dot(&r.dir);

        if denom.abs() < EPSILON {
            return None;
        }

        let t = (self.d - self.normal.dot(&r.ori)) / denom;
        if !r_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let p = intersection - self.q;
        let alpha = self.w.dot(&(p.cross(&self.v)));
        let beta = self.w.dot(&(self.u.cross(&p)));

        if !self.is_interior(alpha, beta) {
            return None;
        }

        let rec = HitRecord::new(intersection, t, self.normal, &r, self.mat.clone(), alpha, beta);
        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}