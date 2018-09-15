// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::prelude::*;

use prelude::*;

use super::{Button, State};

#[derive(Default)]
pub struct Updater;

impl<'a> System<'a> for Updater {
  type SystemData = (
    Read<'a, core::Clock>,
    Read<'a, core::keyboard::Events>,
    Write<'a, State>,
  );

  fn run(&mut self, (clock, events, mut state): Self::SystemData) {
    for button in &mut state.buttons {
      button.repeated = false;
    }

    for event in &events.list {
      match event {
        core::keyboard::Event::Pressed(key) => {
          if let Some(button) = Button::from_keycode(key) {
            let button = &mut state.buttons[button as usize];

            if button.pressed_time.is_none() {
              button.pressed_time = Some(clock.time);
            }

            button.repeated = true;
          }
        }

        core::keyboard::Event::Released(key) => {
          if let Some(button) = Button::from_keycode(key) {
            let button = &mut state.buttons[button as usize];

            button.pressed_time = None;
            button.repeated = false;
          }
        }
      }
    }

    if events.list.len() > 0 {
      println!("{:?}", *state);
    }
  }
}
