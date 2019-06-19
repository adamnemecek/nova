// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Reusable list of commands recorded for submission to the device.
pub struct List {
  pool: Pool,
  buffer: Option<backend::CommandBuffer>,
}

impl List {
  /// Creates a new command buffer using the given pool.
  pub fn new(pool: &Pool) -> Self {
    List {
      buffer: Some(pool.allocate()),
      pool: pool.clone(),
    }
  }

  /// Begins recording commands.
  ///
  /// This function returns a `Recorder` structure for recording the actual
  /// commands. Recording is finished when the structure is dropped or when
  /// the `Recorder::finish` function is called.
  pub fn record(&mut self) -> Recorder {
    Recorder::new(&self.pool, self.buffer.as_mut().unwrap())
  }

  /// Returns a reference to the underlying backend command buffer.
  pub(crate) fn as_backend(&self) -> &backend::CommandBuffer {
    self.buffer.as_ref().unwrap()
  }
}

impl Drop for List {
  fn drop(&mut self) {
    self.pool.recycle(self.buffer.take().unwrap());
  }
}
