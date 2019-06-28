// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::{Element, ElementContext};
use crate::layout::{HorizontalAlign, Layout, VerticalAlign};

#[derive(Debug, PartialEq)]
pub struct Fill;

impl Element for Fill {
  type State = ();

  fn on_awake(&self, ctx: ElementContext<Self>) {
    ctx.put_component(Layout::Fill);
  }

  fn on_sleep(&self, ctx: ElementContext<Self>) {
    ctx.remove_component::<Layout>();
  }
}

#[derive(Debug, Default, PartialEq)]
pub struct AspectRatioFill(pub f32);

impl Element for AspectRatioFill {
  type State = ();

  fn on_awake(&self, ctx: ElementContext<Self>) {
    ctx.put_component(Layout::AspectRatioFill(self.0));
  }

  fn on_change(&self, _old_self: Self, mut ctx: ElementContext<Self>) {
    ctx.put_component(Layout::AspectRatioFill(self.0));
    ctx.rebuild();
  }

  fn on_sleep(&self, ctx: ElementContext<Self>) {
    ctx.remove_component::<Layout>();
  }
}

#[derive(Debug, PartialEq)]
pub struct Align(pub HorizontalAlign, pub VerticalAlign);

impl Element for Align {
  type State = ();

  fn on_awake(&self, ctx: ElementContext<Self>) {
    ctx.put_component(Layout::Align(self.0, self.1));
  }

  fn on_change(&self, _old_self: Self, mut ctx: ElementContext<Self>) {
    ctx.put_component(Layout::Align(self.0, self.1));
    ctx.rebuild();
  }

  fn on_sleep(&self, ctx: ElementContext<Self>) {
    ctx.remove_component::<Layout>();
  }
}
