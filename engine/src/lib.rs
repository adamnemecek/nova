// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate crossbeam_channel;
extern crate ggez;
extern crate image;
pub extern crate nalgebra;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
pub extern crate specs;
#[macro_use]
extern crate specs_derive;

pub mod assets;
pub mod engine;
pub mod graphics;
pub mod input;
pub mod stage;
pub mod time;

mod application;

pub use self::application::*;

pub mod prelude {
  pub use nalgebra::{self, Matrix4, Point2, Point3, Vector2, Vector3};
  pub use specs::prelude::*;
  pub use specs::storage::BTreeStorage;
  pub use std::sync::{Arc, Mutex, RwLock};

  pub use {assets, engine, graphics, input, stage, time, Application};
}

use prelude::*;
