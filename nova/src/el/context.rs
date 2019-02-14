// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::message::{self, Message, MessageComposer};
use super::Element;
use crate::ecs;
use std::fmt;

pub struct Context<'a, E: Element> {
  pub state: &'a mut E::State,
  pub entity: ecs::Entity,
  pub(super) message_queue: &'a message::DeliveryQueue,
}

impl<'a, E: Element + 'static> Context<'a, E> {
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
