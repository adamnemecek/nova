// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A handle for a renderer running on a separate thread.
pub struct Handle {
  messages: channel::Sender<ControlMessage>,
}

impl Handle {
  /// Creates a new handle using the given message channel.
  pub(super) fn new(messages: channel::Sender<ControlMessage>) -> Self {
    Self { messages }
  }

  /// Sets the target FPS of the renderer.
  ///
  /// This is a soft limit on frame rate, meaning the render will try not to
  /// render more frames than this per second.
  pub fn set_target_fps(&self, fps: f64) {
    let _ = self.messages.send(ControlMessage::SetTargetFPS(fps));
  }

  /// Stops the renderer.
  pub fn stop(&self) {
    let _ = self.messages.send(ControlMessage::Stop);
  }
}

/// A message sent by another thread to control the renderer.
pub(super) enum ControlMessage {
  /// Sets the target FPS (the frequency of the renderer's clock).
  SetTargetFPS(f64),
  /// Stops the render loop.
  Stop,
}
