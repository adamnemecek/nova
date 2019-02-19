// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Element, Message};
use crate::ecs;
use std::any::Any;
use std::fmt;

#[derive(Debug)]
pub struct MessageComposer<I> {
  inner: Box<dyn Inner<I>>,
}

impl<I: fmt::Debug + 'static> MessageComposer<I> {
  pub(crate) fn new<E, A>(recipient: ecs::Entity, arg: A, composer: fn(A, I) -> E::Message) -> Self
  where
    E: Element + 'static,
    A: Clone + PartialEq + Send + Sync + fmt::Debug + 'static,
  {
    MessageComposer {
      inner: Box::new(ElementMessageComposer::<E, I, A> {
        recipient,
        arg,
        composer,
      }),
    }
  }

  pub fn compose(&self, input: I) -> Message {
    self.inner.compose(input)
  }
}

impl<I> PartialEq for MessageComposer<I> {
  fn eq(&self, other: &Self) -> bool {
    self.inner.eq(other.inner.as_any())
  }
}

impl<I> Clone for MessageComposer<I> {
  fn clone(&self) -> Self {
    self.inner.clone()
  }
}

trait Inner<I>: Send + Sync + fmt::Debug {
  fn compose(&self, input: I) -> Message;
  fn as_any(&self) -> &dyn Any;
  fn eq(&self, other: &dyn Any) -> bool;
  fn clone(&self) -> MessageComposer<I>;
}

#[derive(Clone)]
struct ElementMessageComposer<E: Element, I, A> {
  recipient: ecs::Entity,
  arg: A,
  composer: fn(A, I) -> E::Message,
}

impl<E, I, A: Clone> Inner<I> for ElementMessageComposer<E, I, A>
where
  E: Element + 'static,
  I: fmt::Debug + 'static,
  A: PartialEq + Send + Sync + fmt::Debug + 'static,
{
  fn compose(&self, input: I) -> Message {
    let payload = (self.composer)(self.arg.clone(), input);

    Message {
      recipient: self.recipient,
      payload: Box::new(payload),
    }
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn eq(&self, other: &dyn Any) -> bool {
    if let Some(other) = other.downcast_ref::<Self>() {
      self.recipient == other.recipient && self.arg == other.arg && self.composer == other.composer
    } else {
      false
    }
  }

  fn clone(&self) -> MessageComposer<I> {
    MessageComposer::new::<E, A>(self.recipient, self.arg.clone(), self.composer)
  }
}

impl<E: Element + fmt::Debug, I, A: fmt::Debug> fmt::Debug for ElementMessageComposer<E, I, A> {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    fmt
      .debug_struct("ElementMessageComposer")
      .field("recipient", &self.recipient.id())
      .field("arg", &self.arg)
      .field("composer", &self.composer)
      .finish()
  }
}
