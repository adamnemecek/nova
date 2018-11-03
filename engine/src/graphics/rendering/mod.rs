use quick_error::quick_error;
use std::sync::{Arc, Mutex};

mod backend;
mod pass;
mod pipeline;
mod prelude;
mod queue;
mod renderer;
mod shader;
mod swapchain;

pub use self::pass::RenderPass;
pub use self::pipeline::*;
use self::prelude::*;
use self::queue::CommandQueue;
pub use self::renderer::*;
pub use self::shader::*;
pub use self::swapchain::Swapchain;

const ENGINE_NAME: &str = "nova";
const ENGINE_VERSION: u32 = 1;

pub struct Device {
  command_queue: CommandQueue,
  raw: backend::Device,
  adapter: backend::Adapter,
  surface: Mutex<backend::Surface>,
  _instance: backend::Instance,
}

/// Initializes rendering for the given window, creating a [`Device`].
pub fn init(window: &winit::Window) -> Result<Arc<Device>, InitError> {
  // Create an instance of the backend.
  let instance = backend::Instance::create(ENGINE_NAME, ENGINE_VERSION);

  // Create a window surface for presentation.
  let surface = instance.create_surface(&window);

  // Select the best available adapter.
  let adapter = select_adapter(&instance, &surface).ok_or(InitError::NoGraphicsDevice)?;

  // Select a supported queue family.
  let queue_family = select_queue_family(&adapter, &surface).ok_or(InitError::NoSupportedQueue)?;

  // Create a logical device and queues.
  let mut gpu = adapter.physical_device.open(&[(queue_family, &[1.0])])?;

  // Take the requested command queue.
  let command_queue = CommandQueue::take(&mut gpu.queues, queue_family.id());

  // Return a `Device` wrapper struct.
  Ok(Arc::new(Device {
    _instance: instance,
    surface: Mutex::new(surface),
    adapter,
    raw: gpu.device,
    command_queue,
  }))
}

/// Selects the best available device adapter.
fn select_adapter(
  instance: &backend::Instance,
  surface: &backend::Surface,
) -> Option<backend::Adapter> {
  instance
    .enumerate_adapters()
    .into_iter()
    // Only select adapters with at least one graphics queue.
    .filter(|adapter| adapter.queue_families.iter().any(|f| f.supports_graphics()))
    // Only select adapters with at least one queue that supports presentation
    // to the window surface.
    .filter(|adapter| {
      adapter
        .queue_families
        .iter()
        .any(|f| surface.supports_queue_family(f))
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

/// Selects a queue family that supports graphics and presentation to the
/// window surface.
fn select_queue_family<'a, 'b>(
  adapter: &'a backend::Adapter,
  surface: &'b backend::Surface,
) -> Option<&'a backend::QueueFamily> {
  adapter
    .queue_families
    .iter()
    .filter(|family| family.supports_graphics())
    .filter(|family| surface.supports_queue_family(family))
    .next()
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
