// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod barrier;
mod fence;
mod list;
mod pool;
mod queue;
mod recorder;
mod semaphore;
mod submission;

pub use self::{barrier::*, fence::*, list::*, pool::*, queue::*, recorder::*, semaphore::*, submission::*};
use super::*;
use gfx_hal::command::RawCommandBuffer as _;
pub use gfx_hal::image::Layout as ImageLayout;

/// Description of a copy operation from a source buffer to a destination
/// buffer.
pub struct BufferCopy {
  /// Range of data to copy from the source buffer.
  pub src_range: ops::Range<u64>,
  /// Index at which to copy the data into the destination buffer.
  pub dest_index: u64,
}
