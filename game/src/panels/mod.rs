mod layout;
mod style;

pub use self::layout::*;
pub use self::style::*;

use nova::ecs;
use nova::ecs::derive::*;
use nova::math::Rect;

/// Component that stores layout state for a panel entity.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Panel {
  /// Dimension indicating distance between the top of the entity's parent and
  /// the top of the entity.
  pub top: Dimension,
  /// Dimension indicating distance between the left side of the entity's parent
  /// and the left side of the entity.
  pub left: Dimension,
  /// Dimension indicating distance between the bottom of the entity's parent
  /// and the bottom of the entity.
  pub bottom: Dimension,
  /// Dimension indicating distance between the right side of the entity's
  /// parent and the right side of the entity.
  pub right: Dimension,
  /// Dimension indicating the size of the entity in the x direction.
  pub width: Dimension,
  /// Dimension indicating the size of the entity in the y direction.
  pub height: Dimension,
  /// Rect describing the location of the entity relative to its parent's rect.
  rect: Rect<f32>,
  /// Rect describing the absolute location of the entity.
  absolute_rect: Rect<f32>,
  /// Parent of this entity if it has one.
  parent: Option<ecs::Entity>,
  /// Children of this entity if it has any.
  children: Vec<ecs::Entity>,
}

impl Panel {
  /// Gets the rect describing the absolute location of the entity.
  pub fn absolute_rect(&self) -> &Rect<f32> {
    &self.absolute_rect
  }
}

impl Default for Panel {
  fn default() -> Self {
    Panel {
      top: Dimension::Auto,
      left: Dimension::Auto,
      bottom: Dimension::Auto,
      right: Dimension::Auto,
      width: Dimension::Auto,
      height: Dimension::Auto,
      rect: Rect::default(),
      absolute_rect: Rect::default(),
      parent: None,
      children: Vec::new(),
    }
  }
}

/// One of the possible dimension definitions for a panel measurement.
pub enum Dimension {
  /// Dimension is automatically calculated from available space.
  Auto,
  /// Dimension is fixed to a specific value.
  Fixed(f32),
}
