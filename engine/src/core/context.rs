// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ggez::conf::{WindowMode, WindowSetup};

use prelude::*;

/// Create a `ggez::ContextBuilder` that can be passed to `Core::new`.
pub fn build(game: &'static str, author: &'static str) -> ggez::ContextBuilder {
  let builder = ggez::ContextBuilder::new(game, author)
    // Create a resizable window with vsync disabled.
    .window_mode(WindowMode::default().resizable(true))
    .window_setup(WindowSetup::default().title(game).vsync(false));

  builder
}
