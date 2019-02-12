// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{ChildNodes, Element, Node, ShouldRebuild};
use derive_more::*;
use std::any::Any;
use std::fmt;

pub trait Instance: Any + Send + Sync + fmt::Debug {
  fn build(&mut self, children: ChildNodes) -> Node;
  fn set_props(&mut self, props: Box<dyn Any>) -> Result<ShouldRebuild, Box<dyn Any>>;
  fn awake(&mut self);
  fn sleep(&mut self);
}

#[derive(Debug, Deref, DerefMut)]
pub struct InstanceBox(Box<dyn Instance>);

impl InstanceBox {
  pub fn new<T: Element + 'static>(props: T::Props) -> Self {
    InstanceBox(Box::new(ElementInstance::<T>::new(props)))
  }
}

#[derive(Debug)]
struct ElementInstance<T: Element> {
  element: T,
  props: T::Props,
  awake: bool,
}

impl<T: Element> ElementInstance<T> {
  fn new(props: T::Props) -> Self {
    ElementInstance {
      element: T::new(&props),
      props,
      awake: false,
    }
  }
}

impl<T: Element + 'static> Instance for ElementInstance<T> {
  fn build(&mut self, children: ChildNodes) -> Node {
    self.element.build(&self.props, children)
  }

  fn set_props(&mut self, props: Box<dyn Any>) -> Result<ShouldRebuild, Box<dyn Any>> {
    let props = props.downcast()?;

    if *props == self.props {
      return Ok(ShouldRebuild(false));
    }

    self.props = *props;

    Ok(self.element.on_prop_change(&self.props))
  }

  fn awake(&mut self) {
    if self.awake {
      return;
    }

    self.awake = true;
    self.element.on_awake(&self.props);
  }

  fn sleep(&mut self) {
    if !self.awake {
      return;
    }

    self.awake = false;
    self.element.on_sleep(&self.props);
  }
}
