// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::message::{self, Message, MessageComposer};
use super::mount;
use super::{ChildNodes, Element};
use crate::ecs;
use std::fmt;

pub struct Context<'a, E: Element> {
  pub state: &'a mut E::State,
  pub entity: ecs::Entity,
  pub children: ChildNodes<'a>,
  pub(super) message_queue: &'a message::DeliveryQueue,
}

impl<'a, E: Element + 'static> Context<'a, E> {
  pub(super) fn new(ctx: MountContext<'a>, state: &'a mut E::State) -> Self {
    Context {
      state,
      entity: ctx.entity,
      children: ctx.children,
      message_queue: ctx.message_queue,
    }
  }

  pub fn compose(&self, msg: E::Message) -> Message {
    let recipient = self.entity;

    Message {
      recipient,
      payload: Box::new(msg),
    }
  }

  pub fn compose_with<I, A>(&self, arg: A, composer: fn(I, A) -> E::Message) -> MessageComposer<I>
  where
    I: fmt::Debug + 'static,
    A: Clone + PartialEq + Send + Sync + fmt::Debug + 'static,
  {
    let recipient = self.entity;

    MessageComposer::<I>::new::<E, A>(recipient, arg, composer)
  }

  pub fn send(&self, message: Message) {
    self.message_queue.messages.push(message);
  }
}

pub(super) struct MountContext<'a> {
  pub entity: ecs::Entity,
  pub children: ChildNodes<'a>,
  pub message_queue: &'a message::DeliveryQueue,
}

impl<'a> MountContext<'a> {
  pub fn new(
    entity: ecs::Entity,
    children: &'a mount::Children,
    message_queue: &'a message::DeliveryQueue,
  ) -> Self {
    MountContext {
      entity,
      children: ChildNodes {
        entities: children.entities.iter(),
      },
      message_queue,
    }
  }
}
