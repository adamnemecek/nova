// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Synchronization primitive used to control order of execution between command
/// buffers.
///
/// This structure is cloneable and all clones refer to the same semaphore. When
/// all clones are dropped, the underlying backend resource is destroyed.
#[derive(Clone)]
pub struct Semaphore(Arc<SemaphoreInner>);

struct SemaphoreInner {
  context: Context,
  semaphore: Expect<backend::Semaphore>,
}

impl Semaphore {
  /// Creates a new semaphore in the given context.
  pub fn new(context: &Context) -> Result<Self, OutOfMemoryError> {
    let semaphore = context.device.create_semaphore()?;

    Ok(Self(Arc::new(SemaphoreInner {
      semaphore: semaphore.into(),
      context: context.clone(),
    })))
  }

  /// Returns a reference to the underlying backend semaphore.
  pub(crate) fn as_backend(&self) -> &backend::Semaphore {
    &self.0.semaphore
  }
}

impl Drop for SemaphoreInner {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_semaphore(self.semaphore.take());
    }
  }
}
