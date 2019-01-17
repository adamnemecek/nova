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
pub use gfx_backend_dx12::{Backend, Instance as BackendInstance};
#[cfg(target_os = "macos")]
pub use gfx_backend_metal::{Backend, Instance as BackendInstance};
#[cfg(all(unix, not(target_os = "macos")))]
pub use gfx_backend_vulkan::{Backend, Instance as BackendInstance};

pub use gfx_hal::Instance as BackendInstanceExt;

#[cfg(windows)]
pub const BACKEND_NAME: &str = "DirectX 12";
#[cfg(target_os = "macos")]
pub const BACKEND_NAME: &str = "Metal";
#[cfg(all(unix, not(target_os = "macos")))]
pub const BACKEND_NAME: &str = "Vulkan";
