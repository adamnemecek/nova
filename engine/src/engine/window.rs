// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Context;
use crate::prelude::*;
use ggez::event::winit_event::Event;
use std::cell::Cell;

pub use ggez::event::winit_event;
pub use ggez::event::winit_event::WindowEvent;

/// Resource that stores the state of the engine window.
pub struct Window {
  /// Size of the window in pixels.
  size: Vector2<f32>,
  /// Whether the window was resized this engine loop.
  was_resized: bool,
  /// Events received by the window during this engine loop.
  events: Vec<WindowEvent>,
}

impl Window {
  /// Gets the size of the window in pixels.
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  /// Gets whether the window was resized this engine loop.
  pub fn was_resized(&self) -> bool {
    self.was_resized
  }

  /// Gets the events that the window received this engine loop.
  pub(crate) fn events(&self) -> &[WindowEvent] {
    &self.events
  }
}

// Blocks default window creation since it must be backed by a real window.
impl Default for Window {
  fn default() -> Self {
    unimplemented!();
  }
}

/// Engine state for a platform-specific window.
pub struct WindowHandle {
  /// Cell initially containing the ggez context which can be taken by the
  /// graphics module.
  pub(crate) ggez: Cell<Option<ggez::Context>>,
  /// winit events loop for the window.
  events_loop: ggez::event::EventsLoop,
}

pub fn create_window(ctx: &mut Context, title: &str) {
  if super::has_resource::<Window>(ctx) {
    panic!("engine context already has a window");
  }

  let (mut ggez, events_loop) = ggez::ContextBuilder::new("nova", "bfrydl")
    .window_mode(ggez::conf::WindowMode::default().resizable(true))
    .window_setup(ggez::conf::WindowSetup::default().title(title).vsync(false))
    .build()
    .expect("could not create ggez::Context");

  let screen = ggez::graphics::screen_coordinates(&mut ggez);

  // Clear the window with eigengrau by default.
  ggez::graphics::clear(
    &mut ggez,
    ggez::graphics::Color::new(0.086, 0.086, 0.114, 1.0),
  );

  ggez::graphics::present(&mut ggez).expect("could not do initial present");

  // Register a resource with info about the window.
  super::add_resource(
    ctx,
    Window {
      size: Vector2::new(screen.w as f32, screen.h as f32),
      was_resized: false,
      events: Vec::new(),
    },
  );

  ctx.window_handle.replace(Some(WindowHandle {
    ggez: Cell::new(Some(ggez)),
    events_loop,
  }));
}

pub(super) fn update_window(ctx: &mut Context) {
  let mut closing = false;

  {
    let mut handle = ctx.window_handle.borrow_mut();
    let handle = handle
      .as_mut()
      .expect("engine context does not have a window");

    let mut window = super::fetch_resource_mut::<Window>(ctx);

    window.was_resized = false;
    window.events.clear();

    handle.events_loop.poll_events(|event| match event {
      Event::WindowEvent { event, .. } => {
        match event {
          WindowEvent::Resized(new_size) => {
            window.size = Vector2::new(new_size.width as f32, new_size.height as f32);
            window.was_resized = true;
          }

          WindowEvent::CloseRequested => {
            closing = true;
          }

          _ => {}
        }

        window.events.push(event);
      }

      _ => {}
    });
  }

  if closing {
    super::exit(ctx);
  }
}
