// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod handle;

pub use self::events::*;
pub use self::handle::*;
pub use winit::CreationError;

use crate::ecs;
use crate::math::Size;
use crate::Engine;
use std::borrow::Borrow;

/// Represents a platform-specfic window.
pub struct Window {
  /// Handle to the platform-specific window
  handle: Handle,
  /// Size of the window in pixels.
  size: Size<u32>,
  /// Whether the user has requested the window be closed.
  closing: bool,
}

impl Window {
  /// Creates a new platform-specific window.
  ///
  /// This function returns a `Window` and an [`EventSource`] which can be used
  /// to poll window events.
  pub fn create() -> Result<(Window, EventSource), CreationError> {
    let events_loop = winit::EventsLoop::new();

    let handle: Handle = winit::WindowBuilder::new()
      .with_title("nova")
      .build(&events_loop)?
      .into();

    let size = handle.calculate_inner_size();

    let window = Window {
      handle,
      size,
      closing: false,
    };

    Ok((window, events_loop.into()))
  }

  /// Gets a reference to the platform-specific window handle.
  pub fn handle(&self) -> &Handle {
    &self.handle
  }

  /// Returns `true` after the user requests closing the window.
  pub fn is_closing(&self) -> bool {
    self.closing
  }

  /// Gets the size of the window in pixels.
  pub fn size(&self) -> Size<u32> {
    self.size
  }

  /// Updates the window by processing events that have occured since the last
  /// update.
  pub fn process_events<E>(&mut self, events: impl IntoIterator<Item = E>)
  where
    E: Borrow<Event>,
  {
    for event in events {
      match event.borrow() {
        Event::CloseRequested => {
          self.closing = true;
        }

        Event::Resized => {
          self.size = self.handle.calculate_inner_size();
        }
      }
    }
  }
}

/// Updates an engine's [`Window`] resource by processing the given events.
pub fn process_events<E>(engine: &mut Engine, events: impl IntoIterator<Item = E>)
where
  E: Borrow<Event>,
{
  ecs::get_resource_mut::<Window>(engine).process_events(events.into_iter());
}

/// Gets whether or not an engine's [`Window`] is closing.
pub fn is_closing(engine: &mut Engine) -> bool {
  ecs::get_resource_mut::<Window>(engine).is_closing()
}
