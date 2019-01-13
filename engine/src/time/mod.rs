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
