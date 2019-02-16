// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Spec;
use crate::el::{Element, Instance};
use std::any::Any;

#[derive(Debug)]
pub(in crate::el) struct Prototype {
  pub new: fn(Box<dyn Any>) -> Instance,
  pub element: Box<dyn Any>,
  pub children: Vec<Spec>,
}

// The `Box<dyn Any>` stored in the `Prototype` always contains a `Send + Sync`
// element.
unsafe impl Send for Prototype {}
unsafe impl Sync for Prototype {}

impl Prototype {
  pub fn new<E: Element + 'static>(element: E, children: Vec<Spec>) -> Self {
    Prototype {
      new: |props| {
        Instance::new::<E>(
          *props
            .downcast::<E>()
            .expect("Incorrect props type for element"),
        )
      },
      element: Box::new(element),
      children,
    }
  }
}
