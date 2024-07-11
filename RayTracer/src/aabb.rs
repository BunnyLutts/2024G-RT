use crate::utils::{Vec3, Interval};
use crate::ray::Ray;

#[derive(Debug, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn by_two_points(p0: Vec3, p1: Vec3) -> Self {
        let x = Interval::new(p0.x.min(p1.x), p0.x.max(p1.x));
        let y = Interval::new(p0.y.min(p1.y), p0.y.max(p1.y));
        let z = Interval::new(p0.z.min(p1.z), p0.z.max(p1.z));
        Self { x, y, z, }
    }

    pub fn combine(&self, other: &AABB) -> Self {
        Self {
            x: self.x.combine(&other.x),
            y: self.y.combine(&other.y),
            z: self.z.combine(&other.z),
        }
    }

    fn hit_interval(ori: f64, dir: f64, r_t: &Interval, tar: &Interval) -> Interval {
        let adinv = 1.0 / dir;
        let t0 = (tar.min - ori) * adinv;
        let t1 = (tar.max - ori) * adinv;
        Interval::new(t0.min(t1), t0.max(t1)).intersect(r_t)
    }

    pub fn hit(&self, ray: &Ray, r_t: &Interval) -> bool {
        let (t_x, t_y, t_z) = (
            Self::hit_interval(ray.ori.x, ray.dir.x, r_t, &self.x),
            Self::hit_interval(ray.ori.y, ray.dir.y, r_t, &self.y),
            Self::hit_interval(ray.ori.z, ray.dir.z, r_t, &self.z),
        );
        // println!("t_x: {:?}, t_y: {:?}, t_z: {:?}", t_x, t_y, t_z);
        !t_x.intersect(&t_y).intersect(&t_z).is_empty()
    }
}

impl Default for AABB {
    fn default() -> Self {
        Self {
            x: Interval::new(f64::INFINITY, f64::NEG_INFINITY),
            y: Interval::new(f64::INFINITY, f64::NEG_INFINITY),
            z: Interval::new(f64::INFINITY, f64::NEG_INFINITY),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{sphere::Sphere, utils::Interval};
    use crate::ray::Ray;
    use crate::Lambertian;
    use crate::utils::Vec3;
    use std::sync::Arc;
    use crate::hittable::{Hittable, HitRecord};

    use super::AABB;

    #[test]
    fn test_aabb_hit() {
        let aabb = AABB::by_two_points(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));
        let inft = Interval::new(0.0, f64::INFINITY);
        let o = Vec3::new(0.0, 0.0, -5.0);

        let ray = Ray::new(o, Vec3::new(0.0, 0.0, 1.0), 0.0, 1);
        assert!(aabb.hit(&ray, &inft));
    }

    #[test]
    fn test_aabb_sphere_hit() {
        let sphere = Sphere::stable_new(Vec3::new(0.0, 0.0, 0.0), 1.0, Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))));
        let aabb = sphere.bounding_box();
        let o = Vec3::new(0.0, 0.0, -5.0);
        let inft = Interval::new(0.0, f64::INFINITY);
        for _ in 0..100 {
            let ray = Ray::new(o, Vec3::random_in_unit_sphere(), 0.0, 1);
            if sphere.hit(&ray, &inft).is_some() {
                assert!(aabb.hit(&ray, &inft));
            }
        }
    }

    #[test]
    fn test_aabb_sphere_hit_para() {
        let sphere = Sphere::stable_new(Vec3::new(0.0, 0.0, 0.0), 1.0, Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))));
        let aabb = sphere.bounding_box();
        let o = Vec3::new(0.0, 0.0, -5.0);
        let inft = Interval::new(0.0, f64::INFINITY);

        let ray = Ray::new(o, Vec3::new(1.0, 0.0, 0.0), 0.0, 1);
        assert!(!aabb.hit(&ray, &inft));

        let ray = Ray::new(o, Vec3::new(0.0, 1.0, 0.0), 0.0, 1);
        assert!(!aabb.hit(&ray, &inft));

        let ray = Ray::new(o, Vec3::new(0.0, 0.0, 1.0), 0.0, 1);
        assert!(aabb.hit(&ray, &inft));
    }
}