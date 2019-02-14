// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Context, Element, MountContext, Node, ShouldRebuild};
use derive_more::*;
use std::any::Any;
use std::fmt;
use std::mem;

pub(super) trait Instance: Any + Send + Sync + fmt::Debug {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: MountContext,
  ) -> Result<ShouldRebuild, Box<dyn Any>>;

  fn awake(&mut self, ctx: MountContext);
  fn sleep(&mut self, ctx: MountContext);

  fn on_message(
    &mut self,
    payload: Box<dyn Any>,
    ctx: MountContext,
  ) -> Result<ShouldRebuild, Box<dyn Any>>;

  fn build(&mut self, ctx: MountContext) -> Node;
}

#[derive(Debug, Deref, DerefMut)]
pub(super) struct InstanceBox(Box<dyn Instance>);

impl InstanceBox {
  pub fn new<T: Element + 'static>(element: T) -> Self {
    InstanceBox(Box::new(ElementInstance::new(element)))
  }
}

#[derive(Debug)]
struct ElementInstance<T: Element> {
  element: T,
  state: T::State,
  awake: bool,
}

impl<T: Element> ElementInstance<T> {
  fn new(element: T) -> Self {
    ElementInstance {
      element,
      state: T::State::default(),
      awake: false,
    }
  }
}

impl<T: Element + 'static> Instance for ElementInstance<T> {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: MountContext,
  ) -> Result<ShouldRebuild, Box<dyn Any>> {
    let mut element = element.downcast::<T>()?;

    if *element == self.element {
      return Ok(ShouldRebuild(false));
    }

    mem::swap(&mut self.element, &mut *element);

    Ok(
      self
        .element
        .on_change(*element, Context::new(ctx, &mut self.state)),
    )
  }

  fn awake(&mut self, ctx: MountContext) {
    if self.awake {
      return;
    }

    self.awake = true;
    self.element.on_awake(Context::new(ctx, &mut self.state));
  }

  fn sleep(&mut self, ctx: MountContext) {
    if !self.awake {
      return;
    }

    self.element.on_sleep(Context::new(ctx, &mut self.state));
    self.awake = false;
  }

  fn on_message(
    &mut self,
    msg: Box<dyn Any>,
    ctx: MountContext,
  ) -> Result<ShouldRebuild, Box<dyn Any>> {
    let msg = msg.downcast::<T::Message>()?;

    Ok(
      self
        .element
        .on_message(*msg, Context::new(ctx, &mut self.state)),
    )
  }

  fn build(&mut self, ctx: MountContext) -> Node {
    self.element.build(Context::new(ctx, &mut self.state))
  }
}
