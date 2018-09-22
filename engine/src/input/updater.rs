// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Engine process that updates `Input` state.
pub struct Updater;

impl engine::Process for Updater {
  fn early_update(&mut self, ctx: &mut engine::Context) {
    if let Some(window) = ctx.window.borrow().as_ref() {
      let clock = engine::fetch_resource::<time::Clock>(ctx);
      let mapping = engine::fetch_resource::<Mapping>(ctx);
      let mut state = engine::fetch_resource_mut::<Input>(ctx);

      // Unset `repeated` flag on every button.
      for button in &mut state.buttons {
        button.repeated = false;
      }

      // Loop through window events.
      for event in window.events() {
        match event {
          engine::window::Event::KeyboardInput { input, .. } => {
            if let Some(key) = input.virtual_keycode {
              for button in mapping.get_buttons_for(key) {
                let button = &mut state.buttons[*button as usize];

                if input.state == engine::window::ElementState::Pressed {
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
}
