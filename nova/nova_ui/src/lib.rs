// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod layout;
pub mod text;

mod container;
mod painter;
mod screen;
mod style;

pub use self::container::Container;
pub use self::painter::Painter;
pub use self::screen::Screen;
pub use self::style::Style;
pub use nova_graphics::Color4 as Color;

use nova_core::engine::Engine;
use nova_core::shred;

pub fn setup(engine: &mut Engine) {
  layout::setup(engine);
  screen::setup(engine);
  style::setup(engine);
  text::setup(engine);
}
