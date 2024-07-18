use crate::hittable::*;
use crate::plane::Plane;
use crate::sphere::Sphere;
use crate::texture::*;
use crate::material::*;
use crate::ray::CameraConfig;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

pub struct Script {
    pub cam: CameraConfig,
    pub world: HittableList,

    hittables: HashMap<String, Arc<dyn Hittable>>,
    textures: HashMap<String, Arc<dyn Texture>>,
    materials: HashMap<String, Arc<dyn Material>>,
}

impl Script {
    pub fn new() -> Script {
        Self {
            cam: CameraConfig::default(),
            world: HittableList::new(),
            hittables: HashMap::new(),
            textures: HashMap::new(),
            materials: HashMap::new(),
        }
    }
}

pub fn load_script(filename: &str) -> Result<Script, String> {
    let file = match File::open(filename) {
        Err(why) => return Err(why.to_string()),
        Ok(file) => file,
    };

    let buffer = BufReader::new(file);

    let mut real_line = String::new();
    for i in buffer.lines() {
        match i {
            Err(why) => return Err(why.to_string()),
            Ok(line) => {
                if line.ends_with("\\") {
                    real_line.push_str(&line[..line.len() - 1]);
                } else {
                    real_line.push_str(&line);
                    
                }
            }
        }
    }

    Err(String::from("New"))
}