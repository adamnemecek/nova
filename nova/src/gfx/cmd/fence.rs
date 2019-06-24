// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Synchronization primitive used to synchronize execution between the CPU and
/// the graphics device.
pub struct Fence {
  context: Context,
  fence: Expect<backend::Fence>,
}

impl Fence {
  /// Creates a new fence in the given context.
  ///
  /// The `signaled` parameter sets the initial state of the fence.
  pub fn new(context: &Context, signaled: bool) -> Result<Self, OutOfMemoryError> {
    let fence = context.device.create_fence(signaled)?;

    Ok(Self {
      fence: fence.into(),
      context: context.clone(),
    })
  }

  /// Waits for the fence to be signaled, then resets it to unsignaled
  /// immediately.
  pub fn wait_and_reset(&self) {
    unsafe {
      self
        .context
        .device
        .wait_for_fence(self.as_backend(), !0)
        .expect("failed to wait for fence");

      self
        .context
        .device
        .reset_fence(self.as_backend())
        .expect("failed to reset fence");
    }
  }

  /// Returns a reference to the underlying backend fence.
  pub(crate) fn as_backend(&self) -> &backend::Fence {
    &self.fence
  }
}

impl Drop for Fence {
  fn drop(&mut self) {
    unsafe {
      self
        .context
        .device
        .destroy_fence(self.fence.take())
    };
  }
}
