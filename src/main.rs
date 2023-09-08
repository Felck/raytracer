mod raytracer;

use std::sync::Arc;

use raytracer::color::Color;
use raytracer::config::Config;
use raytracer::materials::*;
use raytracer::sphere::Sphere;
use raytracer::vec3::*;

pub fn main() -> Result<(), String> {
    let mat_ground = Arc::new(Material::Lambertian(Lambertian::new(Color::new(
        0.8, 0.8, 0.0,
    ))));
    let diffuse = Arc::new(Material::Lambertian(Lambertian::new(Color::new(
        0.1, 0.2, 0.5,
    ))));
    let metal = Arc::new(Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 0.02)));
    let glass = Arc::new(Material::Glass(Glass::new(1.5)));

    let config = Config::new(
        "Raytracer",
        800,
        600,
        vec![
            Box::new(Sphere::new(
                vec3!(0.0, -100.5, -1.5),
                100.0,
                mat_ground.clone(),
            )),
            Box::new(Sphere::new(vec3!(0.0, 0.0, -1.5), 0.5, diffuse.clone())),
            Box::new(Sphere::new(vec3!(-1.0, 0.0, -1.5), 0.5, glass.clone())),
            Box::new(Sphere::new(vec3!(-1.0, 0.0, -1.5), -0.4, glass.clone())),
            Box::new(Sphere::new(vec3!(1.0, 0.0, -1.5), 0.5, metal.clone())),
        ],
    );
    raytracer::run(&config)
}
