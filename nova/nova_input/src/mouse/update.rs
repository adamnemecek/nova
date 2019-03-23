// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::mouse::WriteMouse;
use nova_core::engine::{Engine, EnginePhase};
use nova_core::events::EventReaderId;
use nova_core::math::Point2;
use nova_core::systems::System;
use nova_window::{ButtonState, MouseButton, ReadWindow, WindowEvent};

#[derive(Debug)]
pub struct UpdateMouse {
  reader: EventReaderId<WindowEvent>,
}

impl<'a> System<'a> for UpdateMouse {
  type Data = (ReadWindow<'a>, WriteMouse<'a>);

  fn run(&mut self, (window, mut mouse): Self::Data) {
    for event in window.events.read(&mut self.reader) {
      match event {
        WindowEvent::CursorMoved { position, .. } => {
          mouse.set_position(Some(Point2::new(position.x as f32, position.y as f32)));
        }

        WindowEvent::CursorLeft { .. } => {
          mouse.set_position(None);
        }

        WindowEvent::MouseInput { button, state, .. } => {
          mouse.set_button(
            match button {
              MouseButton::Left => 0,
              MouseButton::Right => 1,
              MouseButton::Middle => 2,
              MouseButton::Other(i) => *i as usize,
            },
            *state == ButtonState::Pressed,
          );
        }

        _ => {}
      }
    }
  }
}

pub fn setup(engine: &mut Engine) {
  let reader = nova_window::borrow_mut(&engine.resources)
    .events
    .register_reader();

  engine.schedule(EnginePhase::BeforeUpdate, UpdateMouse { reader });
}
