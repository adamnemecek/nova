// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::graphics::Rect;

/// Resource that stores the dimensions of the screen or window the engine is
/// drawing to.
pub struct Viewport {
  /// Width of the viewport in pixels.
  pub width: f32,
  /// Height of the viewport in pixels.
  pub height: f32,
}

// Create viewports from rectangles.
impl From<Rect> for Viewport {
  fn from(rect: Rect) -> Self {
    Viewport {
      width: rect.w,
      height: rect.h,
    }
  }
}
