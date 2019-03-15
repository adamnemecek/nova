// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Keyboard, WriteKeyboard};
use nova_core::ecs;
use nova_core::engine::Resources;

#[derive(Debug, Default)]
pub struct UpdateKeyboard {
  reader: Option<nova_window::EventReader>,
}

impl UpdateKeyboard {
  pub fn new() -> Self {
    Self::default()
  }
}

impl<'a> ecs::System<'a> for UpdateKeyboard {
  type SystemData = (
    ecs::ReadResource<'a, nova_window::Events>,
    WriteKeyboard<'a>,
  );

  fn setup(&mut self, res: &mut Resources) {
    res.entry().or_insert_with(Keyboard::default);

    self.reader = Some(
      nova_window::write_events(res)
        .channel_mut()
        .register_reader(),
    );
  }

  fn run(&mut self, (window_events, mut keyboard): Self::SystemData) {
    let reader = match self.reader.as_mut() {
      Some(reader) => reader,
      None => return,
    };

    for event in window_events.channel().read(reader) {
      let input = match event {
        nova_window::Event::KeyboardInput { input, .. } => input,
        _ => continue,
      };

      let key_code = match input.virtual_keycode {
        Some(key_code) => key_code,
        None => continue,
      };

      let state = input.state == nova_window::KeyState::Pressed;

      keyboard.set_key(key_code, state);
    }
  }
}
