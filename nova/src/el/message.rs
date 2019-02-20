// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod queue;

pub use self::queue::MessageQueue;

use crate::ecs;
use std::any::Any;
use std::fmt;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Debug)]
pub struct Message {
  pub(crate) recipient: ecs::Entity,
  pub(crate) payload: Box<dyn Any + Send>,
}

pub trait Payload: Any + Send + fmt::Debug {}

impl<T: Any + Send + fmt::Debug> Payload for T {}

pub struct MessageFn<I>(Arc<dyn Fn(I) -> Message + Send + Sync>);

impl<I> MessageFn<I> {
  pub(crate) fn new<P, F>(recipient: ecs::Entity, func: F) -> Self
  where
    P: Payload + 'static,
    F: Fn(I) -> P + Send + Sync + 'static,
  {
    MessageFn(Arc::new(move |input| Message {
      recipient,
      payload: Box::new(func(input)),
    }))
  }
}

impl<I> Deref for MessageFn<I> {
  type Target = dyn Fn(I) -> Message + Send + Sync;

  fn deref(&self) -> &Self::Target {
    &*self.0
  }
}

impl<I> Clone for MessageFn<I> {
  fn clone(&self) -> Self {
    MessageFn(self.0.clone())
  }
}

impl<I> PartialEq for MessageFn<I> {
  fn eq(&self, other: &Self) -> bool {
    Arc::ptr_eq(&self.0, &other.0)
  }
}

impl<I> fmt::Debug for MessageFn<I> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "MessageFn({:x})",
      &*self.0 as *const _ as *const () as usize
    )
  }
}
