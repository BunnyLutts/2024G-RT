use crate::vec3::Vec3;

pub struct Ray {
    pub ori: Vec3,
    pub dir: Vec3,
}

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.ori;
    let a = r.dir.squared_length();
    let h = r.dir.dot(&oc);
    let c = oc.squared_length() - radius * radius;
    let discriminant = h * h - a * c;
    
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            ori: origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + self.dir * t
    }

    pub fn color(&self) -> Vec3 {
        let cir_cen = Vec3::new(0.0, 0.0, -1.0);
        let t = hit_sphere(&cir_cen, 0.5, self);
        if t>=0.0 {
            let normal = (self.at(t) - cir_cen).normalize();
            return 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
        }

        let blue = Vec3::new(0.5, 0.7, 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let a = 0.5 * (self.dir.normalize().y + 1.0);
        blue * a + white * (1.0 - a)
    }
}
