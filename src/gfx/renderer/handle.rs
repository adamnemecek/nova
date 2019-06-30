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

  /// Stops the renderer.
  pub fn stop(&self) {
    let _ = self.messages.send(ControlMessage::Stop);
  }
}

/// A message sent by another thread to control the renderer.
pub(super) enum ControlMessage {
  /// Stops the render loop.
  Stop,
}
