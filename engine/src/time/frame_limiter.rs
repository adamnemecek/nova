// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant};
use crate::log;
use crate::Engine;
use std::thread;

/// Limits how frequently frames can be completed.
///
/// Frame limiting is useful in FIFO present mode to ensure a steady production
/// of frames matching the monitor refresh rate. In MAILBOX present mode there
/// is no visual benefit, but the player might appreciate not using an excessive
/// amount of CPU.
#[derive(Debug)]
pub struct FrameLimiter {
  /// Target frame duration.
  interval: Duration,
  /// How long to sleep before switching to yields.
  sleep_threshold: Duration,
  /// How long to yield before finishing.
  yield_threshold: Duration,
  /// Last time the `begin_frame()` function was called.
  frame_began: Instant,
  /// Logger to use for writing long frame warnings.
  log: log::Logger,
}

impl FrameLimiter {
  /// Creates a new frame limiter with the given target FPS.
  pub fn new(engine: &Engine, target_fps: f64) -> Self {
    let mut limiter = FrameLimiter {
      interval: Duration::ONE_SEC,
      sleep_threshold: Duration::ONE_SEC,
      yield_threshold: Duration::ONE_SEC,
      frame_began: Instant::now(),
      log: log::fetch_logger(engine).with_source("time::RateLimiter"),
    };

    limiter.set_target_fps(target_fps);

    limiter
  }

  /// Sets the target FPS of the frame limiter.
  pub fn set_target_fps(&mut self, target_fps: f64) {
    assert!(target_fps.is_finite(), "Target FPS must be finite.");
    assert!(target_fps > 0.0, "Target FPS must be positive.");

    assert!(
      target_fps <= 244.0,
      "Target FPS must be less than or equal to 244.",
    );

    let interval = Duration::from_secs(target_fps.recip());

    self.interval = interval;
    self.sleep_threshold = interval - Duration::ONE_MILLI * 2.0;
    self.yield_threshold = interval - Duration::ONE_MICRO * 250.0;
  }

  /// Marks the beginning of the frame and starts the timer.
  pub fn begin_frame(&mut self) {
    self.frame_began = Instant::now();
  }

  /// Marks the end of the frame and stops the timer.
  ///
  /// If the recorded time is less than the configured interval, this function
  /// will block until the entire interval has elapsed.
  pub fn end_frame(&mut self) {
    let mut now = Instant::now();
    let mut delta = now - self.frame_began;

    while delta < self.sleep_threshold {
      thread::sleep(std::time::Duration::from_millis(1));

      now = Instant::now();
      delta = now - self.frame_began;
    }

    while delta < self.yield_threshold {
      thread::yield_now();

      now = Instant::now();
      delta = now - self.frame_began;
    }

    if delta > self.interval {
      self.log.warn("Long frame.").with("duration", delta);
    }
  }
}
