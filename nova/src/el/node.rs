// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Element, InstanceBox};

#[derive(Debug, Default)]
pub struct Node {
  instance: Option<InstanceBox>,
  children: Vec<Node>,
}

pub fn empty() -> Node {
  Default::default()
}

pub fn node<T: Element + 'static>(props: T::Props, children: Vec<Node>) -> Node {
  Node {
    instance: Some(InstanceBox::new::<T>(props)),
    children: Vec::new(),
  }
}

pub fn list(children: Vec<Node>) -> Node {
  Node {
    instance: None,
    children,
  }
}
