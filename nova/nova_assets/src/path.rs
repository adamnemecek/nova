// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AssetPath {
  string: String,
}

impl From<String> for AssetPath {
  fn from(mut string: String) -> Self {
    if !string.starts_with('/') && !string.starts_with("./") {
      string.insert_str(0, "./");
    }

    AssetPath { string }
  }
}

impl From<&str> for AssetPath {
  fn from(str: &str) -> Self {
    str.to_owned().into()
  }
}

impl AssetPath {
  pub fn has_root(&self) -> bool {
    self.string.starts_with('/')
  }

  pub(crate) fn push_component(&mut self, component: &str) {
    if !self.string.ends_with('/') {
      self.string.push('/');
    }

    self.string.push_str(component);
  }

  pub(crate) fn pop_component(&mut self) {
    if self.string.is_empty() {
      return;
    }

    let index = self.string[0..self.string.len() - 1].rfind('/');

    match index {
      Some(0) | None => self.string.truncate(1),
      Some(index) => self.string.truncate(index),
    }
  }

  #[cfg(test)]
  fn as_str(&self) -> &str {
    &self.string
  }
}

impl fmt::Debug for AssetPath {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(&self.string, f)
  }
}

impl fmt::Display for AssetPath {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Display::fmt(&self.string, f)
  }
}

#[cfg(test)]
mod tests {
  use super::AssetPath;

  #[test]
  fn from_string() {
    assert_eq!("/hello/world", AssetPath::from("/hello/world").as_str());
    assert_eq!("./", AssetPath::from("").as_str());
    assert_eq!("./relative/guy", AssetPath::from("relative/guy").as_str());
  }

  #[test]
  fn push_component() {
    let mut a = AssetPath::from("/hello");
    let mut b = AssetPath::from("/hello/");

    a.push_component("world");
    b.push_component("world");

    assert_eq!("/hello/world", a.as_str());
    assert_eq!("/hello/world", b.as_str());
  }

  #[test]
  fn pop_component() {
    let mut a = AssetPath::from("/hello/world");
    let mut b = AssetPath::from("/hello/world/");
    let mut c = AssetPath::from("/hello");
    let mut d = AssetPath::from("/");

    a.pop_component();
    b.pop_component();
    c.pop_component();
    d.pop_component();

    assert_eq!("/hello", a.as_str());
    assert_eq!("/hello", b.as_str());
    assert_eq!("/", c.as_str());
    assert_eq!("/", d.as_str());

    let mut a = AssetPath::from("hello/world");
    let mut b = AssetPath::from("hello/world/");
    let mut c = AssetPath::from("hello");
    let mut d = AssetPath::from("");

    a.pop_component();
    b.pop_component();
    c.pop_component();
    d.pop_component();

    assert_eq!("./hello", a.as_str());
    assert_eq!("./hello", b.as_str());
    assert_eq!(".", c.as_str());
    assert_eq!(".", d.as_str());
  }
}
