use super::camera::Camera;
use super::ray::Hittable;
use super::vec3::*;

pub struct Config {
    pub title: &'static str,
    pub width: usize,
    pub height: usize,
    pub camera: Camera,
    pub world: Vec<Box<dyn Hittable + Sync>>,
}

impl Config {
    pub fn new(
        title: &'static str,
        width: usize,
        height: usize,
        world: Vec<Box<dyn Hittable + Sync>>,
    ) -> Self {
        Self {
            title,
            width,
            height,
            camera: Camera::new(width, height, vec3!(0.0, 0.0, 0.0), 8),
            world,
        }
    }
}
