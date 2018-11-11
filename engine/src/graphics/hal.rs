pub use gfx_hal::*;

use super::Backend;

pub type Adapter = gfx_hal::Adapter<Backend>;

pub mod pso {
  pub use gfx_hal::pso::*;

  use super::Backend;

  pub type GraphicsShaderSet<'a> = gfx_hal::pso::GraphicsShaderSet<'a, Backend>;
  pub type EntryPoint<'a> = gfx_hal::pso::EntryPoint<'a, Backend>;
}

pub mod command {
  pub use gfx_hal::command::*;

  use super::Backend;

  pub type CommandBufferInheritanceInfo<'a> =
    gfx_hal::command::CommandBufferInheritanceInfo<'a, Backend>;
}

pub mod queue {
  pub use gfx_hal::queue::*;

  use super::Backend;

  pub type Queues = gfx_hal::queue::Queues<Backend>;
}

pub mod prelude {
  pub use crate::graphics::hal;
  pub use gfx_hal::command::RawCommandBuffer as AbstractRawCommandBuffer;
  pub use gfx_hal::pool::RawCommandPool as AbstractRawCommandPool;
  pub use gfx_hal::queue::QueueFamily as AbstractQueueFamily;
  pub use gfx_hal::queue::RawCommandQueue as AbstractRawCommandQueue;
  pub use gfx_hal::DescriptorPool as AbstractDescriptorPool;
  pub use gfx_hal::Device as AbstractDevice;
  pub use gfx_hal::Instance as AbstractInstance;
  pub use gfx_hal::PhysicalDevice as AbstractPhysicalDevice;
  pub use gfx_hal::Surface as AbstractSurface;
  pub use gfx_hal::Swapchain as AbstractSwapchain;
}
