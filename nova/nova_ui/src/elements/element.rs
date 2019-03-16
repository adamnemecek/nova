// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::{ElementContext, ElementState};
use crate::specs::{ChildSpecs, Spec};
use std::fmt;

pub trait Element: PartialEq + Send + Sync + fmt::Debug + Sized {
  type State: ElementState + Send + Sync + fmt::Debug + 'static;

  fn on_awake(&self, _ctx: ElementContext<Self>) {}
  fn on_sleep(&self, _ctx: ElementContext<Self>) {}

  fn on_change(&self, _old: Self, mut ctx: ElementContext<Self>) {
    ctx.rebuild();
  }

  fn build(&self, children: ChildSpecs, _ctx: ElementContext<Self>) -> Spec {
    children.into()
  }
}
