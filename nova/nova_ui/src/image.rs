// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::{Element, ElementContext};
use crate::layout::{Constraints, Layout};
use nova_core::components::{self, Component, HashMapStorage};
use nova_core::math::Size;
use nova_core::resources::Resources;
use nova_graphics::images::ImageSlice;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
  pub slice: ImageSlice,
}

impl Image {
  pub fn new(slice: impl Into<ImageSlice>) -> Self {
    Image {
      slice: slice.into(),
    }
  }
}

impl Component for Image {
  type Storage = HashMapStorage<Self>;
}

impl Element for Image {
  type State = ();

  fn on_awake(&self, ctx: ElementContext<Self>) {
    let mut size: Size<f32> = self.slice.data.size().into();

    size.width *= self.slice.rect.width();
    size.height *= self.slice.rect.height();

    ctx.put_component(self.clone());

    ctx.put_component(Layout::Constrained(Constraints {
      min: size,
      max: size,
    }));
  }

  fn on_change(&self, _: Self, mut ctx: ElementContext<Self>) {
    ctx.rebuild();

    self.on_awake(ctx);
  }

  fn on_sleep(&self, ctx: ElementContext<Self>) {
    ctx.remove_component::<Image>();
  }
}

pub fn set_up(res: &mut Resources) {
  components::register::<Image>(res);
}
