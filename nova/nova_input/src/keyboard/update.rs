// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::WriteKeyboard;
use nova_core::engine::{Engine, EnginePhase};
use nova_core::events;
use nova_core::systems::System;
use nova_window::events::{ButtonState, ReadWindowEvents, WindowEvent};

#[derive(Debug)]
pub struct UpdateKeyboard {
  reader: events::ReaderId<WindowEvent>,
}

impl<'a> System<'a> for UpdateKeyboard {
  type Data = (ReadWindowEvents<'a>, WriteKeyboard<'a>);

  fn run(&mut self, (window_events, mut keyboard): Self::Data) {
    for event in window_events.channel().read(&mut self.reader) {
      let input = match event {
        WindowEvent::KeyboardInput { input, .. } => input,
        _ => continue,
      };

      let key_code = match input.virtual_keycode {
        Some(key_code) => key_code,
        None => continue,
      };

      let state = input.state == ButtonState::Pressed;

      keyboard.set_key(key_code, state);
    }
  }
}

pub fn setup(engine: &mut Engine) {
  let reader = nova_window::events::borrow_mut(&engine.resources)
    .channel_mut()
    .register_reader();

  engine.schedule(EnginePhase::BeforeUpdate, UpdateKeyboard { reader });
}
