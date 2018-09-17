// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

pub mod sequence;
pub mod system;

pub use self::sequence::{Frame, Sequence};
pub use self::system::AnimationSystem;

#[derive(Component, Default)]
#[storage(BTreeStorage)]
pub struct Animation {
  pub sequence: Option<Arc<Sequence>>,
  pub elapsed: f64,
}
