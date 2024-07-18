use crate::utils::{Interval, Perlin, Vec3};
use image::{io::Reader, RgbImage};
use std::sync::Arc;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}

pub struct SolidColor {
    albedo: Vec3,
}

impl SolidColor {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
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
    pub fn new(
        inv_scale: f64,
        even: Arc<dyn Texture + Send + Sync>,
        odd: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
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

        match (xgrid + ygrid + zgrid) % 2 {
            0 => self.even.value(u, v, p),
            _ => self.odd.value(u, v, p),
        }
    }
}

pub struct ImageTexture {
    img: RgbImage,
}

impl ImageTexture {
    pub fn new(img: RgbImage) -> Self {
        Self { img }
    }

    pub fn load(filename: &str) -> Result<Self, &str> {
        let img = match Reader::open(filename) {
            Ok(r) => match r.decode() {
                Ok(img) => img,
                Err(_) => return Err("Error decoding image file"),
            },
            Err(_) => return Err("Error opening image file"),
        };

        let img = img.to_rgb8();

        Ok(Self::new(img))
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let unit = Interval::new(0.0, 1.0);
        let u = unit.clamp(u);
        let v = 1.0 - unit.clamp(v);

        let i = (u * (self.img.width() - 1) as f64).floor() as u32;
        let j = (v * (self.img.height() - 1) as f64).floor() as u32;
        let pixel = self.img.get_pixel(i, j);

        const COLOR_SCALE: f64 = 1.0 / 255.0;
        Vec3::new(pixel.0[0] as f64, pixel.0[1] as f64, pixel.0[2] as f64) * COLOR_SCALE
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        // Vec3::ones() * 0.5 * (1.0 + self.noise.noise(&(self.scale * *p)))
        Vec3::new(0.5, 0.5, 0.5)
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p.clone(), 7)).sin())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_image_texture() {
        let img = ImageTexture::load("image/earthmap.jpeg").unwrap();
        for i in 0..img.img.width() {
            for j in 0..img.img.height() {
                let pixel = img.value(
                    i as f64 / (img.img.width() - 1) as f64,
                    j as f64 / (img.img.height() - 1) as f64,
                    &Vec3::new(0.0, 0.0, 0.0),
                );
                println!("{:?}", pixel);
            }
        }
    }
}
