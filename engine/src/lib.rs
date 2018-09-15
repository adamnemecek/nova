// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate ggez;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate specs;

pub mod game;
pub mod graphics;
pub mod motion;
pub mod platform;
pub mod time;

pub use prelude::*;

pub mod prelude {
  pub(crate) use ggez;
  pub use specs::{Component, Entity, World};

  pub use game::Game;
  pub use {game, graphics, motion, platform, time};
}
