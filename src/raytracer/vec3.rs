use auto_ops::*;

#[derive(Clone, Copy)]
pub struct Vec3([f64; 3]);

#[macro_export]
macro_rules! vec3 {
    ( $x:expr, $y:expr, $z:expr ) => {{
        Vec3::new($x, $y, $z)
    }};
}
pub(crate) use vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { 0: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn unit_vec(&self) -> Self {
        self / self.length()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            0: [
                self.y() * other.z() - self.z() * other.y(),
                self.z() * other.x() - self.x() * other.z(),
                self.x() * other.y() - self.y() * other.x(),
            ],
        }
    }
}

impl_op_ex!(-|a: &Vec3| -> Vec3 { Vec3::new(-a.x(), -a.y(), -a.z()) });

impl_op_ex!(+|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(a.x() + b.x(), a.y() + b.y(), a.z() + b.z())
});

impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(a.x() - b.x(), a.y() - b.y(), a.z() - b.z())
});

impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(a.x() * b.x(), a.y() * b.y(), a.z() * b.z())
});

impl_op_ex!(/|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(a.x() / b.x(), a.y() / b.y(), a.z() / b.z())
});

impl_op_ex!(+|a: f64, b: &Vec3| -> Vec3 { Vec3::new(a + b.x(), a + b.y(), a + b.z()) });
impl_op_ex!(+|a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x() + b, a.y() + b, a.z() + b) });

impl_op_ex!(-|a: f64, b: &Vec3| -> Vec3 { Vec3::new(a - b.x(), a - b.y(), a - b.z()) });
impl_op_ex!(-|a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x() - b, a.y() - b, a.z() - b) });

impl_op_ex!(*|a: f64, b: &Vec3| -> Vec3 { Vec3::new(a * b.x(), a * b.y(), a * b.z()) });
impl_op_ex!(*|a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x() * b, a.y() * b, a.z() * b) });

impl_op_ex!(/|a: f64, b: &Vec3| -> Vec3 { Vec3::new(a / b.x(), a / b.y(), a / b.z()) });
impl_op_ex!(/|a: &Vec3, b: f64| -> Vec3 { Vec3::new(a.x() / b, a.y() / b, a.z() / b) });
