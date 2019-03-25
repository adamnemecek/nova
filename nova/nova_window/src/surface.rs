// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::resources::Resources;
use nova_graphics::surfaces::Surface;

pub struct WindowSurface {
  surface: Surface,
}

impl WindowSurface {
  pub fn new(res: &Resources, window: &winit::Window) {
    let surface = Surface::new(res, window);
  }
}
