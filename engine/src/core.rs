// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::prelude::*;

/// Component that stores the position of an entity in the world.
///
/// One unit is the size of one pixel in a sprite, which may be larger than one
/// screen pixel depending on DPI.
#[derive(Clone, Copy)]
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
