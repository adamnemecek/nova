// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Layout, Style};
use crate::el;

#[derive(Debug, Default, PartialEq)]
pub struct Container {
  pub layout: Layout,
  pub style: Style,
}

impl el::Element for Container {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    ctx.put_component(self.layout);
    ctx.put_component(self.style.clone());
  }

  fn on_change(&self, _: Self, ctx: el::Context<Self>) -> el::ShouldRebuild {
    self.on_awake(ctx);

    el::ShouldRebuild(true)
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    ctx.remove_component::<Layout>();
    ctx.remove_component::<Style>();
  }
}
