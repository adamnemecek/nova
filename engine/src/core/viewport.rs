// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::Rect;

/// Resource containing information about the viewport.
pub struct Viewport {
  /// Width of the viewport in pixels.
  pub width: f32,
  /// Height of the viewport in pixels.
  pub height: f32,
}

impl From<Rect> for Viewport {
  /// Creates a new viewport state from a ggez `Rect`.
  fn from(rect: Rect) -> Self {
    Viewport {
      width: rect.w,
      height: rect.h,
    }
  }
}
