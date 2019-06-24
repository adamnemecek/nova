// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod cmd;
pub mod shader;
pub mod vertex;

mod alloc;
mod backend;
mod buffer;
mod color;
mod context;
mod descriptor;
mod fence;
mod framebuffer;
mod image;
mod image_data;
mod loader;
mod pipeline;
mod render_pass;
mod sampler;
mod semaphore;
mod surface;

pub use self::buffer::*;
pub use self::color::*;
pub use self::context::*;
pub use self::descriptor::*;
pub use self::fence::*;
pub use self::framebuffer::*;
pub use self::image::*;
pub use self::image_data::*;
pub use self::loader::*;
pub use self::pipeline::*;
pub use self::render_pass::*;
pub use self::sampler::*;
pub use self::semaphore::*;
pub use self::surface::*;

use self::alloc::*;
use gfx_hal::Device as _;
use nova_log as log;
use nova_math::{self as math, Point2, Rect, Size};
use nova_sync::{channel, Mutex, MutexGuard};
use nova_window as window;
use std::sync::Arc;
use std::{cmp, fmt, iter, mem, ops, slice, thread};

/// An error that occurs when there is not enough device or host memory to
/// complete an operation.
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
