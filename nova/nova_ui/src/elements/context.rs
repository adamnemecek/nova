// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::Element;
use crate::nodes::NodeContext;
use std::ops::{Deref, DerefMut};

pub struct ElementContext<'a, E: Element> {
  pub state: &'a mut E::State,
  node: NodeContext<'a>,
}

impl<'a, E: Element + 'static> ElementContext<'a, E> {
  pub fn new(state: &'a mut E::State, node: NodeContext<'a>) -> Self {
    Self { state, node }
  }
}

impl<'a, E: Element> Deref for ElementContext<'a, E> {
  type Target = NodeContext<'a>;

  fn deref(&self) -> &Self::Target {
    &self.node
  }
}

impl<'a, E: Element> DerefMut for ElementContext<'a, E> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.node
  }
}
