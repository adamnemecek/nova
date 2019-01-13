// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;
use std::task::Wake;

pub(super) struct AtomicWake(AtomicBool);

impl AtomicWake {
  pub fn new() -> Self {
    AtomicWake(AtomicBool::new(true))
  }

  pub fn reset(&self) {
    self.0.store(false, atomic::Ordering::Relaxed);
  }

  pub fn is_awake(&self) -> bool {
    self.0.load(atomic::Ordering::Relaxed)
  }
}

impl Wake for AtomicWake {
  fn wake(arc_self: &Arc<Self>) {
    arc_self.0.store(true, atomic::Ordering::Relaxed);
  }
}
