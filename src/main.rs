mod raytracer;

use std::sync::Arc;

use rand::Rng;
use raytracer::color::Color;
use raytracer::config::Config;
use raytracer::materials::*;
use raytracer::ray::Hittable;
use raytracer::sphere::Sphere;
use raytracer::vec3::*;

pub fn main() -> Result<(), String> {
    let mut world = Vec::<Box<dyn Hittable>>::new();

    world.push(Box::new(Sphere::new(
        vec3!(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Material::Lambertian(Lambertian::new(Color::new(
            0.5, 0.5, 0.5,
        )))),
    )));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = vec3!(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>()
            );

            if ((center - vec3!(4.0, 0.2, 0.0)).length()) < 0.9 {
                continue;
            }

            if choose_mat < 0.8 {
                // diffuse
                world.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Material::Lambertian(Lambertian::new(Color::new(
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                        rng.gen::<f64>() * rng.gen::<f64>(),
                    )))),
                )));
            } else if choose_mat < 0.95 {
                // metal
                world.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Material::Metal(Metal::new(
                        Color::new(
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                            0.5 * (1.0 + rng.gen::<f64>()),
                        ),
                        0.5 * rng.gen::<f64>(),
                    ))),
                )));
            } else {
                // glass
                world.push(Box::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(Material::Glass(Glass::new(1.5))),
                )));
            }
        }
    }

    world.push(Box::new(Sphere::new(
        vec3!(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Material::Glass(Glass::new(1.5))),
    )));
    world.push(Box::new(Sphere::new(
        vec3!(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Material::Lambertian(Lambertian::new(Color::new(
            0.4 as f64, 0.2 as f64, 0.1 as f64,
        )))),
    )));
    world.push(Box::new(Sphere::new(
        vec3!(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Material::Metal(Metal::new(
            Color::new(0.7 as f64, 0.6 as f64, 0.5 as f64),
            0.0,
        ))),
    )));

    let config = Config::new("Raytracer", 1200, 675, world);
    raytracer::run(&config)
}
