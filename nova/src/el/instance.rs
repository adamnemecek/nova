// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{ChildNodes, Element, Node, ShouldRebuild};
use derive_more::*;
use std::any::Any;
use std::fmt;
use std::mem;

pub trait Instance: Any + Send + Sync + fmt::Debug {
  fn build(&mut self, children: ChildNodes) -> Node;
  fn replace_element(&mut self, element: Box<dyn Any>) -> Result<ShouldRebuild, Box<dyn Any>>;
  fn awake(&mut self);
  fn sleep(&mut self);
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
  fn build(&mut self, children: ChildNodes) -> Node {
    self.element.build(&mut self.state, children)
  }

  fn replace_element(&mut self, element: Box<dyn Any>) -> Result<ShouldRebuild, Box<dyn Any>> {
    let mut element = element.downcast::<T>()?;

    if *element == self.element {
      return Ok(ShouldRebuild(false));
    }

    mem::swap(&mut self.element, &mut *element);

    Ok(self.element.on_change(&mut self.state, *element))
  }

  fn awake(&mut self) {
    if self.awake {
      return;
    }

    self.awake = true;
    self.element.on_awake(&mut self.state);
  }

  fn sleep(&mut self) {
    if !self.awake {
      return;
    }

    self.awake = false;
    self.element.on_sleep(&mut self.state);
  }
}
