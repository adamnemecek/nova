use std::ops::{Deref, DerefMut};

pub trait Destroy {
  fn destroy(self);
}

pub struct Destroyable<T: Destroy>(Option<T>);

impl<T: Destroy> From<T> for Destroyable<T> {
  fn from(value: T) -> Self {
    Destroyable(Some(value))
  }
}

impl<T: Destroy> Deref for Destroyable<T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.0.as_ref().expect("destroyed")
  }
}

impl<T: Destroy> DerefMut for Destroyable<T> {
  fn deref_mut(&mut self) -> &mut T {
    self.0.as_mut().expect("destroyed")
  }
}

impl<T: Destroy> Drop for Destroyable<T> {
  fn drop(&mut self) {
    if let Some(value) = self.0.take() {
      value.destroy();
    }
  }
}
