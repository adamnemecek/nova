// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Updates the world's `Input` resource with input events from the given
/// window.
pub fn update(world: &mut World, window: &platform::Window) {
  let clock = world.read_resource::<time::Clock>();
  let mapping = world.read_resource::<Mapping>();
  let mut state = world.write_resource::<Input>();

  // Unset `repeated` flag on every button.
  for button in &mut state.buttons {
    button.repeated = false;
  }

  // Loop through window events.
  for event in window.events() {
    match event {
      platform::WindowEvent::KeyboardInput { input, .. } => {
        if let Some(key) = input.virtual_keycode {
          for button in mapping.get_buttons_for(key) {
            let button = &mut state.buttons[*button as usize];

            if input.state == platform::InputState::Pressed {
              // Set pressed time if the button was not already pressed.
              if button.pressed_at.is_none() {
                button.pressed_at = Some(clock.ticked_at);
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
