// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod children;
mod into_iter;
mod prototype;

pub use self::children::Children;
pub use self::into_iter::IntoIter;
pub(crate) use self::prototype::Prototype;

use super::Element;
use crate::ecs;
use std::iter;
use std::mem;
use std::vec;

#[derive(Debug)]
pub struct Spec(Content);

#[derive(Debug)]
enum Content {
  List(Vec<Spec>),
  Element(Prototype),
  Entity(ecs::Entity),
}

impl Spec {
  pub(crate) fn entity(&self) -> Option<ecs::Entity> {
    match &self.0 {
      Content::Entity(entity) => Some(*entity),
      _ => None,
    }
  }

  pub(crate) fn into_element_prototype(self) -> Prototype {
    match self.0 {
      Content::Element(prototype) => prototype,
      Content::List(_) => panic!("Cannot convert a list node into an element prototype."),
      Content::Entity(_) => {
        panic!("Cannot convert an entity node into an element prototype.")
      }
    }
  }
}

impl From<()> for Spec {
  fn from(_: ()) -> Self {
    empty()
  }
}

impl From<[Spec; 0]> for Spec {
  fn from(_: [Spec; 0]) -> Self {
    empty()
  }
}

impl From<Option<Spec>> for Spec {
  fn from(node: Option<Spec>) -> Self {
    match node {
      Some(node) => node,
      None => empty(),
    }
  }
}

impl<E: Element + 'static> From<E> for Spec {
  fn from(element: E) -> Self {
    spec(element, Vec::new())
  }
}

impl<'a> From<Children<'a>> for Spec {
  fn from(children: Children<'a>) -> Self {
    list(children.collect())
  }
}

impl From<Vec<Spec>> for Spec {
  fn from(specs: Vec<Spec>) -> Self {
    list(specs)
  }
}

impl iter::FromIterator<Spec> for Spec {
  fn from_iter<I: IntoIterator<Item = Spec>>(iter: I) -> Self {
    list(iter.into_iter().collect::<Vec<_>>())
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

pub fn empty() -> Spec {
  Spec(Content::List(Vec::new()))
}

pub fn list(mut children: Vec<Spec>) -> Spec {
  // Flatten nested list nodes.
  let mut i = 0;

  while i < children.len() {
    if let Spec(Content::List(_)) = children[i] {
      // Swap an empty node with the list node in the vec.
      //
      // This is more efficient than removing it, which would move all later
      // elements back one index.
      let mut child = empty();

      mem::swap(&mut children[i], &mut child);

      // Splice the list's children into the vec at its former position,
      // which will overwrite the empty node that was just swapped in.
      children.splice(i..=i, child.into_iter());
    }

    i += 1;
  }

  Spec(Content::List(children))
}

pub fn spec(element: impl Element + 'static, children: impl Into<Spec>) -> Spec {
  Spec(Content::Element(Prototype::new(
    element,
    children.into().into(),
  )))
}
