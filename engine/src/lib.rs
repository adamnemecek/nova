// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// TODO: Remove when RLS supports it.
extern crate crossbeam;
extern crate derive_more;
#[cfg(windows)]
extern crate gfx_backend_dx12;
#[cfg(target_os = "macos")]
extern crate gfx_backend_metal;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate gfx_backend_vulkan;
extern crate gfx_hal;
extern crate gfx_memory;
extern crate glsl_to_spirv;
extern crate image;
extern crate nalgebra;
extern crate num_traits;
extern crate quick_error;
extern crate smallvec;
extern crate specs;
extern crate specs_derive;
extern crate winit;

pub mod ecs;
pub mod graphics;
pub mod math;
pub mod utils;
