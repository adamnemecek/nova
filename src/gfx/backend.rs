// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Use DirectX 12 on Windows, Metal on MacOS, and Vulkan on Linux.
#[cfg(windows)]
pub use gfx_backend_dx12::{Backend, Instance};
#[cfg(target_os = "macos")]
pub use gfx_backend_metal::{Backend, Instance};
#[cfg(all(unix, not(target_os = "macos")))]
pub use gfx_backend_vulkan::{Backend, Instance};

#[cfg(windows)]
pub const NAME: &str = "DirectX 12";
#[cfg(target_os = "macos")]
pub const NAME: &str = "Metal";
#[cfg(all(unix, not(target_os = "macos")))]
pub const NAME: &str = "Vulkan";

// Expose actual raw backend types and specific gfx_hal types as type
// definitions with simpler names and signatures. I don't think this is the
// correct way to use gfx_hal, but it does work and seems clearer than messing
// around with generics everywhere.
pub type Adapter = gfx_hal::Adapter<Backend>;
pub type Barrier<'a> = gfx_hal::memory::Barrier<'a, Backend>;
pub type Buffer = <Backend as gfx_hal::Backend>::Buffer;
pub type CommandPool = <Backend as gfx_hal::Backend>::CommandPool;
pub type CommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;
pub type Descriptor<'a> = gfx_hal::pso::Descriptor<'a, Backend>;
pub type DescriptorLayout = <Backend as gfx_hal::Backend>::DescriptorSetLayout;
pub type DescriptorPool = <Backend as gfx_hal::Backend>::DescriptorPool;
pub type DescriptorSet = <Backend as gfx_hal::Backend>::DescriptorSet;
pub type Device = <Backend as gfx_hal::Backend>::Device;
pub type EntryPoint<'a> = gfx_hal::pso::EntryPoint<'a, Backend>;
pub type Fence = <Backend as gfx_hal::Backend>::Fence;
pub type Framebuffer = <Backend as gfx_hal::Backend>::Framebuffer;
pub type Image = <Backend as gfx_hal::Backend>::Image;
pub type ImageView = <Backend as gfx_hal::Backend>::ImageView;
pub type Memory = <Backend as gfx_hal::Backend>::Memory;
pub type PhysicalDevice = <Backend as gfx_hal::Backend>::PhysicalDevice;
pub type Pipeline = <Backend as gfx_hal::Backend>::GraphicsPipeline;
pub type PipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;
pub type Queue = <Backend as gfx_hal::Backend>::CommandQueue;
pub type Queues = gfx_hal::queue::Queues<Backend>;
pub type QueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;
pub type RenderPass = <Backend as gfx_hal::Backend>::RenderPass;
pub type Sampler = <Backend as gfx_hal::Backend>::Sampler;
pub type Semaphore = <Backend as gfx_hal::Backend>::Semaphore;
pub type Shader = <Backend as gfx_hal::Backend>::ShaderModule;
pub type Surface = <Backend as gfx_hal::Backend>::Surface;
pub type Swapchain = <Backend as gfx_hal::Backend>::Swapchain;
