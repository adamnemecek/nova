// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use gfx_hal::buffer::Access as BufferAccess;
pub use gfx_hal::image::Access as ImageAccess;

use super::*;

/// Describes a synchronization barrier in the command list.
pub struct Barrier<'a>(backend::Barrier<'a>);

impl<'a> Barrier<'a> {
  /// Returns a reference to the backend barrier description.
  pub fn as_backend(&self) -> &backend::Barrier<'a> {
    &self.0
  }
}

/// Returns a pipeline barrier description for an [`Image`].
pub fn image_barrier(
  image: &Image,
  access: ops::Range<ImageAccess>,
  layouts: ops::Range<ImageLayout>,
) -> Barrier {
  Barrier(gfx_hal::memory::Barrier::Image {
    target: image.as_backend(),
    states: (access.start, layouts.start)..(access.end, layouts.end),
    families: None,
    range: gfx_hal::image::SubresourceRange {
      aspects: gfx_hal::format::Aspects::COLOR,
      levels: 0..1,
      layers: 0..1,
    },
  })
}
