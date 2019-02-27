// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod container;
pub mod layout;
mod painter;
mod screen;
mod style;

pub use self::container::Container;
pub use self::layout::Layout;
pub use self::painter::Painter;
pub use self::screen::Screen;
pub use self::style::Style;
pub use nova_graphics::Color4 as Color;

use nova_core::Engine;

pub fn setup(engine: &mut Engine) {
  layout::setup(engine);
  screen::setup(engine);
  style::setup(engine);
}
