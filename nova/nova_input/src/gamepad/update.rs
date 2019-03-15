// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Gamepad, GamepadAxis, GamepadButton, WriteGamepad};
use gilrs::Gilrs;
use nova_core::ecs;
use nova_core::engine::Resources;
use nova_core::log;

#[derive(Debug)]
pub struct UpdateGamepad {
  gilrs: Gilrs,
  gamepad_id: Option<gilrs::GamepadId>,
  log: log::Logger,
}

impl Default for UpdateGamepad {
  fn default() -> Self {
    let log = log::Logger::new(module_path!());

    // Initialize gilrs which manages gamepads.
    let gilrs = match Gilrs::new() {
      Ok(gilrs) => gilrs,

      Err(gilrs::Error::NotImplemented(gilrs)) => {
        log.warn("Cannot initialize gamepad input: gilrs is not supported on this platform.");

        // Gilrs still works when it isn't implemented, but no gamepads will
        // ever be connected.
        gilrs
      }

      Err(err) => panic!("Could not initialize gilrs: {}", err),
    };

    // Use the first available gamepad.
    let gamepad_id = gilrs.gamepads().next().map(|g| g.0);

    log
      .info("Gamepad changed.")
      .with("id", gamepad_id)
      .with("reason", log::Display("Initial"));

    Self {
      gilrs,
      gamepad_id,
      log,
    }
  }
}

impl UpdateGamepad {
  pub fn new() -> Self {
    Self::default()
  }
}

impl<'a> ecs::System<'a> for UpdateGamepad {
  type SystemData = WriteGamepad<'a>;

  fn setup(&mut self, res: &mut Resources) {
    res.entry().or_insert_with(Gamepad::default);
  }

  fn run(&mut self, mut gamepad: Self::SystemData) {
    while let Some(gilrs::Event { id, event, .. }) = self.gilrs.next_event() {
      match event {
        // Use the first gamepad that connects.
        gilrs::EventType::Connected if self.gamepad_id.is_none() => {
          self.gamepad_id = Some(id);

          self
            .log
            .info("Gamepad changed.")
            .with("id", self.gamepad_id)
            .with("reason", log::Display("Connected"));
        }

        // Use the next available gamepad when the current one is disconnected.
        gilrs::EventType::Disconnected if self.gamepad_id == Some(id) => {
          self.gamepad_id = self.gilrs.gamepads().next().map(|g| g.0);

          self
            .log
            .info("Gamepad changed.")
            .with("id", self.gamepad_id)
            .with("reason", log::Display("Disconnected"));
        }

        gilrs::EventType::AxisChanged(axis, state, _) if self.gamepad_id == Some(id) => {
          if let Some(axis) = GamepadAxis::from_gilrs(axis) {
            gamepad.axes.insert(axis, state);
          }
        }

        gilrs::EventType::ButtonChanged(button, state, _) if self.gamepad_id == Some(id) => {
          if let Some(button) = GamepadButton::from_gilrs(button) {
            gamepad.buttons.insert(button, state);
          }
        }

        gilrs::EventType::ButtonPressed(button, _) if self.gamepad_id == Some(id) => {
          if let gilrs::Button::DPadDown
          | gilrs::Button::DPadLeft
          | gilrs::Button::DPadRight
          | gilrs::Button::DPadUp = button
          {
            gamepad
              .buttons
              .insert(GamepadButton::from_gilrs(button).unwrap(), 1.0);
          }
        }

        gilrs::EventType::ButtonReleased(button, _) if self.gamepad_id == Some(id) => {
          if let gilrs::Button::DPadDown
          | gilrs::Button::DPadLeft
          | gilrs::Button::DPadRight
          | gilrs::Button::DPadUp = button
          {
            gamepad
              .buttons
              .insert(GamepadButton::from_gilrs(button).unwrap(), 0.0);
          }
        }

        _ => {}
      }
    }
  }
}
