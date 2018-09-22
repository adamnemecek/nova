// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Trait describing a _process_ that is updated during each iteration of an
/// engine loop.
pub trait Process {
  /// Invoked after early systems are dispatched.
  fn early_update(&mut self, _ctx: &mut Context) {}
  /// Invoked after systems are dispatched.
  fn update(&mut self, _ctx: &mut Context) {}
  /// Invoked after late systems are dispatched.
  fn late_update(&mut self, _ctx: &mut Context) {}
}
