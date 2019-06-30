// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Synchronization primitive used to control order of execution between command
/// buffers.
pub struct Semaphore {
  context: Arc<Context>,
  semaphore: Expect<backend::Semaphore>,
}

impl Semaphore {
  /// Creates a new semaphore in the given context.
  pub fn new(context: &Arc<Context>) -> Result<Self, OutOfMemoryError> {
    let semaphore = context.device().create_semaphore()?;

    Ok(Self { context: context.clone(), semaphore: semaphore.into() })
  }

  /// Returns a reference to the underlying backend semaphore.
  pub fn as_backend(&self) -> &backend::Semaphore {
    &self.semaphore
  }
}

impl Drop for Semaphore {
  fn drop(&mut self) {
    unsafe {
      self.context.device().destroy_semaphore(self.semaphore.take());
    }
  }
}
