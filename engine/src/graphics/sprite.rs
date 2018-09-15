// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::prelude::*;
use std::sync::Arc;

use super::Atlas;

/// Component representing a sprite to be drawn.
pub struct Sprite {
  /// Atlas to source frames from.
  pub atlas: Arc<Atlas>,
  /// Index of the frame in the atlas to render.
  pub cell: usize,
}

impl Component for Sprite {
  type Storage = HashMapStorage<Self>;
}
