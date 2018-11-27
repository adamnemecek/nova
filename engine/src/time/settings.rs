// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Duration;

/// A resource that stores the time settings for an engine instance.
pub struct Settings {
  /// The minimum duration of time that must elapse with each clock update.
  ///
  /// Set this to a short duration to prevent very short delta times which could
  /// introduce imprecision when used to scale larger floating point values.
  ///
  /// Set the minimum and maximum delta time to the same duration for a fixed
  /// time step when determinism is important.
  pub min_delta_time: Duration,
  /// The maximum duration of time that can elapse with each clock update.
  ///
  /// Set this to a short duration to prevent large delta times from breaking
  /// logic that expects smaller increments. For example, a game designed to run
  /// at 60 Hz might behave in unexpected ways if the player's computer lags and
  /// the game experiences a sudden one-second jump in time.
  ///
  /// Set the minimum and maximum delta time to the same duration for a fixed
  /// time step when determinism is important.
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
