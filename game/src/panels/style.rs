use crate::graphics::Color;
use nova::ecs::derive::*;

/// Component that stores the style of a panel.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Style {
  /// Background color of the panel.
  pub color: Color,
  pub background: Background,
}

pub enum Background {
  None,
  Solid,
}

impl Default for Style {
  fn default() -> Self {
    Style {
      color: Color([1.0, 1.0, 1.0, 1.0]),
      background: Background::None,
    }
  }
}
