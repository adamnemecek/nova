// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Use DirectX 12 on Windows, Metal on MacOS, and Vulkan on Linux.
#[cfg(windows)]
pub use gfx_backend_dx12::*;
#[cfg(target_os = "macos")]
pub use gfx_backend_metal::*;
#[cfg(all(unix, not(target_os = "macos")))]
pub use gfx_backend_vulkan::*;

#[cfg(windows)]
pub const NAME: &str = "DirectX 12";
#[cfg(target_os = "macos")]
pub const NAME: &str = "Metal";
#[cfg(all(unix, not(target_os = "macos")))]
pub const NAME: &str = "Vulkan";

pub type Adapter = gfx_hal::Adapter<Backend>;
pub type Device = <Backend as gfx_hal::Backend>::Device;

pub type Queue = <Backend as gfx_hal::Backend>::CommandQueue;
pub type Queues = gfx_hal::queue::Queues<Backend>;
pub type QueueFamily = <Backend as gfx_hal::Backend>::QueueFamily;

pub type Surface = <Backend as gfx_hal::Backend>::Surface;
pub type Swapchain = <Backend as gfx_hal::Backend>::Swapchain;

pub type Image = <Backend as gfx_hal::Backend>::Image;
pub type ImageView = <Backend as gfx_hal::Backend>::ImageView;

pub type Fence = <Backend as gfx_hal::Backend>::Fence;
pub type Semaphore = <Backend as gfx_hal::Backend>::Semaphore;

pub type RenderPass = <Backend as gfx_hal::Backend>::RenderPass;

pub type Framebuffer = <Backend as gfx_hal::Backend>::Framebuffer;

pub type CommandPool = <Backend as gfx_hal::Backend>::CommandPool;
pub type CommandBuffer = <Backend as gfx_hal::Backend>::CommandBuffer;
