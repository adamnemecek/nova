// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::ecs;
use nova_core::engine::{Engine, EngineEvent};
use nova_core::math::{Matrix4, Size};
use nova_window::Window;

#[derive(Debug)]
pub struct Screen {
  size: Size<f32>,
  projection: Matrix4<f32>,
}

impl Screen {
  fn new() -> Self {
    Screen {
      size: Size::default(),
      projection: Matrix4::identity(),
    }
  }

  pub fn size(&self) -> Size<f32> {
    self.size
  }

  pub fn projection(&self) -> &Matrix4<f32> {
    &self.projection
  }

  fn set_pixel_size(&mut self, size: Size<u32>) {
    let pixels_per_unit = if size.height > size.width {
      (size.width / 1280).max(1) as f32
    } else {
      (size.height / 720).max(1) as f32
    };

    self.size = Size::<f32>::from(size) / pixels_per_unit;

    self.projection =
      Matrix4::new_orthographic(0.0, size.width as f32, 0.0, size.height as f32, -1.0, 1.0)
        .prepend_scaling(pixels_per_unit);
  }
}

#[derive(Debug)]
pub struct UpdateScreenInfo;

impl<'a> ecs::System<'a> for UpdateScreenInfo {
  type SystemData = (
    ecs::ReadResource<'a, Window>,
    ecs::WriteResource<'a, Screen>,
  );

  fn run(&mut self, (window, mut screen): Self::SystemData) {
    screen.set_pixel_size(window.size());
  }
}

pub fn setup(engine: &mut Engine) {
  engine.resources_mut().entry().or_insert_with(Screen::new);

  let mut update = UpdateScreenInfo;

  if engine.resources().has_value::<Window>() {
    ecs::System::run(&mut update, ecs::SystemData::fetch(engine.resources()));
  }

  engine.on_event(EngineEvent::TickStarted, update);
}
