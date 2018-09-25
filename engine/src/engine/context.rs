// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::init;
use super::WindowHandle;
use std::cell::RefCell;

/// Global state of the engine.
pub struct Context<'a> {
  /// Specs world of the engine.
  pub(super) world: specs::World,
  /// State used during initial setup of the engine, then set to `None`.
  pub(super) init_state: Option<init::State<'a>>,
  /// Handle to the window created with `window::create_window`.
  pub(crate) window_handle: RefCell<Option<WindowHandle>>,
  /// Whether the engine will exit.
  pub(super) exiting: bool,
}

impl<'a> Context<'a> {
  pub fn new() -> Self {
    Context {
      world: specs::World::new(),
      window_handle: RefCell::new(None),
      init_state: Some(init::State::default()),
      exiting: false,
    }
  }
}
