use std::cmp::Ordering;
use std::sync::Arc;
use rand::Rng;

use crate::material::Material;
use crate::utils::{Vec3, Interval};
use crate::aabb::AABB;
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face_out: bool,
    pub mat: Arc<dyn Material+ Sync + Send>,
}

impl HitRecord {
    pub fn new(p: Vec3, t: f64, out_normal: Vec3, r: &Ray, mat: Arc<dyn Material + Sync + Send>) -> Self {
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

    fn bounding_box(&self) -> AABB;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Sync + Send>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable + Sync + Send>) {
        self.bbox = self.bbox.combine(&obj.bounding_box());
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

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

pub struct BVHNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: AABB,
}

impl BVHNode {

    fn box_x_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        a.bounding_box().x.min.total_cmp(&b.bounding_box().x.min)
    }
    fn box_y_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        a.bounding_box().y.min.total_cmp(&b.bounding_box().y.min)
    }
    fn box_z_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
        a.bounding_box().z.min.total_cmp(&b.bounding_box().z.min)
    }

    pub fn new(objects: Vec<Arc<dyn Hittable + Send + Sync>>) -> Self {
        let mut bbox = AABB::default();
        for i in &objects {
            bbox = bbox.combine(&i.bounding_box());
        }
        let axis = bbox.longest_axis();
        let compare_fn = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => unreachable!(),
        };
        if objects.len() == 1 {
            // println!("1: {:?}", objects[0].bounding_box());
            Self {
                left: objects[0].clone(),
                right: objects[0].clone(),
                bbox,
            }
        } else if objects.len() == 2 {
            // println!("2: {:?}", objects[0].bounding_box().combine(&objects[1].bounding_box()));
            Self {
                bbox,
                left: objects[0].clone(),
                right: objects[1].clone(),
            }
        } else {
            let mut objects = objects;
            objects.sort_by(compare_fn);
            let mid = objects.len() / 2;
            let (left_vec, right_vec) = objects.split_at(mid);
            let left = Arc::new(BVHNode::new(left_vec.to_vec()));
            let right = Arc::new(BVHNode::new(right_vec.to_vec()));
            // println!("3: {:?}", left.bbox.combine(&right.bbox));
            Self {
                bbox,
                left, right,
            }
        }
    }

    pub fn from(origin: HittableList) -> Self {
        Self::new(origin.objects)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, r_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, r_t) {
            None
        } else {
            let hit_left = self.left.hit(r, r_t);
            let hit_right = self.right.hit(r, r_t);

            if hit_left.is_none() {
                hit_right
            } else if hit_right.is_none() {
                hit_left
            } else {
                let left = hit_left.unwrap();
                let right = hit_right.unwrap();
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}