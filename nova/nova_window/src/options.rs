// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::math::Size;

#[derive(Clone)]
pub struct WindowOptions {
  pub title: String,
  pub size: Size<u32>,
}

impl WindowOptions {
  pub fn new() -> Self {
    Self {
      title: String::new(),
      size: Size::new(2560, 1440),
    }
  }

  pub fn set_title(&mut self, title: &str) {
    self.title.replace_range(.., title);
  }
}

impl Default for WindowOptions {
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
