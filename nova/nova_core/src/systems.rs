// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod derive {
  pub use shred_derive::*;
}

pub use specs::shred::{DynamicSystemData, SystemData};

use std::fmt;

/// A system that runs on data borrowed from a set of shared resources.
pub trait System<'a>: fmt::Debug {
  type Data: SystemData<'a>;

  fn run(&mut self, data: Self::Data);
}
