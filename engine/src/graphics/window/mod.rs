// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod swapchain;

mod surface;

pub use self::surface::Surface;
pub use self::swapchain::Swapchain;
pub use winit::CreationError;

use crate::graphics::backend;
use crate::math::algebra::Vector2;
use std::sync::Arc;

/// Represents a platform-specfic window.
pub struct Window {
  /// Events loop the window was created with.
  events_loop: winit::EventsLoop,
  /// Raw winit window structure.
  raw: winit::Window,
  /// Rendering surface created from the window with a backend instance.
  surface: Surface,
  /// Size of the window in pixels.
  size: Vector2<u32>,
  /// Whether the user has requested the window be closed.
  closing: bool,
}

impl Window {
  /// Creates a new platform-specific window with a rendering surface for the
  /// given backend instance.
  pub fn new(backend: &Arc<backend::Instance>) -> Result<Window, CreationError> {
    let events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
      .with_title("nova")
      .build(&events_loop)?;

    let size = pixel_size_of(&window);

    let surface = Surface::new(backend, &window);

    Ok(Window {
      events_loop,
      raw: window,
      surface,
      size,
      closing: false,
    })
  }

  /// Gets a mutable reference to the rendering surface of the window.
  pub fn surface_mut(&mut self) -> &mut Surface {
    &mut self.surface
  }

  /// Returns `true` after the user requests closing the window.
  pub fn is_closing(&self) -> bool {
    self.closing
  }

  /// Gets the size of the window in pixels.
  pub fn size(&self) -> Vector2<u32> {
    self.size
  }

  /// Updates the window by processing events that have occured since the last
  /// update.
  pub fn update(&mut self) {
    let mut closing = false;
    let mut resized = false;

    self.events_loop.poll_events(|event| match event {
      winit::Event::WindowEvent { event, .. } => match event {
        winit::WindowEvent::CloseRequested => {
          closing = true;
        }

        winit::WindowEvent::Resized(_) => {
          resized = true;
        }

        _ => {}
      },

      _ => {}
    });

    if closing {
      self.closing = true;
    }

    if resized {
      self.size = pixel_size_of(&self.raw);
    }
  }
}

/// Determines the size of a window in pixels.
fn pixel_size_of(window: &winit::Window) -> Vector2<u32> {
  let size = window
    .get_inner_size()
    .expect("window destroyed")
    .to_physical(window.get_hidpi_factor());

  Vector2::new(size.width.round() as u32, size.height.round() as u32)
}
