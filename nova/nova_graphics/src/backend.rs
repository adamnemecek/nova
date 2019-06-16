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
