#[derive(Debug, Clone, Copy)]
pub struct Color(pub [f32; 4]);

impl Color {
  pub const WHITE: Self = Color([1.0, 1.0, 1.0, 1.0]);

  pub fn a(&self) -> f32 {
    self.0[0]
  }
}
