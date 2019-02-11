// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Node;
use super::{Element, RebuildNeeded};
use derive_more::*;
use std::any::Any;
use std::fmt;

pub trait Instance: fmt::Debug {
  fn build(&mut self) -> Node;
  fn set_props(&mut self, props: Box<dyn Any>) -> Result<RebuildNeeded, Box<dyn Any>>;
  fn on_mount(&mut self);
  fn on_unmount(&mut self);
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
  element: Option<T>,
  props: T::Props,
}

impl<T: Element> ElementInstance<T> {
  fn new(props: T::Props) -> Self {
    ElementInstance {
      element: None,
      props,
    }
  }
}

impl<T: Element> Instance for ElementInstance<T> {
  fn build(&mut self) -> Node {
    let element = self.element.as_mut().expect("element is not mounted");

    element.build(&self.props)
  }

  fn set_props(&mut self, props: Box<dyn Any>) -> Result<RebuildNeeded, Box<dyn Any>> {
    let props = props.downcast()?;

    if *props == self.props {
      return Ok(RebuildNeeded::No);
    }

    self.props = *props;

    if let Some(element) = self.element.as_mut() {
      return Ok(element.on_prop_change(&self.props));
    }

    Ok(RebuildNeeded::No)
  }

  fn on_mount(&mut self) {
    self.element = Some(T::new(&self.props));
  }

  fn on_unmount(&mut self) {
    self.element = None;
  }
}
