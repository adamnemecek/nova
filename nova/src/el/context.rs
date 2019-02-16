// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::hierarchy;
use super::{Element, Message, MessageComposer};
use crate::ecs;
use crate::engine;
use std::fmt;

pub struct Context<'a, 'b, E: Element> {
  pub state: &'a mut E::State,
  pub(super) hierarchy: &'a mut hierarchy::Context<'b>,
}

impl<'a, 'b, E: Element + 'static> Context<'a, 'b, E> {
  pub fn entity(&self) -> ecs::Entity {
    self.hierarchy.entity
  }

  pub fn resources(&self) -> &engine::Resources {
    self.hierarchy.resources
  }

  pub fn compose<I, A>(&self, arg: A, composer: fn(A, I) -> E::Message) -> MessageComposer<I>
  where
    I: fmt::Debug + 'static,
    A: Clone + PartialEq + Send + Sync + fmt::Debug + 'static,
  {
    let recipient = self.hierarchy.entity;

    MessageComposer::<I>::new::<E, A>(recipient, arg, composer)
  }

  pub fn send(&self, message: Message) {
    self.hierarchy.queue_message(message);
  }
}
