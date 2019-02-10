// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod renderer;

use crate::ecs;
use crate::engine;
use crate::graphics::Color4;

pub use self::renderer::*;

pub fn setup(res: &mut engine::Resources) {
  ecs::register::<Layout>(res);
  ecs::register::<Background>(res);
}

pub struct Layout {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

impl ecs::Component for Layout {
  type Storage = ecs::BTreeStorage<Self>;
}

pub struct Background {
  pub color: Color4,
}

impl ecs::Component for Background {
  type Storage = ecs::BTreeStorage<Self>;
}

impl Default for Background {
  fn default() -> Self {
    Background {
      color: Color4::TRANSPARENT,
    }
  }
}
