pub mod prelude;

mod buffer;
mod commands;
mod pass;
mod pipeline;
mod renderer;
mod shader;
mod swapchain;
mod sync;
mod texture;
mod vertices;

pub use self::backend::NAME as BACKEND_NAME;
pub use self::buffer::*;
pub use self::commands::*;
pub use self::pass::*;
pub use self::pipeline::*;
pub use self::renderer::*;
pub use self::shader::*;
pub use self::swapchain::*;
pub use self::sync::*;
pub use self::texture::*;
pub use self::vertices::*;

use self::prelude::*;
use super::backend;
use super::window;
use crate::prelude::*;
use std::sync::{Arc, Mutex};

const ENGINE_NAME: &str = "nova";
const ENGINE_VERSION: u32 = 1;

pub struct Device {
  queues: CommandQueueSet,
  pub(super) raw: backend::Device,
  memory_properties: gfx_hal::MemoryProperties,
  adapter: backend::Adapter,
  _instance: Arc<backend::Instance>,
}

impl Device {
  pub fn queues(&self) -> &CommandQueueSet {
    &self.queues
  }
}

/// Selects the best available device adapter.
fn select_adapter(
  instance: &backend::Instance,
  surface: &backend::Surface,
) -> Option<backend::Adapter> {
  instance
    .enumerate_adapters()
    .into_iter()
    // Only select adapters with at least one graphics queue that supports
    // presentation to the surface.
    .filter(|adapter| {
      adapter
        .queue_families
        .iter()
        .any(|f| f.supports_graphics() && surface.supports_queue_family(f))
    })
    // Only select adapters with at least one non-graphics queue.
    .filter(|adapter| {
      adapter
        .queue_families
        .iter()
        .any(|f| !f.supports_graphics())
    })
    // Select the adapter with the higest score:
    .max_by_key(|adapter| {
      let mut score = 0;

      // Prefer discrete graphics devices over integrated ones.
      if adapter.info.device_type == gfx_hal::adapter::DeviceType::DiscreteGpu {
        score += 1000;
      }

      score
    })
}

pub fn init() -> Result<(Arc<Device>, window::Window), InitError> {
  // Create an instance of the backend.
  let instance = Arc::new(backend::Instance::create(ENGINE_NAME, ENGINE_VERSION));

  // Create a window.
  let mut window = window::Window::new(&instance)?;

  // Select the best available adapter.
  let adapter =
    select_adapter(&instance, window.raw_surface()).ok_or(InitError::NoGraphicsDevice)?;

  // Cache the memory properties info.
  let memory_properties = adapter.physical_device.memory_properties();

  // Determine queue families to open.
  let queue_families = select_queue_families(&adapter, window.raw_surface());

  // Create a logical device and queues.
  let gpu = adapter
    .physical_device
    .open(&queue_families.create_info())?;

  let device = Arc::new(Device {
    _instance: instance,
    adapter,
    memory_properties,
    raw: gpu.device,
    queues: CommandQueueSet::new(gpu.queues, &queue_families),
  });

  Ok((device, window))
}

quick_error! {
  #[derive(Debug)]
  pub enum InitError {
    NoGraphicsDevice {
      display("No devices available that support graphics command and presentation to the window surface.")
    }
    CouldNotCreateWindow(err: window::CreationError) {
      display("Could not create a window to render on.")
      from()
    }
    NoSupportedQueue {
      display("Device has no queues that support graphics commands and presentation to the window surface.")
    }
    CouldNotCreateDevice(err: gfx_hal::error::DeviceCreationError) {
      display("Could not create device: {}", err)
      from()
    }
  }
}
