use crate::ecs::derive::*;
use crate::graphics;

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Style {
  pub background_color: graphics::Color,
}

impl Default for Style {
  fn default() -> Self {
    Style {
      background_color: graphics::Color::TRANSPARENT,
    }
  }
}
