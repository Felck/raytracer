mod raytracer;
use raytracer::config::Config;
use raytracer::sphere::Sphere;
use raytracer::vec3::*;

pub fn main() -> Result<(), String> {
    let config = Config::new(
        "Raytracer",
        800,
        600,
        vec![
            Box::new(Sphere::new(vec3!(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(vec3!(0.0, -100.5, -0.5), 100.0)),
        ],
    );
    raytracer::run(&config)
}
