// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Content, Spec};
use std::iter;
use std::vec;

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
