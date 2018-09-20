// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `platform` module handles low-level device setup.
//!
//! The `Window` struct creates a window for the current platform and sets it up
//! for hardware-accelerated drawing with `graphics::Canvas`. Each frame, the
//! window should be updated to process events which can be used to resize the
//! canvas and update the `input` module.

use super::*;
pub use ggez::event::winit_event::{ElementState as InputState, KeyboardInput, WindowEvent};

mod window;

pub use self::window::*;
