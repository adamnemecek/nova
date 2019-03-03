// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Color;
use nova_core::ecs;
use nova_core::Engine;
use nova_graphics::images::ImageSlice;

#[derive(Debug, PartialEq, Clone)]
pub struct Style {
  pub bg_color: Color,
  pub bg_image: Option<ImageSlice>,
}

impl Default for Style {
  fn default() -> Self {
    Self {
      bg_color: Color::WHITE,
      bg_image: None,
    }
  }
}

impl ecs::Component for Style {
  type Storage = ecs::HashMapStorage<Self>;
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<Style>(engine.resources_mut());
}
