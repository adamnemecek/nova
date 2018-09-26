// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use winit::{EventsLoop, WindowBuilder};

pub mod events;

pub use self::events::WindowEvent;

pub struct Window {
  inner: winit::Window,
  events: Vec<WindowEvent>,
  size: Vector2<f32>,
}

impl Window {
  pub fn events(&self) -> &[WindowEvent] {
    &self.events
  }

  pub fn as_winit(&self) -> &winit::Window {
    &self.inner
  }

  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  pub fn was_resized(&self) -> bool {
    self.events.iter().any(|ev| match ev {
      WindowEvent::Resized(_) => true,
      _ => false,
    })
  }
}

impl Default for Window {
  fn default() -> Self {
    unimplemented!();
  }
}

struct Extension {
  events_loop: winit::EventsLoop,
}

impl engine::Extension for Extension {
  fn before_systems(&mut self, ctx: &mut engine::Context) {
    let mut exiting = false;

    {
      let mut window = engine::fetch_resource_mut::<Window>(ctx);

      window.events.clear();

      self.events_loop.poll_events(|event| match event {
        winit::Event::WindowEvent { event, .. } => {
          match event {
            WindowEvent::CloseRequested => {
              exiting = true;
            }

            WindowEvent::Resized(size) => {
              window.size = size_to_vector(size, window.inner.get_hidpi_factor());
            }

            _ => {}
          };

          window.events.push(event);
        }

        _ => {}
      });
    }

    if exiting {
      engine::exit_loop(ctx);
    }
  }
}

pub fn init(ctx: &mut engine::Context) {
  let events_loop = EventsLoop::new();
  let window = WindowBuilder::new()
    .with_title("nova")
    .build(&events_loop)
    .expect("could not create window");

  let size = size_to_vector(
    window.get_inner_size().expect("window is closed"),
    window.get_hidpi_factor(),
  );

  engine::add_extension(ctx, Extension { events_loop });

  engine::add_resource(
    ctx,
    Window {
      inner: window,
      events: Vec::new(),
      size,
    },
  );
}

fn size_to_vector(size: winit::dpi::LogicalSize, dpi: f64) -> Vector2<f32> {
  let size = size.to_physical(dpi);

  Vector2::new(size.width as f32, size.height as f32)
}
