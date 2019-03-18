use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;
use std::borrow::Borrow;

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

impl<'a> From<&'a String> for SharedStr {
  fn from(value: &'a String) -> Self {
    From::<String>::from(value.clone())
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

impl AsRef<OsStr> for SharedStr {
  fn as_ref(&self) -> &OsStr {
    (**self).as_ref()
  }
}

impl Deref for SharedStr {
  type Target = str;

  fn deref(&self) -> &str {
    self.as_ref()
  }
}

impl Borrow<str> for SharedStr {
  fn borrow(&self) -> &str {
    self.as_ref()
  }
}

impl fmt::Debug for SharedStr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", &**self)
  }
}

impl fmt::Display for SharedStr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", &**self)
  }
}

impl PartialEq for SharedStr {
  fn eq(&self, other: &SharedStr) -> bool {
    **self == **other
  }
}

impl Eq for SharedStr {}

impl Hash for SharedStr {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      SharedStr::Static(value) => value.hash(state),
      SharedStr::Arc(value) => value.hash(state),
    }
  }
}
