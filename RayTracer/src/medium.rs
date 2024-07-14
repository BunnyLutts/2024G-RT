use crate::hittable::{HitRecord, Hittable};
use crate::material::{Isotropic, Material};
use crate::utils::{self, rand01, Interval, Vec3};
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Send + Sync>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material + Send + Sync>,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable + Send + Sync>,
        density: f64,
        phase_function: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub fn create(boundary: Arc<dyn Hittable + Send + Sync>, density: f64, albedo: Vec3) -> Self {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::ray::Ray, r_t: &Interval) -> Option<HitRecord> {
        const ENABLE_DEBUG: bool = false;
        let debug = ENABLE_DEBUG && rand01() < 0.00001;

        let rec1 = self.boundary.hit(r, &utils::UNIVERSE);
        if rec1.is_none() {
            return None;
        }
        let mut rec1 = rec1.unwrap();

        let rec2 = self
            .boundary
            .hit(r, &Interval::new(rec1.t + 0.0001, f64::INFINITY));
        if rec2.is_none() {
            return None;
        }
        let mut rec2 = rec2.unwrap();

        rec1.t = rec1.t.max(r_t.min).max(0.0);
        rec2.t = rec2.t.min(r_t.max);
        if rec1.t >= rec2.t {
            return None;
        }

        let ray_length = r.dir.norm();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rand01().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        if debug {
            println!(
                "hit_distance: {}, distance_inside_boundary: {}\nt={t}\np=({}, {}, {})\n",
                hit_distance, distance_inside_boundary, p.x, p.y, p.z
            );
        }

        Some(HitRecord{
            p,
            normal: Vec3::ones(),
            t,
            face_out: true,
            mat: self.phase_function.clone(),
            u: 0.0,
            v: 0.0,
        })
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}
