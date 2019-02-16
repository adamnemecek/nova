// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod renderer;

use self::renderer::Renderer;
use crate::ecs;
use crate::el;
use crate::graphics::{self, Color4};
use crate::Engine;

pub use crate::graphics::Color4 as Color;

pub fn setup(engine: &mut Engine, renderer: &mut graphics::Renderer) {
  ecs::register::<Layout>(engine.resources_mut());
  ecs::register::<Style>(engine.resources_mut());

  renderer.add(Renderer::new(renderer));
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Layout {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

impl Layout {
  pub fn set<E: el::Element + 'static>(ctx: &el::Context<E>, value: Layout) {
    let mut layouts = ecs::write_components(ctx.resources());

    let _ = layouts.insert(ctx.entity(), value);
  }

  pub fn unset<E: el::Element + 'static>(ctx: &el::Context<E>) {
    let mut layouts = ecs::write_components::<Layout>(ctx.resources());

    layouts.remove(ctx.entity());
  }
}

impl ecs::Component for Layout {
  type Storage = ecs::BTreeStorage<Self>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Style {
  pub background: Color,
}

impl Style {
  pub fn set<E: el::Element + 'static>(ctx: &el::Context<E>, value: Style) {
    let mut styles = ecs::write_components(ctx.resources());

    let _ = styles.insert(ctx.entity(), value);
  }

  pub fn unset<E: el::Element + 'static>(ctx: &el::Context<E>) {
    let mut styles = ecs::write_components::<Style>(ctx.resources());

    styles.remove(ctx.entity());
  }
}

impl ecs::Component for Style {
  type Storage = ecs::BTreeStorage<Self>;
}

impl Default for Style {
  fn default() -> Self {
    Style {
      background: Color4::TRANSPARENT,
    }
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct Div {
  pub layout: Layout,
  pub style: Style,
}

impl el::Element for Div {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    Layout::set(&ctx, self.layout);
    Style::set(&ctx, self.style);
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    Layout::unset(&ctx);
    Style::unset(&ctx);
  }
}
