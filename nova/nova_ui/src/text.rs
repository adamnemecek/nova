// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod fonts;

mod positioned;

pub use glyph_brush_layout::{HorizontalAlign, VerticalAlign};

use nova_core::ecs;
use nova_core::engine::Engine;

#[derive(Debug)]
pub struct Text {
  pub content: String,
  pub h_align: HorizontalAlign,
  pub v_align: VerticalAlign,
}

impl Default for Text {
  fn default() -> Self {
    Text {
      content: String::new(),
      h_align: HorizontalAlign::Center,
      v_align: VerticalAlign::Center,
    }
  }
}

impl ecs::Component for Text {
  type Storage = ecs::HashMapStorage<Self>;
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<Text>(engine.resources_mut());

  fonts::setup(engine);
  positioned::setup(engine);
}
