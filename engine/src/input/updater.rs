// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Input, Mapping};
use crate::prelude::*;
use crate::time::Clock;
use crate::window::events::{ElementState, WindowEvent};
use crate::window::Window;

/// Engine process that updates `Input` state.
pub struct Updater;

impl<'a> System<'a> for Updater {
  type SystemData = (
    ReadResource<'a, Window>,
    ReadResource<'a, Clock>,
    ReadResource<'a, Mapping>,
    WriteResource<'a, Input>,
  );

  fn run(&mut self, (window, clock, mapping, mut state): Self::SystemData) {
    // Unset `repeated` flag on every button.
    for button in &mut state.buttons {
      button.repeated = false;
    }

    // Loop through window events.
    for event in window.events() {
      match event {
        WindowEvent::KeyboardInput { input, .. } => {
          if let Some(key) = input.virtual_keycode {
            for button in mapping.get_buttons_for(key) {
              let button = &mut state.buttons[*button as usize];

              if input.state == ElementState::Pressed {
                // Set pressed time if the button was not already pressed.
                if button.pressed_at.is_none() {
                  button.pressed_at = Some(clock.time);
                }

                button.repeated = true;
              } else {
                button.pressed_at = None;
                button.repeated = false;
              }
            }
          }
        }

        _ => {}
      }
    }
  }
}
