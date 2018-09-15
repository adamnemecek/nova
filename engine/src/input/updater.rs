// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use specs::prelude::*;

use prelude::*;

use super::{Button, Input};

#[derive(Default)]
pub struct InputUpdater;

impl<'a> System<'a> for InputUpdater {
  type SystemData = (
    Read<'a, core::Clock>,
    Read<'a, core::keyboard::Events>,
    Write<'a, Input>,
  );

  fn run(&mut self, (clock, events, mut input): Self::SystemData) {
    for state in &mut input.buttons {
      state.repeated = false;
    }

    for event in &events.list {
      match event {
        core::keyboard::Event::Pressed(key) => {
          if let Some(button) = Button::from_keycode(key) {
            let state = &mut input.buttons[button as usize];

            if state.pressed_time.is_some() {
              state.repeated = true;
            } else {
              state.pressed_time = Some(clock.time);
            }
          }
        }

        core::keyboard::Event::Released(key) => {
          if let Some(button) = Button::from_keycode(key) {
            let state = &mut input.buttons[button as usize];

            state.pressed_time = None;
            state.repeated = false;
          }
        }
      }
    }

    if events.list.len() > 0 {
      println!("{:?}", *input);
    }
  }
}
