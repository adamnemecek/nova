// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::layout::Layout;
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
