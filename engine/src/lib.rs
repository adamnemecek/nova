// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate ggez;
pub extern crate nalgebra;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
pub extern crate specs;
#[macro_use]
extern crate specs_derive;

pub mod core;
pub mod graphics;
pub mod input;
pub mod stage;
pub mod unstable;

pub use core::Core;

pub mod prelude {
  pub(crate) use ggez;
  pub use nalgebra::{self, Point2, Point3, Vector2, Vector3};
  pub use specs::prelude::*;
  pub use specs::storage::BTreeStorage;

  pub use {core, graphics, input, stage, unstable, Core};
}
