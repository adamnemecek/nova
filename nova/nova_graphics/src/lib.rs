// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod images;

mod color;

pub use self::color::Color4;

// TODO: Remove after creating a custom `SystemData` derive macro.
use nova_core::shred;

use nova_core::engine::Engine;

pub fn setup(engine: &mut Engine) {
  images::setup(engine);
}
