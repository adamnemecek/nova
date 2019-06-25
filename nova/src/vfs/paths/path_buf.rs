// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub struct PathBuf(String);

impl PathBuf {
  pub fn append(&mut self, path: impl AsRef<Path>) {
    let path: &str = path.as_ref().as_ref();

    if path.starts_with('/') {
      self.0.replace_range(.., path);

      return;
    }

    if !self.0.ends_with('/') {
      self.0.reserve(path.len() + 1);
      self.0.push('/');
    }

    self.0.push_str(path);
  }

  pub fn prepend(&mut self, path: impl AsRef<Path>) {
    if self.is_absolute() {
      return;
    }

    let path: &str = path.as_ref().as_ref();

    if !path.ends_with('/') {
      self.0.reserve(path.len() + 1);
      self.0.insert(0, '/');
    }

    self.0.insert_str(0, path);
  }
}

impl<'a> From<&'a str> for PathBuf {
  fn from(path: &'a str) -> Self {
    PathBuf(path.to_owned())
  }
}

impl<'a> From<&'a Path> for PathBuf {
  fn from(path: &'a Path) -> Self {
    let path: &str = path.as_ref();

    PathBuf(path.to_owned())
  }
}

impl From<String> for PathBuf {
  fn from(path: String) -> Self {
    PathBuf(path)
  }
}

impl AsRef<Path> for PathBuf {
  fn as_ref(&self) -> &Path {
    self.0.as_ref()
  }
}

impl ops::Deref for PathBuf {
  type Target = Path;

  fn deref(&self) -> &Path {
    self.as_ref()
  }
}

impl fmt::Display for PathBuf {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", &*self)
  }
}
