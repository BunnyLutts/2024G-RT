use std::cmp::Ordering;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::{v3, Interval, Vec3};

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub face_out: bool,
    pub mat: Arc<dyn Material + Sync + Send>,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        t: f64,
        out_normal: Vec3,
        r: &Ray,
        mat: Arc<dyn Material + Sync + Send>,
        u: f64,
        v: f64,
    ) -> Self {
        let face_out = r.dir.dot(&out_normal) < 0.0;
        let normal = if face_out { out_normal } else { -out_normal };
        Self {
            p,
            normal,
            t,
            face_out,
            mat,
            u,
            v,
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
    fn box_x_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
        a.bounding_box().x.min.total_cmp(&b.bounding_box().x.min)
    }
    fn box_y_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
        a.bounding_box().y.min.total_cmp(&b.bounding_box().y.min)
    }
    fn box_z_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
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
            Self { bbox, left, right }
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

pub struct Translation {
    object: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3,
    bbox: AABB,
}

impl Translation {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translation {
    fn hit(&self, r: &Ray, r_t: &Interval) -> Option<HitRecord> {
        let offset_r = Ray::new(r.ori - self.offset, r.dir, r.time, r.cnt);

        match self.object.hit(&offset_r, r_t) {
            Some(rec) => Some(HitRecord {
                p: rec.p + self.offset,
                ..rec
            }),
            None => None,
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }
}

pub struct RotationY {
    object: Arc<dyn Hittable + Send + Sync>,
    cos_theta: f64,
    sin_theta: f64,
    bbox: AABB,
}

impl RotationY {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {
        let rad = angle.to_radians();
        let sin_theta = rad.sin();
        let cos_theta = rad.cos();
        let bbox = object.bounding_box();

        let mut min = v3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = v3(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (x, y, z) = (
                        i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min,
                        j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min,
                        k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min,
                    );

                    let nx = x * cos_theta + z * sin_theta;
                    let nz = -sin_theta * x + z * cos_theta;

                    min.x = min.x.min(nx);
                    min.y = min.y.min(y);
                    min.z = min.z.min(nz);

                    max.x = max.x.max(nx);
                    max.y = max.y.max(y);
                    max.z = max.z.max(nz);
                }
            }
        }

        Self {
            bbox: AABB::by_two_points(min, max),
            object,
            cos_theta,
            sin_theta,
        }
    }
}

impl Hittable for RotationY {
    fn bounding_box(&self) -> AABB {
        self.bbox.clone()
    }

    fn hit(&self, r: &Ray, r_t: &Interval) -> Option<HitRecord> {
        let ori = v3(
            self.cos_theta * r.ori.x - self.sin_theta * r.ori.z,
            r.ori.y,
            self.sin_theta * r.ori.x + self.cos_theta * r.ori.z,
        );

        let dir = v3(
            self.cos_theta * r.dir.x - self.sin_theta * r.dir.z,
            r.dir.y,
            self.sin_theta * r.dir.x + self.cos_theta * r.dir.z,
        );

        let rotated_r = Ray::new(ori, dir, r.time, r.cnt);

        match self.object.hit(&rotated_r, r_t) {
            Some(rec) => Some(HitRecord {
                p: v3(
                    self.cos_theta * rec.p.x + self.sin_theta * rec.p.z,
                    rec.p.y,
                    -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z,
                ),
                normal: v3(
                    self.cos_theta * rec.normal.x - self.sin_theta * rec.normal.z,
                    rec.normal.y,
                    self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
                ),
                ..rec
            }),
            None => None,
        }
    }
}
