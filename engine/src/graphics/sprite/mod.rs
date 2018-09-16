// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use prelude::*;

use super::Atlas;

pub mod animation;

pub use self::animation::{Animated, Animator};

/// Component representing a sprite to be drawn.
#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Sprite {
  /// Atlas to source cells from.
  pub atlas: Arc<Atlas>,
  /// Cell in the atlas to render.
  pub cell: usize,
  /// Whether to flip the sprite horizontally.
  pub hflip: bool,
}
