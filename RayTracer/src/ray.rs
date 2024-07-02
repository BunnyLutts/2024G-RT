use crate::vec3::Vec3;

pub struct Ray {
    ori: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { ori: origin, dir: direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + self.dir * t
    }

    pub fn color(&self) -> Vec3 {
        let blue = Vec3::new(0.5, 0.7, 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let a = 0.5*(self.dir.normalize().y + 1.0);
        blue * a + white * (1.0 - a)
    }
}