// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod constraints;
mod system;

use self::constraints::Constraints;
use nova_core::ecs;
use nova_core::engine::{Engine};
use nova_core::math::{Size};
use std::f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layout {
  Stack,
  Fill,
  FixedSize(Size<f32>),
}

impl ecs::Component for Layout {
  type Storage = ecs::HashMapStorage<Self>;
}

impl Default for Layout {
  fn default() -> Self {
    Layout::Stack
  }
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<Layout>(engine.resources_mut());
  system::setup(engine);
}
