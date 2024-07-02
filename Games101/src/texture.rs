#![allow(warnings)]
use nalgebra::{Vector3};

use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub struct Texture {
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new(name: &str) -> Self {
        let img_data = imread(name, IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        Texture {
            img_data,
            width,
            height,
        }
    }

    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        if u < 0.0 { u = 0.0; }
        if u > 1.0 { u = 1.0; }
        if v < 0.0 { v = 0.0; }
        if v > 1.0 { v = 1.0; }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();

        Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
    }

    fn mix_u8(a: &VecN<u8, 3>, b: &VecN<u8, 3>, alpha: f64) -> Vector3<f64> {
        Vector3::new(
            b[2] as f64 * alpha + a[2] as f64 * (1.0 - alpha),
            b[1] as f64 * alpha + a[1] as f64 * (1.0 - alpha),
            b[0] as f64 * alpha + a[0] as f64 * (1.0 - alpha),
        )
    }

    fn mix_f64(a: &Vector3<f64>, b: &Vector3<f64>, alpha: f64) -> Vector3<f64> {
        Vector3::new(
            b.x * alpha + a.x * (1.0 - alpha),
            b.y * alpha + a.y * (1.0 - alpha),
            b.z * alpha + a.z * (1.0 - alpha),
        )
    }

    pub fn get_color_bilinear(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        // 在此实现双线性插值函数, 并替换掉get_color
        u = u.max(0.0).min(1.0);
        v = v.max(0.0).min(1.0);
        let x0 = ((u * self.width as f64 - 0.5).floor() as i32).min(self.width as i32 - 1).max(0);
        let x1 = (x0 + 1).min(self.width as i32 - 1).max(0);
        let y0 = (((1.0-v) * self.height as f64 - 0.5).floor() as i32).min(self.height as i32 - 1).max(0);
        let y1 = (y0 + 1).min(self.height as i32 - 1).max(0);
        let color0: &VecN<u8, 3> = self.img_data.at_2d(y0, x0).unwrap();
        let color1: &VecN<u8, 3> = self.img_data.at_2d(y0, x1).unwrap();
        let color2: &VecN<u8, 3> = self.img_data.at_2d(y1, x0).unwrap();
        let color3: &VecN<u8, 3> = self.img_data.at_2d(y1, x1).unwrap();
        let alpha = u*self.width as f64 - x0 as f64 - 0.5;
        let beta = (1.0-v)*self.height as f64 - y0 as f64 - 0.5;
        Self::mix_f64(
            &Self::mix_u8(color0, color1, alpha),
            &Self::mix_u8(color2, color3, alpha),
            beta,
        )
    }
}