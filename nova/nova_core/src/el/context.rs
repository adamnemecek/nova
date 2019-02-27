// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::message::{self, MessageFn, MessageQueue};
use super::Element;
use crate::ecs;
use crate::engine;
use std::ops::Deref;

#[derive(Clone, Copy)]
pub struct NodeContext<'a> {
  pub resources: &'a engine::Resources,
  pub entities: &'a ecs::Entities,
  pub entity: ecs::Entity,
  pub messages: &'a MessageQueue,
}

impl<'a> NodeContext<'a> {
  pub fn message_fn<I: 'static, P: message::Payload>(&self, func: fn(I) -> P) -> MessageFn<I> {
    MessageFn::new(self.entity, func)
  }

  pub fn put_component<T: ecs::Component>(&self, component: T) {
    ecs::write_components(&self.resources)
      .insert(self.entity, component)
      .expect("The element entity is not alive.");
  }

  pub fn remove_component<T: ecs::Component>(&self) {
    ecs::write_components::<T>(&self.resources).remove(self.entity);
  }
}

pub struct Context<'a, E: Element> {
  pub state: &'a mut E::State,
  outer: NodeContext<'a>,
}

impl<'a, E: Element + 'static> Context<'a, E> {
  pub fn new(state: &'a mut E::State, outer: NodeContext<'a>) -> Self {
    Context::<'a> { state, outer }
  }
}

impl<'a, E: Element> Deref for Context<'a, E> {
  type Target = NodeContext<'a>;

  fn deref(&self) -> &Self::Target {
    &self.outer
  }
}
