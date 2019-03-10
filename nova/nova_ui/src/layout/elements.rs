// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::layout::{HorizontalAlign, Layout, VerticalAlign};
use nova_core::el;

#[derive(Debug, PartialEq)]
pub struct Fill;

impl el::Element for Fill {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    ctx.put_component(Layout::Fill);
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    ctx.remove_component::<Layout>();
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct AspectRatioFill(pub f32);

impl el::Element for AspectRatioFill {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    ctx.put_component(Layout::AspectRatioFill(self.0));
  }

  fn on_change(&self, _old_self: Self, ctx: el::Context<Self>) -> el::ShouldRebuild {
    ctx.put_component(Layout::AspectRatioFill(self.0));

    el::ShouldRebuild(true)
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    ctx.remove_component::<Layout>();
  }
}

#[derive(Debug, PartialEq)]
pub struct Align(pub HorizontalAlign, pub VerticalAlign);

impl el::Element for Align {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    ctx.put_component(Layout::Align(self.0, self.1));
  }

  fn on_change(&self, _old_self: Self, ctx: el::Context<Self>) -> el::ShouldRebuild {
    ctx.put_component(Layout::Align(self.0, self.1));

    el::ShouldRebuild(true)
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    ctx.remove_component::<Layout>();
  }
}
