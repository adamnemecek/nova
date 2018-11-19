// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod backend;
pub mod buffer;
pub mod commands;
pub mod device;
pub mod hal;
pub mod image;
pub mod render;
pub mod window;

mod color;
mod fence;
mod gpu;
mod semaphore;

pub use self::backend::Backend;
pub use self::buffer::Buffer;
pub use self::color::Color4;
pub use self::commands::{CommandPool, Commands};
pub use self::device::Device;
pub use self::fence::Fence;
pub use self::gpu::Gpu;
pub use self::image::Image;
pub use self::semaphore::Semaphore;

pub mod prelude {
  pub use super::backend::{self, Backend};
  pub use super::hal;
  pub use super::hal::command::RawCommandBuffer as AbstractRawCommandBuffer;
  pub use super::hal::pool::RawCommandPool as AbstractRawCommandPool;
  pub use super::hal::queue::QueueFamily as AbstractQueueFamily;
  pub use super::hal::queue::RawCommandQueue as AbstractRawCommandQueue;
  pub use super::hal::DescriptorPool as AbstractDescriptorPool;
  pub use super::hal::Device as AbstractDevice;
  pub use super::hal::Instance as AbstractInstance;
  pub use super::hal::PhysicalDevice as AbstractPhysicalDevice;
  pub use super::hal::Surface as AbstractSurface;
  pub use super::hal::Swapchain as AbstractSwapchain;
}
