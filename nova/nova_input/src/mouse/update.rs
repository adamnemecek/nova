// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::WriteMouse;
use nova_core::ecs;
use nova_core::engine::{Engine, EnginePhase};
use nova_core::math::Point2;

#[derive(Debug)]
pub struct UpdateMouse {
  reader: nova_window::EventReader,
}

impl<'a> ecs::System<'a> for UpdateMouse {
  type SystemData = (ecs::ReadResource<'a, nova_window::Events>, WriteMouse<'a>);

  fn run(&mut self, (window_events, mut mouse): Self::SystemData) {
    for event in window_events.channel().read(&mut self.reader) {
      match event {
        nova_window::Event::CursorMoved { position, .. } => {
          mouse.set_position(Some(Point2::new(position.x as f32, position.y as f32)));
        }

        nova_window::Event::CursorLeft { .. } => {
          mouse.set_position(None);
        }

        nova_window::Event::MouseInput { button, state, .. } => {
          mouse.set_button(
            match button {
              nova_window::MouseButton::Left => 0,
              nova_window::MouseButton::Right => 1,
              nova_window::MouseButton::Middle => 2,
              nova_window::MouseButton::Other(i) => *i as usize,
            },
            *state == nova_window::ButtonState::Pressed,
          );
        }

        _ => {}
      }
    }
  }
}

pub fn setup(engine: &mut Engine) {
  let reader = nova_window::write_events(&engine.resources)
    .channel_mut()
    .register_reader();

  engine.schedule(EnginePhase::BeforeUpdate, UpdateMouse { reader });
}
