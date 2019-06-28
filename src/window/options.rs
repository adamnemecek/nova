// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Options for opening a new window.
#[derive(Clone)]
pub struct Options {
  /// Sets the title of the window.
  ///
  /// Defaults to the name of the executable.
  pub title: String,

  /// Sets whether the window is resizable.
  ///
  /// Defaults to `false`.
  pub resizable: bool,

  /// Sets the window size in pixels.
  ///
  /// Defaults to `(1280, 720)`.
  pub size: Option<Size<f64>>,
}

impl Options {
  pub fn new() -> Self {
    Self {
      title: String::new(),
      resizable: false,
      size: None,
    }
  }

  pub fn set_title(&mut self, title: &str) {
    self.title.replace_range(.., title);
  }
}

impl Default for Options {
  fn default() -> Self {
    let mut options = Self::new();

    if let Ok(exe) = std::env::current_exe() {
      if let Some(stem) = exe.file_stem() {
        options.set_title(&stem.to_string_lossy());
      }
    }

    options
  }
}
