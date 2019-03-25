// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{GamepadAxis, GamepadButton, WriteGamepad};
use gilrs::Gilrs;
use nova_core::engine::{Engine, EnginePhase};
use nova_core::log;
use nova_core::systems::System;
use uuid::Uuid;

#[derive(Debug)]
pub struct UpdateGamepad {
  /// ID of the gamepad currently used for input events.
  gamepad_id: Option<gilrs::GamepadId>,
  gilrs: Gilrs,
  log: log::Logger,
}

impl UpdateGamepad {
  fn select_initial(&mut self) {
    for (id, gamepad) in self.gilrs.gamepads() {
      self.log_connected(gamepad);

      if self.gamepad_id.is_none() {
        self.gamepad_id = Some(id);
      }
    }

    self.log_selected(self.gamepad_id.map(|id| self.gilrs.gamepad(id)));
  }

  fn log_connected(&self, gamepad: gilrs::Gamepad) {
    self
      .log
      .info("Gamepad connected.")
      .with("name", gamepad.name())
      .with("uuid", Uuid::from_bytes(gamepad.uuid()));
  }

  fn log_disconnected(&self, gamepad: gilrs::Gamepad) {
    self
      .log
      .info("Gamepad disconnected.")
      .with("name", gamepad.name())
      .with("uuid", Uuid::from_bytes(gamepad.uuid()));
  }

  fn log_selected(&self, gamepad: Option<gilrs::Gamepad>) {
    if let Some(gamepad) = gamepad {
      self
        .log
        .info("Gamepad selected.")
        .with("name", gamepad.name())
        .with("uuid", Uuid::from_bytes(gamepad.uuid()));
    } else {
      self.log.info("No gamepads available.");
    }
  }
}

impl<'a> System<'a> for UpdateGamepad {
  type Data = WriteGamepad<'a>;

  fn run(&mut self, mut gamepad: Self::Data) {
    while let Some(gilrs::Event { id, event, .. }) = self.gilrs.next_event() {
      match event {
        // Use the first gamepad that connects.
        gilrs::EventType::Connected => {
          let gamepad = self.gilrs.gamepad(id);

          self.log_connected(gamepad);

          if self.gamepad_id.is_none() {
            self.gamepad_id = Some(id);
            self.log_selected(Some(gamepad));
          }
        }

        // Use the next available gamepad when the current one is disconnected.
        gilrs::EventType::Disconnected => {
          self.log_disconnected(self.gilrs.gamepad(id));

          if self.gamepad_id == Some(id) {
            let gamepad = self.gilrs.gamepads().next();

            self.gamepad_id = gamepad.map(|g| g.0);
            self.log_selected(gamepad.map(|g| g.1));
          }
        }

        gilrs::EventType::AxisChanged(axis, value, _) if self.gamepad_id == Some(id) => {
          match axis {
            gilrs::Axis::DPadX => {
              if value > 0.0 {
                gamepad.set_button(GamepadButton::DPadLeft, 0.0);
                gamepad.set_button(GamepadButton::DPadRight, value);
              } else {
                gamepad.set_button(GamepadButton::DPadLeft, -value);
                gamepad.set_button(GamepadButton::DPadRight, 0.0);
              }
            }

            gilrs::Axis::DPadY => {
              if value > 0.0 {
                gamepad.set_button(GamepadButton::DPadDown, 0.0);
                gamepad.set_button(GamepadButton::DPadUp, value);
              } else {
                gamepad.set_button(GamepadButton::DPadDown, -value);
                gamepad.set_button(GamepadButton::DPadUp, 0.0);
              }
            }

            axis => {
              if let Some(axis) = GamepadAxis::from_gilrs(axis) {
                gamepad.set_axis(axis, value);
              }
            }
          };
        }

        gilrs::EventType::ButtonChanged(button, value, _) if self.gamepad_id == Some(id) => {
          if let Some(button) = GamepadButton::from_gilrs(button) {
            gamepad.set_button(button, value);
          }
        }

        gilrs::EventType::ButtonPressed(button, _) if self.gamepad_id == Some(id) => {
          if let Some(button) = GamepadButton::from_gilrs(button) {
            gamepad.set_button(button, 1.0);
          }
        }

        gilrs::EventType::ButtonReleased(button, _) if self.gamepad_id == Some(id) => {
          if let Some(button) = GamepadButton::from_gilrs(button) {
            gamepad.set_button(button, 0.0);
          }
        }

        _ => {}
      }
    }
  }
}

pub fn set_up(engine: &mut Engine) {
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

  // Initialize the system and schedule it.
  let mut update_gamepad = UpdateGamepad {
    gilrs,
    gamepad_id: None,
    log,
  };

  update_gamepad.select_initial();

  engine.schedule_seq(EnginePhase::BeforeUpdate, update_gamepad);
}
