// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod clock;
mod duration;
mod instant;
mod rate_limiter;
mod settings;
mod timer;

pub use self::clock::*;
pub use self::duration::*;
pub use self::instant::*;
pub use self::rate_limiter::*;
pub use self::settings::*;
pub use self::timer::*;

use super::EngineHandle;

pub async fn delay(engine: &EngineHandle, duration: Duration) {
  let mut timer = Timer::new(engine);

  await!(timer.until(duration));
}

pub fn delta_time(engine: &EngineHandle) -> Duration {
  engine.execute(|ctx| ctx.fetch_resource::<Clock>().delta_time)
}

pub(crate) fn setup(engine: &EngineHandle) {
  engine.execute_mut(|ctx| {
    ctx.ensure_resource::<Settings>();
    ctx.ensure_resource::<Clock>();
  });
}

/// Updates the [`Clock`] resource of the given engine with the current time.
///
/// The clock is updated using the settings in the [`Settings`] resource.
pub(crate) fn tick(engine: &EngineHandle) {
  engine.execute(|ctx| {
    let settings = ctx.fetch_resource::<Settings>();
    let mut clock = ctx.fetch_resource_mut::<Clock>();

    clock.tick(&settings);
  });
}
