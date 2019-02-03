// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(feature = "window")]
use crate::window;

pub struct Options {
  #[cfg(feature = "graphics")]
  pub graphics: bool,
  #[cfg(feature = "window")]
  pub window: Option<window::Options>,
}

impl Default for Options {
  fn default() -> Self {
    Options {
      #[cfg(feature = "graphics")]
      graphics: true,
      #[cfg(feature = "window")]
      window: Some(Default::default()),
    }
  }
}
