// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use ggez::event::winit_event::Event;
use std::cell::Cell;

/// Struct that represents a platform-specific window.
pub struct Window {
  /// Cell initially containing the ggez context. Taken by `graphics::Canvas`.
  pub(crate) ctx: Cell<Option<ggez::Context>>,
  events_loop: ggez::event::EventsLoop,
  events: Vec<WindowEvent>,
  size: Vector2<f32>,
  was_resized: bool,
  is_closing: bool,
}

impl Window {
  /// Creates a new window with the given title.ü
  pub fn new(title: &str) -> Window {
    let (mut ctx, events_loop) = ggez::ContextBuilder::new("nova", "bfrydl")
      .window_mode(ggez::conf::WindowMode::default().resizable(true))
      .window_setup(ggez::conf::WindowSetup::default().title(title).vsync(false))
      .build()
      .expect("could not create ggez::Context");

    let screen = ggez::graphics::screen_coordinates(&mut ctx);

    // Clear the window with eigengrau by default.
    ggez::graphics::clear(
      &mut ctx,
      ggez::graphics::Color::new(0.086, 0.086, 0.114, 1.0),
    );

    ggez::graphics::present(&mut ctx).expect("could not do initial present");

    Window {
      ctx: Cell::new(Some(ctx)),
      events_loop,
      events: Vec::new(),
      size: Vector2::new(screen.w, screen.h),
      was_resized: false,
      is_closing: false,
    }
  }

  /// Gets the events that ocurred during the previous update.
  pub fn events(&self) -> &[WindowEvent] {
    &self.events
  }

  /// Gets the size of the window in pixels.
  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  /// Returns `true` if the window was resized during the previous update.
  pub fn was_resized(&self) -> bool {
    self.was_resized
  }

  /// Returns `true` if the user has requested that the window close.
  pub fn is_closing(&self) -> bool {
    self.is_closing
  }

  /// Updates the window, processing all events that occurred since the last
  /// update.
  pub fn update(&mut self) {
    let mut size = self.size;
    let mut was_resized = false;
    let mut is_closing = self.is_closing;

    let events = &mut self.events;

    events.clear();

    self.events_loop.poll_events(|event| match event {
      Event::WindowEvent { event, .. } => {
        match event {
          WindowEvent::Resized(new_size) => {
            size = Vector2::new(new_size.width as f32, new_size.height as f32);
            was_resized = true;
          }

          WindowEvent::CloseRequested => {
            is_closing = true;
          }

          _ => {}
        }

        events.push(event);
      }

      _ => {}
    });

    self.size = size;
    self.was_resized = was_resized;
    self.is_closing = is_closing;
  }
}
