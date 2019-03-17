// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Mouse, WriteMouse};
use nova_core::ecs;
use nova_core::math::Point2;

#[derive(Debug, Default)]
pub struct UpdateMouse {
  reader: Option<nova_window::EventReader>,
}

impl UpdateMouse {
  pub fn new() -> Self {
    Self::default()
  }
}

impl<'a> ecs::System<'a> for UpdateMouse {
  type SystemData = (ecs::ReadResource<'a, nova_window::Events>, WriteMouse<'a>);

  fn setup(&mut self, res: &mut ecs::Resources) {
    res.entry().or_insert_with(Mouse::default);

    self.reader = Some(
      nova_window::write_events(res)
        .channel_mut()
        .register_reader(),
    );
  }

  fn run(&mut self, (window_events, mut mouse): Self::SystemData) {
    let reader = match self.reader.as_mut() {
      Some(reader) => reader,
      None => return,
    };

    for event in window_events.channel().read(reader) {
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
