// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `backend` module exposes the contents of the most appropriate backend
//! graphics API for the target platform.
//!
//! Common backend structure types are exposed so that other modules can make
//! direct use of these types rather than making every type and function generic
//! on the backend type.

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

pub type Device = <Backend as gfx_hal::Backend>::Device;
pub type QueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;

pub type Surface = <Backend as gfx_hal::Backend>::Surface;
pub type Swapchain = <Backend as gfx_hal::Backend>::Swapchain;
pub type Framebuffer = <Backend as gfx_hal::Backend>::Framebuffer;

pub type Image = <Backend as gfx_hal::Backend>::Image;
pub type ImageView = <Backend as gfx_hal::Backend>::ImageView;

pub type RenderPass = <Backend as gfx_hal::Backend>::RenderPass;
pub type PipelineLayout = <Backend as gfx_hal::Backend>::PipelineLayout;
pub type GraphicsPipeline = <Backend as gfx_hal::Backend>::GraphicsPipeline;
pub type ShaderModule = <Backend as gfx_hal::Backend>::ShaderModule;

pub type CommandPool = <Backend as gfx_hal::Backend>::CommandPool;
pub type CommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;
pub type CommandQueue = <Backend as gfx_hal::Backend>::CommandQueue;

pub type Fence = <Backend as gfx_hal::Backend>::Fence;
pub type Semaphore = <Backend as gfx_hal::Backend>::Semaphore;

pub type Memory = <Backend as gfx_hal::Backend>::Memory;
pub type Buffer = <Backend as gfx_hal::Backend>::Buffer;

pub type Sampler = <Backend as gfx_hal::Backend>::Sampler;

pub type DescriptorSetLayout = <Backend as gfx_hal::Backend>::DescriptorSetLayout;
pub type DescriptorPool = <Backend as gfx_hal::Backend>::DescriptorPool;
pub type DescriptorSet = <Backend as gfx_hal::Backend>::DescriptorSet;
