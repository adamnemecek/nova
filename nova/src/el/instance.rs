// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::message;
use super::{ChildNodes, Context, Element, Node, ShouldRebuild};
use crate::ecs;
use derive_more::*;
use std::any::Any;
use std::fmt;
use std::mem;

pub trait Instance: Any + Send + Sync + fmt::Debug {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    message_queue: &message::DeliveryQueue,
  ) -> Result<ShouldRebuild, Box<dyn Any>>;

  fn awake(&mut self, entity: ecs::Entity, message_queue: &message::DeliveryQueue);
  fn sleep(&mut self, message_queue: &message::DeliveryQueue);

  fn on_message(
    &mut self,
    payload: Box<dyn Any>,
    message_queue: &message::DeliveryQueue,
  ) -> Result<ShouldRebuild, Box<dyn Any>>;

  fn build(&mut self, children: ChildNodes, message_queue: &message::DeliveryQueue) -> Node;
}

#[derive(Debug, Deref, DerefMut)]
pub struct InstanceBox(Box<dyn Instance>);

impl InstanceBox {
  pub(super) fn new<T: Element + 'static>(element: T) -> Self {
    InstanceBox(Box::new(ElementInstance::new(element)))
  }
}

#[derive(Debug)]
struct ElementInstance<T: Element> {
  element: T,
  state: T::State,
  entity: Option<ecs::Entity>,
}

impl<T: Element> ElementInstance<T> {
  fn new(element: T) -> Self {
    ElementInstance {
      element,
      state: T::State::default(),
      entity: None,
    }
  }
}

impl<T: Element + 'static> Instance for ElementInstance<T> {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    message_queue: &message::DeliveryQueue,
  ) -> Result<ShouldRebuild, Box<dyn Any>> {
    let mut element = element.downcast::<T>()?;

    if *element == self.element {
      return Ok(ShouldRebuild(false));
    }

    mem::swap(&mut self.element, &mut *element);

    Ok(self.element.on_change(
      *element,
      Context {
        state: &mut self.state,
        entity: self.entity.expect("Element is not awake."),
        message_queue,
      },
    ))
  }

  fn awake(&mut self, entity: ecs::Entity, message_queue: &message::DeliveryQueue) {
    if self.entity.is_some() {
      return;
    }

    self.entity = Some(entity);

    self.element.on_awake(Context {
      state: &mut self.state,
      entity,
      message_queue,
    });
  }

  fn sleep(&mut self, message_queue: &message::DeliveryQueue) {
    if let Some(entity) = self.entity {
      self.element.on_sleep(Context {
        state: &mut self.state,
        entity,
        message_queue,
      });

      self.entity = None;
    }
  }

  fn on_message(
    &mut self,
    msg: Box<dyn Any>,
    message_queue: &message::DeliveryQueue,
  ) -> Result<ShouldRebuild, Box<dyn Any>> {
    let msg = msg.downcast::<T::Message>()?;

    if let Some(entity) = self.entity {
      Ok(self.element.on_message(
        *msg,
        Context {
          state: &mut self.state,
          entity,
          message_queue,
        },
      ))
    } else {
      Ok(ShouldRebuild(false))
    }
  }

  fn build(&mut self, children: ChildNodes, message_queue: &message::DeliveryQueue) -> Node {
    self.element.build(
      children,
      Context {
        state: &mut self.state,
        entity: self.entity.expect("Element is not awake."),
        message_queue,
      },
    )
  }
}
