// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::buffer::Access as BufferAccess;

use super::*;

/// Describes a synchronization barrier in the command list.
pub struct Barrier<'a>(backend::Barrier<'a>);

impl<'a> Barrier<'a> {
  /// Returns a reference to the backend barrier description.
  pub(crate) fn as_backend(&self) -> &backend::Barrier<'a> {
    &self.0
  }
}

/// Returns a barrier description for a buffer or a subset of a buffer.
pub fn buffer_barrier(
  buffer: &Buffer,
  range: impl ops::RangeBounds<u64>,
  access: ops::Range<BufferAccess>,
) -> Barrier {
  let start = match range.start_bound() {
    ops::Bound::Unbounded => None,
    ops::Bound::Included(i) => Some(*i),
    ops::Bound::Excluded(i) => Some(*i + 1),
  };

  let end = match range.start_bound() {
    ops::Bound::Unbounded => None,
    ops::Bound::Included(i) => Some(*i),
    ops::Bound::Excluded(i) => Some(*i - 1),
  };

  Barrier(gfx_hal::memory::Barrier::Buffer {
    target: buffer.as_backend(),
    states: access,
    families: None,
    range: start..end,
  })
}
