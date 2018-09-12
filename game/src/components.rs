use ggez::graphics::{Color, Image, Rect};
use specs::prelude::*;

/// Component that stores the position of an entity in the world.
///
/// One unit is the size of one pixel in a sprite, which may be larger than one
/// screen pixel depending on DPI.
pub struct Position {
  /// West/East coordinate. East is positive.
  pub x: f32,
  /// North/South coordinate. South is positive.
  pub y: f32,
  /// Up/Down coordinate. Up is positive.
  pub z: f32,
}

impl Component for Position {
  type Storage = VecStorage<Self>;
}

/// Component that stores information on the sprite that represents the entity.
pub struct Sprite {
  pub image: Image,
  pub color: Color,
  pub rect: Rect,
}

impl Component for Sprite {
  type Storage = HashMapStorage<Self>;
}
