use crate::vec3::Vec3;

pub struct Configuration {
    pub image_width: usize,
    pub image_height: usize,
    pub aspect_ratio: f64,
    pub view_height: f64,
    pub view_width: f64,
    pub focal_length: f64,
    pub camera_center: Vec3,
    pub view_u: Vec3,
    pub view_v: Vec3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub view_upper_left: Vec3,
    pub pixel00_loc: Vec3,
}

impl Configuration {
    pub fn new(
        img_width: usize, view_height: f64, ratio: f64,
        focal_length: f64, camera_center: Vec3
        
    ) -> Self {
        let image_width = img_width;
        let image_height = ((img_width as f64 / ratio) as usize).max(1);
        let view_width = view_height * (image_width as f64 / image_height as f64);
        let (view_u, view_v) = (
            Vec3::new(view_width, 0.0, 0.0),
            Vec3::new(0.0, -view_height, 0.0),
        );
        let (pixel_delta_u, pixel_delta_v) = (
            view_u / (image_width as f64),
            view_v / (image_height as f64),
        );
        let view_upper_left = 
            camera_center - Vec3::new(0.0, 0.0, focal_length) - view_u / 2.0 - view_v / 2.0;
        let pixel00_loc = view_upper_left + pixel_delta_u + pixel_delta_v;
        Self {
            image_width, image_height,
            aspect_ratio: ratio,
            view_height, view_width, focal_length,
            camera_center,
            view_u, view_v, pixel_delta_u, pixel_delta_v,
            view_upper_left, pixel00_loc,
        }
    }
}