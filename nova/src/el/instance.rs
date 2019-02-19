// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::hierarchy;
use super::spec::{self, Spec};
use super::{Context, Element, ShouldRebuild};
use derive_more::*;
use std::any::Any;
use std::fmt;
use std::mem;

#[derive(Debug, Deref, DerefMut)]
pub(crate) struct Instance(Box<dyn InstanceLike>);

impl Instance {
  pub fn new<T: Element + 'static>(element: T) -> Self {
    Instance(Box::new(ElementInstance::new(element)))
  }
}

pub(crate) trait InstanceLike: Any + Send + Sync + fmt::Debug {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: &mut hierarchy::Context,
  ) -> Result<ShouldRebuild, Box<dyn Any>>;

  fn awake(&mut self, ctx: &mut hierarchy::Context);
  fn sleep(&mut self, ctx: &mut hierarchy::Context);

  fn on_message(
    &mut self,
    payload: Box<dyn Any>,
    ctx: &mut hierarchy::Context,
  ) -> Result<ShouldRebuild, Box<dyn Any>>;

  fn build(&mut self, children: &hierarchy::Children, ctx: &mut hierarchy::Context) -> Spec;
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

impl<T: Element + 'static> InstanceLike for ElementInstance<T> {
  fn replace_element(
    &mut self,
    element: Box<dyn Any>,
    ctx: &mut hierarchy::Context,
  ) -> Result<ShouldRebuild, Box<dyn Any>> {
    let mut element = (element as Box<dyn Any>).downcast::<T>()?;

    if *element == self.element {
      return Ok(ShouldRebuild(false));
    }

    mem::swap(&mut self.element, &mut *element);

    Ok(
      self
        .element
        .on_change(*element, Context::new(&mut self.state, ctx)),
    )
  }

  fn awake(&mut self, ctx: &mut hierarchy::Context) {
    if self.awake {
      return;
    }

    self.awake = true;

    self.element.on_awake(Context::new(&mut self.state, ctx));
  }

  fn sleep(&mut self, ctx: &mut hierarchy::Context) {
    if !self.awake {
      return;
    }

    self.element.on_sleep(Context::new(&mut self.state, ctx));

    self.awake = false;
  }

  fn on_message(
    &mut self,
    msg: Box<dyn Any>,
    ctx: &mut hierarchy::Context,
  ) -> Result<ShouldRebuild, Box<dyn Any>> {
    let msg = msg.downcast::<T::Message>()?;

    Ok(
      self
        .element
        .on_message(*msg, Context::new(&mut self.state, ctx)),
    )
  }

  fn build(&mut self, children: &hierarchy::Children, ctx: &mut hierarchy::Context) -> Spec {
    self.element.build(
      spec::Children {
        entities: children.entities.iter(),
      },
      Context::new(&mut self.state, ctx),
    )
  }
}
