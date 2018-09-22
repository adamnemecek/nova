// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use std::cell::RefCell;

/// Global state of the engine.
pub struct Context<'a> {
  /// ECS world of the engine.
  pub(super) world: World,
  /// Window of the engine or `None` if it was not created from one.
  pub(crate) window: RefCell<Option<Window>>,
  /// State used during initial setup of the engine, then set to `None`.
  pub(super) init_state: Option<init::State<'a>>,
  /// Whether the engine will exit.
  pub(super) exiting: bool,
}

impl<'a> Context<'a> {
  pub fn new() -> Self {
    Context {
      world: World::new(),
      window: RefCell::new(None),
      init_state: Some(init::State::default()),
      exiting: false,
    }
  }
}

// Create an engine context from a window for an engine with graphics support.
impl<'a> From<Window> for Context<'a> {
  fn from(window: Window) -> Self {
    let context = Context::new();

    context.window.replace(Some(window));
    context
  }
}
