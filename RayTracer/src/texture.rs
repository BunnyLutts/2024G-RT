use std::sync::Arc;
use crate::utils::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo, }
    }

    pub fn from(r: f64, g: f64, b: f64) -> Self {
        Self::new(Vec3::new(r, g, b))
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture + Send + Sync>,
    odd: Arc<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new(inv_scale: f64, even: Arc<dyn Texture + Send + Sync>, odd: Arc<dyn Texture + Send + Sync>) -> Self {
        Self {
            inv_scale,
            even,
            odd,
        }
    }

    pub fn create(scale: f64, even_color: Vec3, odd_color: Vec3) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(even_color)),
            odd: Arc::new(SolidColor::new(odd_color)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let (xgrid, ygrid, zgrid) = (
            (self.inv_scale * p.x).floor() as i64,
            (self.inv_scale * p.y).floor() as i64,
            (self.inv_scale * p.z).floor() as i64,
        );

        match (xgrid+ygrid+zgrid) % 2 {
            0 => self.even.value(u, v, p),
            _ => self.odd.value(u, v, p),
        }
    }
}