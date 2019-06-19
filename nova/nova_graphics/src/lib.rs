// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod alloc;
mod backend;
mod buffer;
mod cmd;
mod color;
mod context;
mod fence;
mod image;
mod pipeline;
pub mod renderer;
mod semaphore;
mod shader;
mod render_pass;
mod framebuffer;
mod surface;
mod vertex;

use self::render_pass::*;
use self::framebuffer::*;
use self::alloc::*;
use self::buffer::*;
use self::color::*;
pub use self::context::*;
use self::fence::*;
use self::image::*;
use self::pipeline::*;
use self::semaphore::*;
use self::surface::*;
use gfx_hal::Device as _;
use nova_log as log;
use nova_math::{self as math, Point2, Size};
use nova_sync::{channel, Mutex, MutexGuard};
use nova_time as time;
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
