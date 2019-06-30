// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

pub enum ControlMessage {
  SetTargetFPS(f64),
  Stop,
}

pub struct Handle {
  messages: channel::Sender<ControlMessage>,
}

impl Handle {
  pub(crate) fn new(messages: channel::Sender<ControlMessage>) -> Self {
    Self { messages }
  }

  pub fn set_target_fps(&self, fps: f64) {
    let _ = self.messages.send(ControlMessage::SetTargetFPS(fps));
  }

  pub fn stop(&self) {
    let _ = self.messages.send(ControlMessage::Stop);
  }
}
