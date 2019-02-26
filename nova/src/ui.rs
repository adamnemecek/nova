// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod layout;
mod painter;
mod screen;

use crate::ecs;
use crate::el;
use crate::graphics::Image;
use crate::renderer::TextureId;
use crate::Engine;

pub use self::layout::Layout;
pub use self::painter::Painter;
pub use self::screen::Screen;
pub use crate::graphics::Color4 as Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Style {
  pub bg_color: Color,
  pub bg_image: Option<Image>,
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
  type Storage = ecs::BTreeStorage<Self>;
}

#[derive(Debug, Default)]
pub struct StyleCache {
  bg_texture: Option<TextureId>,
}

impl ecs::Component for StyleCache {
  type Storage = ecs::BTreeStorage<Self>;
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
    ctx.put_component(self.layout);
    ctx.put_component(self.style.clone());
    ctx.put_component(StyleCache::default());
  }

  fn on_change(&self, _: Self, ctx: el::Context<Self>) -> el::ShouldRebuild {
    self.on_awake(ctx);

    el::ShouldRebuild(true)
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    ctx.remove_component::<Layout>();
    ctx.remove_component::<Style>();
    ctx.remove_component::<StyleCache>();
  }
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<Style>(engine.resources_mut());
  ecs::register::<StyleCache>(engine.resources_mut());

  screen::setup(engine);
  layout::setup(engine);
}
