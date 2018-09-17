// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// System that updates the `Input` resource from core input events.
#[derive(Default)]
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
  type SystemData = (
    Read<'a, core::Clock>,
    Read<'a, core::input::KeyEvents>,
    Write<'a, Input>,
  );

  fn run(&mut self, (clock, events, mut input): Self::SystemData) {
    // Unset `repeated` flag on every button.
    for button in &mut input.buttons {
      button.repeated = false;
    }

    for event in &events.list {
      match event {
        // When a button is pressed, update its pressed time and set repeated.
        core::input::KeyEvent::Pressed(key) => {
          if let Some(button) = Button::from_keycode(key) {
            let button = &mut input.buttons[button as usize];

            // Set pressed time if the button was not already pressed.
            if button.pressed_time.is_none() {
              button.pressed_time = Some(clock.time);
            }

            button.repeated = true;
          }
        }

        // When a button is released, unset its pressed time and repeated flag.
        core::input::KeyEvent::Released(key) => {
          if let Some(button) = Button::from_keycode(key) {
            let button = &mut input.buttons[button as usize];

            button.pressed_time = None;
            button.repeated = false;
          }
        }
      }
    }
  }
}
