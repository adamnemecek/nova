// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::controls::{ControlBinding, WriteControls};
use crate::gamepad::{Gamepad, GamepadEvent, ReadGamepad};
use crate::keyboard::{Keyboard, KeyboardEvent, ReadKeyboard};
use crate::mouse::{Mouse, MouseEvent, ReadMouse};
use nova_core::engine::{Engine, EnginePhase};
use nova_core::events::EventReaderId;
use nova_core::systems::System;

#[derive(Debug)]
pub struct UpdateControls {
  keyboard_reader: EventReaderId<KeyboardEvent>,
  mouse_reader: EventReaderId<MouseEvent>,
  gamepad_reader: EventReaderId<GamepadEvent>,
}

impl<'a> System<'a> for UpdateControls {
  type Data = (
    ReadKeyboard<'a>,
    ReadGamepad<'a>,
    ReadMouse<'a>,
    WriteControls<'a>,
  );

  fn run(&mut self, (keyboard, gamepad, mouse, mut controls): Self::Data) {
    for event in keyboard.events.read(&mut self.keyboard_reader) {
      let KeyboardEvent::KeyChanged { key, value } = event;

      controls.set_bound_values(ControlBinding::Key(*key), if *value { 1.0 } else { 0.0 });
    }

    for event in gamepad.events.read(&mut self.gamepad_reader) {
      match event {
        GamepadEvent::ButtonChanged { button, value } => {
          controls.set_bound_values(ControlBinding::GamepadButton(*button), *value);
        }

        GamepadEvent::AxisChanged { axis, value } => {
          controls.set_bound_values(ControlBinding::GamepadAxis(*axis), *value);
        }
      }
    }

    for event in mouse.events.read(&mut self.mouse_reader) {
      if let MouseEvent::ButtonChanged { button, value } = event {
        controls.set_bound_values(
          ControlBinding::MouseButton(*button),
          if *value { 1.0 } else { 0.0 },
        );
      }
    }
  }
}

pub fn set_up(engine: &mut Engine) {
  let keyboard_reader = Keyboard::borrow_mut(&engine.resources)
    .events
    .register_reader();

  let mouse_reader = Mouse::borrow_mut(&engine.resources)
    .events
    .register_reader();

  let gamepad_reader = Gamepad::borrow_mut(&engine.resources)
    .events
    .register_reader();

  engine.schedule(
    EnginePhase::BeforeUpdate,
    UpdateControls {
      keyboard_reader,
      mouse_reader,
      gamepad_reader,
    },
  );
}
