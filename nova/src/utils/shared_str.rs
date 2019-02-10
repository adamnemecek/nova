use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

/// An immutable `str` container that can be freely shared across threads.
/// 
/// The value is stored in either an `Arc<str>` or `&'static str`. In the latter
/// case, no allocation is made.
#[derive(Clone)]
pub enum SharedStr {
  Static(&'static str),
  Arc(Arc<str>),
}

impl Default for SharedStr {
  fn default() -> Self {
    SharedStr::Static("")
  }
}

impl<'a> From<&'a SharedStr> for SharedStr {
  fn from(value: &'a SharedStr) -> Self {
    value.clone()
  }
}

impl From<&'static str> for SharedStr {
  fn from(value: &'static str) -> Self {
    SharedStr::Static(value)
  }
}

impl From<String> for SharedStr {
  fn from(value: String) -> Self {
    SharedStr::Arc(value.into_boxed_str().into())
  }
}

impl<'a> From<Cow<'a, str>> for SharedStr {
  fn from(value: Cow<'a, str>) -> Self {
    SharedStr::from(value.into_owned())
  }
}

impl AsRef<str> for SharedStr {
  fn as_ref(&self) -> &str {
    match self {
      SharedStr::Static(value) => value,
      SharedStr::Arc(ref value) => value,
    }
  }
}

impl Deref for SharedStr {
  type Target = str;

  fn deref(&self) -> &str {
    self.as_ref()
  }
}

impl fmt::Debug for SharedStr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.as_ref())
  }
}

impl fmt::Display for SharedStr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.as_ref())
  }
}

impl PartialEq for SharedStr {
  fn eq(&self, other: &SharedStr) -> bool {
    self.as_ref() == other.as_ref()
  }
}

impl Hash for SharedStr {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      SharedStr::Static(value) => value.hash(state),
      SharedStr::Arc(value) => value.hash(state),
    }
  }
}
