// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::ffi::OsStr;

#[repr(transparent)]
pub struct Path(str);

impl Path {
  pub fn is_absolute(&self) -> bool {
    self.0.starts_with('/')
  }

  pub fn components(&self) -> PathIter {
    PathIter::new(self)
  }

  pub fn has_prefix(&self, prefix: impl AsRef<Path>) -> bool {
    self.strip_prefix(prefix).is_some()
  }

  pub fn strip_prefix(&self, prefix: impl AsRef<Path>) -> Option<&Path> {
    let mut components = self.components();

    for prefix_comp in prefix.as_ref().components() {
      match components.next() {
        Some(comp) if comp == prefix_comp => continue,
        _ => return None,
      }
    }

    Some(components.as_path())
  }

  pub fn join(&self, other: impl AsRef<Path>) -> PathBuf {
    let mut path: PathBuf = self.into();

    path.append(other);
    path
  }
}

impl AsRef<Path> for str {
  fn as_ref(&self) -> &Path {
    unsafe { &*(self as *const str as *const Path) }
  }
}

impl AsRef<str> for Path {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl AsRef<OsStr> for Path {
  fn as_ref(&self) -> &OsStr {
    self.0.as_ref()
  }
}

impl AsRef<Path> for String {
  fn as_ref(&self) -> &Path {
    let value: &str = self.as_ref();

    value.as_ref()
  }
}

impl AsRef<FsPath> for Path {
  fn as_ref(&self) -> &FsPath {
    self.0.as_ref()
  }
}

impl fmt::Display for Path {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut iter = self.components();

    match iter.next() {
      Some("/") => write!(f, "/{}", iter.next().unwrap_or(""))?,
      Some(".") => write!(f, "{}", iter.next().unwrap_or("."))?,
      None => return Ok(()),

      Some(c) => panic!("unexpected initial path component: {}", c),
    };

    for component in iter {
      write!(f, "/{}", component)?;
    }

    Ok(())
  }
}
