// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod renderer;

mod backend;
mod buffer;
mod cmd;
mod color;
mod context;
mod descriptors;
mod framebuffer;
mod image;
mod image_data;
mod memory;
mod pipeline;
mod render_pass;
mod sampler;
mod shader;
mod surface;
mod vertex;

pub use self::context::*;

use self::{
  buffer::*, color::*, descriptors::*, framebuffer::*, image::*, memory::*, render_pass::*,
  sampler::*, surface::*,
};

use super::*;
use gfx_hal::{Device as _, Instance as _, PhysicalDevice as _};

/// An error which indicates that there is not enough of either host or device
/// memory remaining to complete an operation.
#[derive(Debug)]
pub struct OutOfMemoryError;

impl std::error::Error for OutOfMemoryError {}

impl fmt::Display for OutOfMemoryError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "out of memory")
  }
}

impl From<gfx_hal::device::OutOfMemory> for OutOfMemoryError {
  fn from(_: gfx_hal::device::OutOfMemory) -> Self {
    OutOfMemoryError
  }
}
