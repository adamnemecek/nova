// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::{Element, ElementContext, ElementState as _, MessageHandler};
use crate::messages::MessagePayload;
use crate::nodes::NodeContext;
use crate::specs::{ChildSpecs, Spec};
use nova_core::collections::FnvHashMap;
use std::any::{Any, TypeId};
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub(crate) struct ElementInstance(Box<dyn NodeElement>);

impl ElementInstance {
  pub fn new<T: Element + 'static>(element: T, ctx: NodeContext) -> Self {
    ElementInstance(Box::new(NodeElementImpl::new(element, ctx)))
  }
}

impl Deref for ElementInstance {
  type Target = dyn NodeElement;

  fn deref(&self) -> &dyn NodeElement {
    &*self.0
  }
}

impl DerefMut for ElementInstance {
  fn deref_mut(&mut self) -> &mut dyn NodeElement {
    &mut *self.0
  }
}

pub(crate) trait NodeElement: Any + Send + Sync + fmt::Debug {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: NodeContext,
  ) -> Result<(), Box<dyn Any>>;

  fn awake(&mut self, ctx: NodeContext);
  fn sleep(&mut self, ctx: NodeContext);

  fn build(&mut self, children: ChildSpecs, ctx: NodeContext) -> Spec;

  fn on_message(&mut self, ctx: NodeContext, payload: MessagePayload)
    -> Result<(), MessagePayload>;
}

struct NodeElementImpl<T: Element> {
  element: T,
  state: T::State,
  awake: bool,
  message_handlers: FnvHashMap<TypeId, MessageHandler<T>>,
}

impl<T: Element> NodeElementImpl<T> {
  fn new(element: T, ctx: NodeContext) -> Self {
    let state = T::State::new(ctx);

    NodeElementImpl {
      element,
      state,
      awake: false,
      message_handlers: FnvHashMap::default(),
    }
  }
}

impl<T: Element + 'static> NodeElement for NodeElementImpl<T> {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: NodeContext,
  ) -> Result<(), Box<dyn Any>> {
    let mut element = (element as Box<dyn Any>).downcast::<T>()?;

    if *element != self.element {
      mem::swap(&mut self.element, &mut *element);

      self.element.on_change(
        *element,
        ElementContext {
          node: ctx,
          state: &mut self.state,
          message_handlers: &mut self.message_handlers,
        },
      );
    }

    Ok(())
  }

  fn awake(&mut self, ctx: NodeContext) {
    if self.awake {
      return;
    }

    self.awake = true;

    self.element.on_awake(ElementContext {
      node: ctx,
      state: &mut self.state,
      message_handlers: &mut self.message_handlers,
    });
  }

  fn sleep(&mut self, mut ctx: NodeContext) {
    if !self.awake {
      return;
    }

    for (type_id, _) in self.message_handlers.drain() {
      ctx.unsubscribe(type_id);
    }

    self.element.on_sleep(ElementContext {
      node: ctx,
      state: &mut self.state,
      message_handlers: &mut self.message_handlers,
    });

    self.awake = false;
  }

  fn build(&mut self, children: ChildSpecs, ctx: NodeContext) -> Spec {
    self.element.build(
      children,
      ElementContext {
        node: ctx,
        state: &mut self.state,
        message_handlers: &mut self.message_handlers,
      },
    )
  }

  fn on_message(
    &mut self,
    ctx: NodeContext,
    payload: MessagePayload,
  ) -> Result<(), MessagePayload> {
    let type_id = (*payload).type_id();

    match self.message_handlers.remove(&type_id) {
      Some(mut handler) => {
        let result = handler(
          &self.element,
          ElementContext {
            node: ctx,
            state: &mut self.state,
            message_handlers: &mut self.message_handlers,
          },
          payload,
        );

        self.message_handlers.insert(type_id, handler);

        result
      }

      _ => Err(payload),
    }
  }
}

impl<E: Element> fmt::Debug for NodeElementImpl<E> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("NodeElement")
      .field("element", &self.element)
      .field("state", &self.state)
      .field("awake", &self.awake)
      .finish()
  }
}
