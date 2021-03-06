// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod elements;

mod align;
mod constraints;
mod system;

pub use self::align::{HorizontalAlign, VerticalAlign};
pub use self::constraints::Constraints;

use nova_core::components::{self, Component, HashMapStorage};
use nova_core::engine::Engine;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
  Constrained(Constraints),
  Fill,
  AspectRatioFill(f32),
  Align(HorizontalAlign, VerticalAlign),
}

impl Component for Layout {
  type Storage = HashMapStorage<Self>;
}

impl Default for Layout {
  fn default() -> Self {
    Layout::Constrained(Constraints::default())
  }
}

pub fn set_up(engine: &mut Engine) {
  components::register::<Layout>(&mut engine.resources);

  system::set_up(engine);
}
