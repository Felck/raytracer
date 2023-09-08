use rand::rngs::ThreadRng;
use rand::Rng;

use super::color::*;
use super::interval::Interval;
use super::materials::Scatterable;
use super::ray::*;
use super::vec3::*;

pub struct Camera {
    pub img_width: usize,
    pub img_height: usize,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pixel00: Vec3,
    du: Vec3,
    dv: Vec3,
    sample_size: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new(
        img_width: usize,
        img_height: usize,
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        sample_size: u32,
        max_depth: u32,
    ) -> Self {
        let focal_length = (look_from - look_at).length();
        let h = (vfov.to_radians() / 2.0).tan();
        let vp_height = 2.0 * h * focal_length;
        let vp_width = vp_height * (img_width as f64 / img_height as f64);

        let w = (look_from - look_at).unit_vec();
        let u = vup.cross(&w).unit_vec();
        let v = w.cross(&u);

        let vp_u = vp_width * u;
        let vp_v = vp_height * -v;

        let du = vp_u / img_width as f64;
        let dv = vp_v / img_height as f64;

        let viewport_upper_left = look_from - (focal_length * w) - vp_u / 2.0 - vp_v / 2.0;
        let pixel00 = viewport_upper_left + 0.5 * (du + dv);

        Self {
            img_width,
            img_height,
            origin: look_from,
            u,
            v,
            w,
            pixel00,
            du,
            dv,
            sample_size,
            max_depth,
        }
    }

    pub fn ray(&self, x: usize, y: usize, rng: &mut ThreadRng) -> Ray {
        let pixel_center = self.pixel00 + self.dv * (y as f64) + self.du * (x as f64);
        let sample = pixel_center + self.pixel_sample_square(rng);
        let direction = sample - self.origin;
        Ray::new(self.origin, direction)
    }

    pub fn pixel_sample_square(&self, rng: &mut ThreadRng) -> Vec3 {
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        return (px * self.du) + (py * self.dv);
    }

    pub fn render(
        &self,
        y: usize,
        band: &mut [u8],
        world: &Vec<Box<dyn Hittable>>,
        rng: &mut ThreadRng,
    ) {
        for x in 0..self.img_width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..self.sample_size {
                let ray = self.ray(x, y, rng);
                color += Self::ray_color(&ray, world, self.max_depth, rng);
            }
            let c = (color / self.sample_size).linear_to_gamma();
            Self::write_pixel(band, x, c);
        }
    }

    fn ray_color(
        ray: &Ray,
        world: &Vec<Box<dyn Hittable>>,
        depth: u32,
        rng: &mut ThreadRng,
    ) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let hit = world.hit(ray, Interval::new(0.001, f64::INFINITY));
        match hit {
            Some(rec) => {
                let scattered = rec.material.scatter(ray, &rec, rng);
                match scattered {
                    Some((sc_ray, albedo)) => {
                        albedo * Self::ray_color(&sc_ray, world, depth - 1, rng)
                    }
                    None => return Color::new(0.0, 0.0, 0.0),
                }
            }
            None => {
                // sky
                let a = (ray.direction.unit_vec().y + 1.0) * 0.5;
                return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
            }
        }
    }

    fn write_pixel(band: &mut [u8], x: usize, color: Color) {
        let p = x * 3;
        band[p] = (256.0 * color.r) as u8;
        band[p + 1] = (256.0 * color.g) as u8;
        band[p + 2] = (256.0 * color.b) as u8;
    }
}
