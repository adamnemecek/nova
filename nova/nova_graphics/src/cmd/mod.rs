// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod barrier;
mod list;
mod pool;
mod queue;
mod recorder;
mod submission;

pub use self::barrier::*;
pub use self::list::*;
pub use self::pool::*;
pub use self::queue::*;
pub use self::recorder::*;
pub use self::submission::*;
use super::*;
use gfx_hal::command::RawCommandBuffer as _;

/// Description of a copy operation from a source buffer to a destination
/// buffer.
pub struct BufferCopy {
  /// Range of data to copy from the source buffer.
  pub src_range: ops::Range<u64>,
  /// Index at which to copy the data into the destination buffer.
  pub dest_index: u64,
}
