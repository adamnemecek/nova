// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::specs::Spec;
use nova_core::entities::Entity;
use std::slice;

pub struct ChildSpecs<'a> {
  entities: slice::Iter<'a, Entity>,
}

impl<'a> ChildSpecs<'a> {
  pub(crate) fn new(entities: &'a [Entity]) -> Self {
    Self {
      entities: entities.iter(),
    }
  }
}

impl<'a> Iterator for ChildSpecs<'a> {
  type Item = Spec;

  fn next(&mut self) -> Option<Spec> {
    self.entities.next().cloned().map(Spec::from_entity)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.entities.size_hint()
  }
}

impl<'a> ExactSizeIterator for ChildSpecs<'a> {}
