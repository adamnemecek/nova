// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use crate::specs::ChildSpecs;

use crate::elements::{Element, ElementPrototype};
use nova_core::entities::Entity;
use std::iter;
use std::mem;
use std::vec;

#[derive(Debug)]
pub struct Spec(Content);

#[derive(Debug)]
enum Content {
  List(Vec<Spec>),
  Element(ElementPrototype),
  Entity(Entity),
}

impl Default for Spec {
  fn default() -> Self {
    Self(Content::List(Vec::new()))
  }
}

impl Spec {
  pub fn new<E>(element: E, children: impl Into<Spec>) -> Self
  where
    E: Element + 'static,
  {
    Self(Content::Element(ElementPrototype::new(
      element,
      children.into().into(),
    )))
  }

  pub fn from_entity(entity: Entity) -> Self {
    Self(Content::Entity(entity))
  }

  pub fn entity(&self) -> Option<Entity> {
    match &self.0 {
      Content::Entity(entity) => Some(*entity),
      _ => None,
    }
  }

  pub fn into_element_prototype(self) -> ElementPrototype {
    match self.0 {
      Content::Element(prototype) => prototype,
      Content::List(_) => panic!("Cannot convert a list node into an element prototype."),
      Content::Entity(_) => panic!("Cannot convert an entity node into an element prototype."),
    }
  }
}

impl From<()> for Spec {
  fn from(_: ()) -> Self {
    Spec::default()
  }
}

impl From<[Spec; 0]> for Spec {
  fn from(_: [Spec; 0]) -> Self {
    Spec::default()
  }
}

impl From<Option<Spec>> for Spec {
  fn from(node: Option<Spec>) -> Self {
    match node {
      Some(node) => node,
      None => Spec::default(),
    }
  }
}

impl<E: Element + 'static> From<E> for Spec {
  fn from(element: E) -> Self {
    Self::new(element, Spec::default())
  }
}

impl<'a> From<ChildSpecs<'a>> for Spec {
  fn from(children: ChildSpecs<'a>) -> Self {
    children.collect()
  }
}

impl From<Vec<Spec>> for Spec {
  fn from(mut specs: Vec<Spec>) -> Self {
    // Flatten nested list nodes.
    let mut i = 0;

    while i < specs.len() {
      if let Spec(Content::List(_)) = specs[i] {
        // Swap an empty node with the list node in the vec.
        //
        // This is more efficient than removing it, which would move all later
        // elements back one index.
        let mut spec = Spec::default();

        mem::swap(&mut specs[i], &mut spec);

        // Splice the list's children into the vec at its former position,
        // which will overwrite the empty node that was just swapped in.
        specs.splice(i..=i, spec.into_iter());
      }

      i += 1;
    }

    Spec(Content::List(specs))
  }
}

impl iter::FromIterator<Spec> for Spec {
  fn from_iter<I: IntoIterator<Item = Spec>>(iter: I) -> Self {
    iter.into_iter().collect::<Vec<_>>().into()
  }
}

impl From<Spec> for Vec<Spec> {
  fn from(spec: Spec) -> Self {
    match spec {
      Spec(Content::List(specs)) => specs,
      node => vec![node],
    }
  }
}

impl IntoIterator for Spec {
  type Item = Spec;
  type IntoIter = IntoIter;

  fn into_iter(self) -> IntoIter {
    match self.0 {
      Content::List(specs) => IntoIter::List(specs.into_iter()),
      content => IntoIter::Element(iter::once(Spec(content))),
    }
  }
}

pub enum IntoIter {
  Element(iter::Once<Spec>),
  List(vec::IntoIter<Spec>),
}

impl From<Spec> for IntoIter {
  fn from(spec: Spec) -> Self {
    match spec.0 {
      Content::List(specs) => IntoIter::List(specs.into_iter()),
      content => IntoIter::Element(iter::once(Spec(content))),
    }
  }
}

impl Iterator for IntoIter {
  type Item = Spec;

  fn next(&mut self) -> Option<Spec> {
    match self {
      IntoIter::Element(iter) => iter.next(),
      IntoIter::List(iter) => iter.next(),
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    match self {
      IntoIter::Element(iter) => iter.size_hint(),
      IntoIter::List(iter) => iter.size_hint(),
    }
  }
}

impl ExactSizeIterator for IntoIter {}
