// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::math::Size;

#[derive(Debug, PartialEq, Clone)]
pub struct Settings {
  pub title: String,
  pub resizable: bool,
  pub fullscreen: bool,
  pub size: Size<u32>,
}

impl Settings {
  pub fn set_title(&mut self, title: &str) {
    self.title.replace_range(.., title)
  }
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      title: "Nova".into(),
      resizable: true,
      fullscreen: false,
      size: Size::new(640, 360),
    }
  }
}
