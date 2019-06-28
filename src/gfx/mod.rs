// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod render;

mod alloc;
mod backend;
mod buffer;
mod cmd;
mod color;
mod context;
mod descriptors;
mod image;
mod image_data;
mod loader;
mod sampler;
mod shader;
mod vertex;

pub use self::context::*;
pub use self::image::*;
pub use self::image_data::*;
pub use self::loader::*;

use self::alloc::*;
use self::buffer::*;
use self::color::*;
use self::descriptors::*;
use self::sampler::*;
use super::*;
use gfx_hal::Device as _;

/// An error which indicates that there is not enough of either host or device
/// memory remaining to complete the operation.
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
