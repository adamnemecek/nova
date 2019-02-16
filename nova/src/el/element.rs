// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::message;
use super::spec::{self, Spec};
use super::Context;
use std::fmt;
use std::ops::{Deref, DerefMut};

pub trait Element: PartialEq + Send + Sync + fmt::Debug + Sized {
  type State: Default + Send + Sync + fmt::Debug + 'static;
  type Message: message::Payload;

  fn on_awake(&self, _ctx: Context<Self>) {}
  fn on_sleep(&self, _ctx: Context<Self>) {}

  fn on_change(&self, _old: Self, _ctx: Context<Self>) -> ShouldRebuild {
    ShouldRebuild(true)
  }

  fn on_message(&self, _msg: Self::Message, _ctx: Context<Self>) -> ShouldRebuild {
    ShouldRebuild(false)
  }

  fn build(&self, children: spec::Children, _: Context<Self>) -> Spec {
    children.into()
  }
}

#[derive(Default, PartialEq, Eq, Clone, Copy, Debug)]
pub struct ShouldRebuild(pub bool);

impl Deref for ShouldRebuild {
  type Target = bool;

  fn deref(&self) -> &bool {
    &self.0
  }
}

impl DerefMut for ShouldRebuild {
  fn deref_mut(&mut self) -> &mut bool {
    &mut self.0
  }
}
