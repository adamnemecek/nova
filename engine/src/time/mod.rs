// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The `time` module provides shared time state.
//!
//! The `Clock` resource stores time information and must be updated once per
//! frame with the `tick` function.

use super::*;

pub use std::time::{Duration, Instant};

mod clock;
mod updater;

pub use self::clock::*;
pub use self::updater::*;

/// Initializes time in the given engine context.
pub fn init(ctx: &mut engine::Context) {
  engine::add_resource(ctx, Clock::default());
  engine::add_system_early(ctx, Updater::default(), "time::Updater", &[]);
}
