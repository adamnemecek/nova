// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::{Element, ElementInstance};
use crate::nodes::NodeContext;
use crate::specs::Spec;
use std::any::Any;
use std::fmt;

pub struct ElementPrototype {
  pub new: fn(Box<dyn Any>, NodeContext) -> ElementInstance,
  pub element: Box<dyn Any>,
  pub child_specs: Vec<Spec>,
}

// The `Box<dyn Any>` stored in the `ElementPrototype` always contains a
// `Send + Sync` element.
unsafe impl Send for ElementPrototype {}
unsafe impl Sync for ElementPrototype {}

impl ElementPrototype {
  pub fn new<E: Element + 'static>(element: E, child_specs: Vec<Spec>) -> Self {
    Self {
      new: |props, ctx| {
        ElementInstance::new::<E>(
          *props
            .downcast::<E>()
            .expect("Incorrect props type for element"),
          ctx,
        )
      },
      element: Box::new(element),
      child_specs,
    }
  }
}

impl fmt::Debug for ElementPrototype {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Prototype")
      .field("element", &self.element)
      .field("children", &self.child_specs)
      .finish()
  }
}
