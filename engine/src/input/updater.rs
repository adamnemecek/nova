// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::prelude::*;

use prelude::*;

use super::{Button, State};

/// System that updates input state.
#[derive(Default)]
pub struct Updater;

impl<'a> System<'a> for Updater {
  type SystemData = (
    Read<'a, core::Clock>,
    Read<'a, core::keyboard::Events>,
    Write<'a, State>,
  );

  fn run(&mut self, (clock, events, mut state): Self::SystemData) {
    // Unset `repeated` flag for every button.
    for button in &mut state.buttons {
      button.repeated = false;
    }

    for event in &events.list {
      match event {
        // When a button is pressed, update pressed time and set repeated.
        core::keyboard::Event::Pressed(key) => {
          if let Some(button) = Button::from_keycode(key) {
            let button = &mut state.buttons[button as usize];

            // Set pressed time if the button was not already pressed.
            if button.pressed_time.is_none() {
              button.pressed_time = Some(clock.time);
            }

            button.repeated = true;
          }
        }

        // When a button is released, unset pressed time and repeated flag.
        core::keyboard::Event::Released(key) => {
          if let Some(button) = Button::from_keycode(key) {
            let button = &mut state.buttons[button as usize];

            button.pressed_time = None;
            button.repeated = false;
          }
        }
      }
    }
  }
}
