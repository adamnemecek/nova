// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// List of commands that can be submitted to a command queue of a graphics
/// device.
pub struct List {
  pool: Rc<RefCell<Pool>>,
  buffer: Expect<backend::CommandBuffer>,
}

impl List {
  /// Creates a new command list using the given pool.
  pub fn new(pool: &Rc<RefCell<Pool>>) -> Self {
    List { buffer: pool.borrow_mut().allocate().into(), pool: pool.clone() }
  }

  /// Returns the queue ID this command list was created for.
  pub fn queue_id(&self) -> QueueId {
    self.pool.borrow().queue_id()
  }

  /// Begins recording commands, returning a `Recorder` struct with methods for
  /// adding commands to the list.
  pub fn begin(&mut self) -> Recorder {
    Recorder::new(&self.pool, &mut self.buffer)
  }

  /// Returns a reference to the underlying backend command buffer.
  pub fn as_backend(&self) -> &backend::CommandBuffer {
    &self.buffer
  }
}

impl Drop for List {
  fn drop(&mut self) {
    self.pool.borrow_mut().recycle(self.buffer.take());
  }
}
