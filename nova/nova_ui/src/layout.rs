// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod elements;

mod constraints;
mod system;

pub use self::constraints::Constraints;

use nova_core::ecs;
use nova_core::engine::Engine;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
  Constrained(Constraints),
  Fill,
  AspectRatioFill(f32),
}

impl ecs::Component for Layout {
  type Storage = ecs::HashMapStorage<Self>;
}

impl Default for Layout {
  fn default() -> Self {
    Layout::Constrained(Constraints::default())
  }
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<Layout>(engine.resources_mut());
  system::setup(engine);
}
