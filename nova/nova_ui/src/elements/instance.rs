// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::{Element, ElementContext, ElementState as _};
use crate::nodes::NodeContext;
use crate::specs::{ChildSpecs, Spec};
use std::any::Any;
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub(crate) struct ElementInstance(Box<dyn NodeElement>);

impl ElementInstance {
  pub fn new<T: Element + 'static>(element: T, ctx: NodeContext) -> Self {
    ElementInstance(Box::new(Inner::new(element, ctx)))
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
}

#[derive(Debug)]
struct Inner<T: Element> {
  element: T,
  state: T::State,
  awake: bool,
}

impl<T: Element> Inner<T> {
  fn new(element: T, ctx: NodeContext) -> Self {
    let state = T::State::new(ctx);

    Self {
      element,
      state,
      awake: false,
    }
  }
}

impl<T: Element + 'static> NodeElement for Inner<T> {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: NodeContext,
  ) -> Result<(), Box<dyn Any>> {
    let mut element = (element as Box<dyn Any>).downcast::<T>()?;

    if *element != self.element {
      mem::swap(&mut self.element, &mut *element);

      self
        .element
        .on_change(*element, ElementContext::new(&mut self.state, ctx));
    }

    Ok(())
  }

  fn awake(&mut self, ctx: NodeContext) {
    if self.awake {
      return;
    }

    self.awake = true;

    self
      .element
      .on_awake(ElementContext::new(&mut self.state, ctx));
  }

  fn sleep(&mut self, ctx: NodeContext) {
    if !self.awake {
      return;
    }

    self
      .element
      .on_sleep(ElementContext::new(&mut self.state, ctx));
    self.awake = false;
  }

  fn build(&mut self, children: ChildSpecs, ctx: NodeContext) -> Spec {
    self
      .element
      .build(children, ElementContext::new(&mut self.state, ctx))
  }
}
