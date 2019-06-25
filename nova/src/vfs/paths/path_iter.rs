// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub struct PathIter<'a> {
  path: &'a str,
  initial: bool,
}

impl<'a> PathIter<'a> {
  pub fn new(path: &'a Path) -> Self {
    Self {
      path: path.as_ref(),
      initial: true,
    }
  }

  pub fn as_path(&self) -> &'a Path {
    unsafe { &*(self.path as *const str as *const Path) }
  }
}

impl<'a> Iterator for PathIter<'a> {
  type Item = &'a str;

  fn next(&mut self) -> Option<Self::Item> {
    if self.initial {
      self.initial = false;

      if self.path.starts_with('/') {
        self.path = &self.path[1..];

        return Some("/");
      } else {
        return Some(".");
      }
    }

    if self.path.is_empty() {
      return None;
    }

    let (component, rest) = match self.path.find('/') {
      Some(index) => (&self.path[..index], &self.path[index + 1..]),
      None => (self.path, ""),
    };

    self.path = rest;

    if component.is_empty() || component == "." {
      return self.next();
    }

    Some(component)
  }
}

impl AsRef<Path> for PathIter<'_> {
  fn as_ref(&self) -> &Path {
    self.as_path()
  }
}
