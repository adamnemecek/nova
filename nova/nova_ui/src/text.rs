// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod fonts;
pub mod position;

pub use glyph_brush_layout::{HorizontalAlign, VerticalAlign};

use nova_core::ecs;
use nova_core::el;
use nova_core::engine::Engine;
use nova_core::SharedStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Text {
  pub content: SharedStr,
  pub h_align: HorizontalAlign,
  pub v_align: VerticalAlign,
}

impl Default for Text {
  fn default() -> Self {
    Text {
      content: SharedStr::default(),
      h_align: HorizontalAlign::Center,
      v_align: VerticalAlign::Center,
    }
  }
}

impl ecs::Component for Text {
  type Storage = ecs::HashMapStorage<Self>;
}

impl el::Element for Text {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    ctx.put_component(self.clone());
  }

  fn on_change(&self, _: Self, ctx: el::Context<Self>) -> el::ShouldRebuild {
    self.on_awake(ctx);

    el::ShouldRebuild(true)
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    ctx.remove_component::<Text>();
  }
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<Text>(engine.resources_mut());

  fonts::setup(engine);
  position::setup(engine);
}
