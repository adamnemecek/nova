// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::error::DeviceCreationError;

use super::*;

pub struct Context {
  memory: Memory,
  queues: cmd::Queues,
  device: backend::Device,
  _adapter: backend::Adapter,
  _backend: Arc<backend::Instance>,
}

impl Context {
  pub(crate) fn new(
    backend: impl Into<Arc<backend::Instance>>,
    adapter: backend::Adapter,
    device: backend::Device,
    queues: cmd::Queues,
  ) -> Self {
    let memory = Memory::new(&adapter);

    Context { memory, queues, device, _adapter: adapter, _backend: backend.into() }
  }

  pub(crate) fn device(&self) -> &backend::Device {
    &self.device
  }

  pub(crate) fn memory(&self) -> &Memory {
    &self.memory
  }

  pub(crate) fn queues(&self) -> &cmd::Queues {
    &self.queues
  }

  /// Waits for the graphics device to be idle, meaning no command buffers are
  /// being executed.
  pub(crate) fn wait_idle(&self) {
    let _ = self.device.wait_idle();
  }
}
