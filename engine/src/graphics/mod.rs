// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `graphics` module handles basic drawing and image data.
//!
//! The `Canvas` struct is built from a `platform::Window` and can be used to
//! draw grahpics with hardware acceleration. The canvas can draw `Image` assets
//! loaded from standard image files.
//!
//! The `Atlas` struct provides a wrapper around an `Image` asset that slices
//! it into cells. An atlas is also known as a sprite sheet.
//!
//! The `Sprite` component stores state for drawing a cell from an `Atlas` for
//! the entity it is attached to.

use super::*;
pub use ggez::graphics::{Color, DrawParam as DrawParams, Rect};

mod atlas;
mod canvas;
mod image;

pub use self::atlas::*;
pub use self::canvas::*;
pub use self::image::*;
