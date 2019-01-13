// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Duration;
use crate::process;
use crate::EngineHandle;

pub struct Timer {
  engine: EngineHandle,
  elapsed: Duration,
}

impl Timer {
  pub fn new(engine: &EngineHandle) -> Timer {
    Timer {
      engine: engine.clone(),
      elapsed: Duration::ZERO,
    }
  }

  pub async fn until(&mut self, duration: Duration) {
    let mut elapsed = self.elapsed;

    while elapsed < duration {
      await!(process::next_tick());

      self.engine.execute(|ctx| {
        elapsed += super::delta(ctx);
      })
    }

    self.elapsed = elapsed - duration;
  }
}
