use std::ops::{Deref, DerefMut};

pub struct Nullable<T> {
  value: Option<T>,
}

impl<T> Nullable<T> {
  pub fn new() -> Self {
    Nullable { value: None }
  }

  pub fn take(&mut self) -> Option<T> {
    self.value.take()
  }
}

impl<T> From<T> for Nullable<T> {
  fn from(value: T) -> Self {
    Nullable { value: Some(value) }
  }
}

impl<T> Default for Nullable<T> {
  fn default() -> Self {
    Nullable::new()
  }
}

impl<T> Deref for Nullable<T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.value.as_ref().expect("value is null")
  }
}

impl<T> DerefMut for Nullable<T> {
  fn deref_mut(&mut self) -> &mut T {
    self.value.as_mut().expect("value is null")
  }
}

impl<T> AsRef<T> for Nullable<T> {
  fn as_ref(&self) -> &T {
    self.deref()
  }
}
