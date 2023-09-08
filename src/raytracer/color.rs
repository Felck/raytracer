use auto_ops::*;

#[derive(Clone, Copy)]
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
        Self::new(
            self.r / samples as f64,
            self.g / samples as f64,
            self.b / samples as f64,
        )
    }

    pub fn linear_to_gamma(&self) -> Self {
        Self::new(self.r.sqrt(), self.g.sqrt(), self.b.sqrt())
    }
}

impl_op_ex!(+|a: &Color, b: &Color| -> Color { Color::new(a.r + b.r, a.g + b.g, a.b + b.b) });

impl_op_ex!(*|a: &Color, b: &Color| -> Color { Color::new(a.r * b.r, a.g * b.g, a.b * b.b) });

impl_op_ex!(*|a: f64, b: &Color| -> Color { Color::new(a * b.r, a * b.g, a * b.b) });

impl_op_ex!(/|a: &Color, b: u32| -> Color { Color::new(a.r / b as f64, a.g / b as f64, a.b / b as f64) });

impl_op_ex!(+=|a: &mut Color, b: &Color| { a.r += b.r; a.g += b.g; a.b += b.b });
