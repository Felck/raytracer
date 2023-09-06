use auto_ops::*;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn avg_by_samples(&self, samples: u32) -> Self {
        Self {
            r: self.r / samples as f64,
            g: self.g / samples as f64,
            b: self.b / samples as f64,
        }
    }
}

impl_op_ex!(+|a: &Color, b: &Color| -> Color { Color::new(a.r + b.r, a.g + b.g, a.b + b.b) });

impl_op_ex!(*|a: f64, b: &Color| -> Color { Color::new(a * b.r, a * b.g, a * b.b) });

impl_op_ex!(/|a: &Color, b: u32| -> Color { Color::new(a.r / b as f64, a.g / b as f64, a.b / b as f64) });

impl_op_ex!(+=|a: &mut Color, b: &Color| { a.r += b.r; a.g += b.g; a.b += b.b });
