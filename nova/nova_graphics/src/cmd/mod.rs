// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod barrier;
mod list;
mod pool;
mod recorder;

pub use self::barrier::*;
pub use self::list::*;
pub use self::pool::*;
pub use self::recorder::*;

use super::*;
use gfx_hal::command::RawCommandBuffer as _;

pub struct BufferCopy {
  pub src_range: ops::Range<u64>,
  pub dst_offset: u64,
}
