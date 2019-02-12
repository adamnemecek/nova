// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Node;
use super::{Element, InstanceBox};
use std::any::Any;

#[derive(Debug)]
pub struct Prototype {
  pub new: fn(Box<dyn Any>) -> InstanceBox,
  pub props: Box<dyn Any>,
  pub children: Vec<Node>,
}

impl Prototype {
  pub fn new<E: Element + 'static>(props: E::Props, children: Vec<Node>) -> Self {
    Prototype {
      new: |props| {
        InstanceBox::new::<E>(
          *props
            .downcast::<E::Props>()
            .expect("Incorrect props type for element"),
        )
      },
      props: Box::new(props),
      children,
    }
  }
}
