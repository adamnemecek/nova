// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Duration;

pub struct Settings {
  pub min_delta_time: Duration,
  pub max_delta_time: Duration,
}

impl Default for Settings {
  fn default() -> Self {
    Settings {
      min_delta_time: Duration::ZERO,
      max_delta_time: Duration::ONE_SEC,
    }
  }
}
