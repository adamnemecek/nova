// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod rect;

pub use self::rect::ScreenRect;

use nova_core::components;
use nova_core::engine::{Engine, EnginePhase};
use nova_core::math::{Matrix4, Size};
use nova_core::resources::{ReadResource, WriteResource};
use nova_core::systems::System;
use nova_window::Window;

#[derive(Debug)]
pub struct Screen {
  size: Size<f32>,
  dpi: f32,
  projection: Matrix4<f32>,
}

impl Screen {
  fn new() -> Self {
    Screen {
      size: Size::default(),
      dpi: 1.0,
      projection: Matrix4::identity(),
    }
  }

  pub fn size(&self) -> Size<f32> {
    self.size
  }

  pub fn dpi(&self) -> f32 {
    self.dpi
  }

  pub fn projection(&self) -> &Matrix4<f32> {
    &self.projection
  }

  fn set_pixel_size(&mut self, size: Size<u32>) {
    self.dpi = if size.height > size.width {
      (size.width / 1280).max(1) as f32
    } else {
      (size.height / 720).max(1) as f32
    };

    self.size = Size::<f32>::from(size);

    self.projection =
      Matrix4::new_orthographic(0.0, size.width as f32, 0.0, size.height as f32, -1.0, 1.0);
  }
}

#[derive(Debug)]
struct UpdateScreenInfo;

impl<'a> System<'a> for UpdateScreenInfo {
  type Data = (ReadResource<'a, Window>, WriteResource<'a, Screen>);

  fn run(&mut self, (window, mut screen): Self::Data) {
    screen.set_pixel_size(window.size());
  }
}

pub fn set_up(engine: &mut Engine) {
  components::register::<ScreenRect>(&mut engine.resources);

  engine.resources.entry().or_insert_with(Screen::new);

  engine.schedule(EnginePhase::BeforeUpdate, UpdateScreenInfo);
}
