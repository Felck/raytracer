use super::camera::Camera;
use super::ray::Hittable;
use super::vec3::*;

pub struct Config {
    pub title: &'static str,
    pub width: usize,
    pub height: usize,
    pub camera: Camera,
    pub world: Vec<Box<dyn Hittable>>,
}

impl Config {
    pub fn new(
        title: &'static str,
        width: usize,
        height: usize,
        world: Vec<Box<dyn Hittable>>,
    ) -> Self {
        Self {
            title,
            width,
            height,
            camera: Camera::new(
                width,
                height,
                vec3!(13.0, 2.0, 3.0),
                vec3!(0.0, 0.0, 0.0),
                vec3!(0.0, 1.0, 0.0),
                20.0,
                500,
                50,
            ),
            world,
        }
    }
}
