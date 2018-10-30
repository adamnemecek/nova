// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::backend;
use super::backend::Backend;
use super::shaders;
use super::shaders::ShaderPair;
use gfx_hal as hal;
use gfx_hal::{Device, Instance, Surface};
use winit;

const APP_NAME: &str = "nova";
const APP_VERSION: u32 = 1;

/// Abstraction around base resources of the backend graphics API.
pub struct Renderer {
  /// Instance of a graphics backend.
  pub(super) instance: backend::Instance,
  /// Main render surface.
  pub(super) surface: backend::Surface,
  /// Physical graphics device adapter.
  pub(super) adapter: hal::Adapter<Backend>,
  /// Logical graphics device.
  pub(super) device: backend::Device,
  /// Group of graphics command queues for submitting commands.
  pub(super) queue_group: hal::QueueGroup<Backend, hal::Graphics>,
  /// Pool of command queues to use for rendering.
  pub(super) command_pool: hal::CommandPool<Backend, hal::Graphics>,
  pub(super) default_shaders: ShaderPair,
}

impl Renderer {
  pub fn new(window: &winit::Window) -> Renderer {
    // Create an instance of the backend.
    let instance = backend::Instance::create(APP_NAME, APP_VERSION);

    // Create a surface from the window.
    let surface = instance.create_surface(&window);

    // Find the adapter (graphics card) to render with.
    let mut adapter = {
      // Get a list of available adapters.
      let mut adapters = instance.enumerate_adapters();

      // Take the first available adapter.
      //
      // TODO: Find the best available adapter.
      adapters.remove(0)
    };

    // Open a logical device and a graphics queue group supported by the
    // surface.
    //
    // The queue group contains pools of queues that will be used to
    // send commands to the card.

    const QUEUE_COUNT: usize = 1;

    let (device, queue_group) = adapter
      .open_with(QUEUE_COUNT, |family| surface.supports_queue_family(family))
      .expect("could not open device");

    // Create a command pool to get graphics command queues from.

    const MAX_QUEUES: usize = 16;

    let command_pool = device.create_command_pool_typed(
      &queue_group,
      gfx_hal::pool::CommandPoolCreateFlags::empty(),
      MAX_QUEUES,
    );

    let default_shaders = shaders::create_default(&device);

    device.map_memory();

    // Return the completed context.
    Renderer {
      instance,
      surface,
      adapter,
      device,
      queue_group,
      command_pool,
      default_shaders,
    }
  }

  pub fn destroy(self) {
    self.default_shaders.destroy(&self.device);

    self
      .device
      .destroy_command_pool(self.command_pool.into_raw());

    // Drop everything in the reverse order it was created in.
    drop(self.queue_group);
    drop(self.device);
    drop(self.adapter);
    drop(self.surface);
    drop(self.instance);
  }
}
