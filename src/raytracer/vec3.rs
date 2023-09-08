use auto_ops::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[macro_export]
macro_rules! vec3 {
    ( $x:expr, $y:expr, $z:expr ) => {{
        Vec3::new($x, $y, $z)
    }};
}
pub(crate) use vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn unit_vec(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn near_zero(&self) -> bool {
        //self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
        self.x.abs() < 1.0e-7 && self.y.abs() < 1.0e-7 && self.z.abs() < 1.0e-7
    }

    pub fn reflect(&self, normal: &Vec3) -> Self {
        return self - 2.0 * self.dot(normal) * normal;
    }

    pub fn refract(&self, n: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (self + n * cos_theta);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
        r_out_perp + r_out_parallel
    }

    fn random(min: f64, max: f64, rng: &mut ThreadRng) -> Self {
        Vec3::new(
            min + (max - min) * rng.gen::<f64>(),
            min + (max - min) * rng.gen::<f64>(),
            min + (max - min) * rng.gen::<f64>(),
        )
    }

    pub fn random_unit_vec(rng: &mut ThreadRng) -> Self {
        let mut v = Vec3::random(-1.0, 1.0, rng);
        loop {
            if v.length_squared() < 1.0 {
                break;
            }
            v = Vec3::random(-1.0, 1.0, rng);
        }
        v.unit_vec()
    }

    pub fn random_on_hemisphere(normal: &Vec3, rng: &mut ThreadRng) -> Self {
        let v = Self::random_unit_vec(rng);
        if v.dot(normal) > 0.0 {
            return v;
        } else {
            return -v;
        }
    }
}

impl_op_ex!(-|a: &Vec3| -> Vec3 { Vec3::new(-a.x, -a.y, -a.z) });

impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });

impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });

impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z) });

impl_op_ex!(/|a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x / b.x, a.y / b.y, a.z / b.z) });

impl_op_ex!(+|a: f64, b: &Vec3| -> Vec3 { Vec3::new(a + b.x, a + b.y, a + b.z) });
impl_op_ex!(+|a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x + b, a.y + b, a.z + b) });

impl_op_ex!(-|a: f64, b: &Vec3| -> Vec3 { Vec3::new(a - b.x, a - b.y, a - b.z) });
impl_op_ex!(-|a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x - b, a.y - b, a.z - b) });

impl_op_ex!(*|a: f64, b: &Vec3| -> Vec3 { Vec3::new(a * b.x, a * b.y, a * b.z) });
impl_op_ex!(*|a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b) });

impl_op_ex!(/|a: f64, b: &Vec3| -> Vec3 { Vec3::new(a / b.x, a / b.y, a / b.z) });
impl_op_ex!(/|a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b) });
