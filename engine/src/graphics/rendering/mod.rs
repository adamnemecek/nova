mod backend;
mod buffer;
mod commands;
mod pass;
mod pipeline;
mod prelude;
mod renderer;
mod shader;
mod swapchain;
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
pub use self::texture::*;
pub use self::vertices::*;

use self::prelude::*;
use crate::prelude::*;
use crate::window;
use std::sync::{Arc, Mutex};

const ENGINE_NAME: &str = "nova";
const ENGINE_VERSION: u32 = 1;

pub struct Device {
  queues: CommandQueueSet,
  raw: backend::Device,
  memory_properties: gfx_hal::MemoryProperties,
  adapter: backend::Adapter,
  surface: Mutex<backend::Surface>,
  _instance: backend::Instance,
}

impl Device {
  pub fn new(window: &window::Window) -> Result<Arc<Device>, InitError> {
    // Create an instance of the backend.
    let instance = backend::Instance::create(ENGINE_NAME, ENGINE_VERSION);

    // Create a window surface for presentation.
    let surface = instance.create_surface(window.raw());

    // Select the best available adapter.
    let adapter = select_adapter(&instance, &surface).ok_or(InitError::NoGraphicsDevice)?;

    // Cache the memory properties info.
    let memory_properties = adapter.physical_device.memory_properties();

    // Determine queue families to open.
    let queue_families = select_queue_families(&adapter, &surface);

    // Create a logical device and queues.
    let gpu = adapter
      .physical_device
      .open(&queue_families.create_info())?;

    let device = Arc::new(Device {
      _instance: instance,
      surface: Mutex::new(surface),
      adapter,
      memory_properties,
      raw: gpu.device,
      queues: CommandQueueSet::new(gpu.queues, &queue_families),
    });

    Ok(device)
  }

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

quick_error! {
  #[derive(Debug)]
  pub enum InitError {
    NoGraphicsDevice {
      display("No devices available that support graphics command and presentation to the window surface.")
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
