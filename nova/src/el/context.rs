// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::hierarchy;
use super::{Element, MessageComposer, MessageQueue};
use crate::ecs;
use crate::engine;
use std::fmt;

pub struct Context<'a, E: Element> {
  pub state: &'a mut E::State,
  pub resources: &'a engine::Resources,
  pub entities: &'a ecs::Entities,
  pub entity: ecs::Entity,
  pub messages: &'a MessageQueue,
}

impl<'a, E: Element + 'static> Context<'a, E> {
  pub fn new(state: &'a mut E::State, ctx: &'a hierarchy::Context) -> Self {
    Context::<'a> {
      state,
      resources: ctx.resources,
      entities: ctx.entities,
      entity: ctx.entity,
      messages: ctx.messages,
    }
  }

  pub fn compose<I, A>(&self, arg: A, composer: fn(A, I) -> E::Message) -> MessageComposer<I>
  where
    I: fmt::Debug + 'static,
    A: Clone + PartialEq + Send + Sync + fmt::Debug + 'static,
  {
    let recipient = self.entity;

    MessageComposer::<I>::new::<E, A>(recipient, arg, composer)
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
