// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Duration, Instant};
use std::collections::VecDeque;

const AVG_WINDOW: usize = 100;

/// Calculates time statistics for each frame and the average over many frames.
///
/// This structure can be updated each frame to calculate:
/// - Time to complete each frame (the “frame time”)
/// - Time between each frame (the “delta time”)
/// - Averages of both times
/// - Average frames per second
#[derive(Default, Debug)]
pub struct FrameTimer {
  /// Instant when the current frame began.
  frame_began: Option<Instant>,
  /// Time info for the current frame.
  frame: Frame,
  /// Time info for up to [`AVG_WINDOW`] previous frames.
  frames: VecDeque<Frame>,
  /// Time info for the average frame.
  avg_frame: Frame,
  /// Average frames per second. This is the reciprocal of the average delta
  /// time.
  avg_fps: f64,
}

/// Time info for a single frame.
#[derive(Default, Clone, Copy, Debug)]
struct Frame {
  /// Duration of time between the beginning and end of this frame.
  pub frame_time: Duration,
  /// Duration of time between the beginning of the previous frame and the
  /// beginning of this frame.
  pub delta_time: Duration,
}

impl FrameTimer {
  /// Creates a new frame timer with initially empty stats.
  pub fn new() -> Self {
    FrameTimer::default()
  }

  /// Gets the duration of time between the beginning and end of the most recent
  /// frame.
  ///
  /// After [`begin_frame()`] is called, this duration is zero until
  /// [`end_frame()`] is called.
  pub fn frame_time(&self) -> Duration {
    self.frame.frame_time
  }

  /// Gets the duration of time between beginning of the previous frame and the
  /// beginning of the current frame.
  ///
  /// This duration is zero until [`begin_frame()`] has been called at least
  /// twice.
  pub fn delta_time(&self) -> Duration {
    self.frame.delta_time
  }

  /// Gets a simple moving average of the duration between the beginning and end
  /// of each frame.
  pub fn avg_frame_time(&self) -> Duration {
    self.avg_frame.frame_time
  }

  /// Gets a simple moving average of the duration between the beginnings of
  /// each frame.
  pub fn avg_delta_time(&self) -> Duration {
    self.avg_frame.delta_time
  }

  /// Gets a simple moving average of the frames per second.
  ///
  /// This is the reciprocal of [`avg_delta_time()`].
  pub fn avg_fps(&self) -> f64 {
    self.avg_fps
  }

  /// Marks the beginning of a frame.
  ///
  /// After the first frame, this function updates [`delta_time()`] and the
  /// average frame statistics.
  pub fn begin_frame(&mut self) {
    let began = Instant::now();
    let prev = self.frame_began;

    self.frame_began = Some(began);

    if let Some(prev) = prev {
      let delta = began - prev;

      self.frame.delta_time = delta;

      // Add the frame to the average window.
      self.frames.push_front(self.frame);

      self.avg_frame = if self.frames.len() <= AVG_WINDOW {
        // Manually average times each frame until the window is full.
        let mut total = Frame::default();

        for f in &self.frames {
          total.frame_time += f.frame_time;
          total.delta_time += f.delta_time;
        }

        let len = self.frames.len() as f64;

        Frame {
          frame_time: total.frame_time / len,
          delta_time: total.delta_time / len,
        }
      } else {
        // When the window is full, calculate a moving average by removing the
        // oldest values and adding the newest with each frame.
        let old = self.frames.pop_back().unwrap();
        let len = self.frames.len() as f64;

        Frame {
          frame_time: self.avg_frame.frame_time + self.frame.frame_time / len
            - old.frame_time / len,
          delta_time: self.avg_frame.delta_time + self.frame.delta_time / len
            - old.delta_time / len,
        }
      };

      self.avg_fps = self.avg_frame.delta_time.as_secs().recip();
    }

    self.frame.frame_time = Duration::default();
  }

  /// Marks the end of a frame.
  ///
  /// This function updates [`frame_time()`].
  pub fn end_frame(&mut self) {
    let began = self
      .frame_began
      .expect("The `end_frame()` function was called before `begin_frame()`.");

    self.frame.frame_time = Instant::now() - began;
  }
}
