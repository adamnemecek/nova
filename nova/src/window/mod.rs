// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod options;

pub use self::options::Options;

use super::*;

/// Handle to a platform-specific window.
///
/// When this structure is dropped, the window is closed.
#[derive(Clone)]
pub struct Handle {
  window: Arc<winit::Window>,
  events: channel::Receiver<winit::WindowEvent>,
}

impl Handle {
  /// Returns the DPI scaling factor of the window.
  ///
  /// On standard DPI screens this returns `1.0`. On high definition screens
  /// it may return `2.0` or some other multiplier.
  pub fn dpi_scaling(&self) -> f64 {
    self.window.get_hidpi_factor()
  }

  /// Returns the size of the window in pixels.
  pub fn size(&self) -> Size<f64> {
    if let Some(size) = self.window.get_inner_size() {
      Size::new(size.width, size.height) * self.dpi_scaling()
    } else {
      Size::default()
    }
  }

  /// Returns the next window event if one is available or `None` if there is no
  /// available event.
  pub fn next_event(&self) -> Option<Event> {
    self.events.try_recv().ok().and_then(|event| match event {
      winit::WindowEvent::CloseRequested => Some(Event::CloseRequested),
      winit::WindowEvent::Resized(_) => Some(Event::Resized),

      _ => None,
    })
  }
}

/// A window event.
#[derive(Debug)]
pub enum Event {
  /// The user requested for the window to close, such as by clicking on the
  /// window's X button.
  CloseRequested,

  /// The window has been resized. Use the `Window::size()` method to get the
  /// new size.
  Resized,
}

/// Opens a new window with the given options.
pub fn open(options: Options) -> Handle {
  // Create channels to communicate with the window's event loop thread.
  let (send_events, recv_events) = channel::unbounded();
  let (send_window, recv_window) = channel::bounded(0);

  // Start the event loop thread.
  thread::spawn(move || {
    // Set up an event loop and create the window.
    let mut events_loop = winit::EventsLoop::new();

    let monitor = events_loop.get_primary_monitor();

    // Use the given size or a default size that is a multiple of 1280x720.
    let size = match options.size {
      Some(size) => winit::dpi::PhysicalSize::new(size.width, size.height),

      None => {
        let monitor_size = monitor.get_dimensions();

        // Get the fractional multiple of 1280x720 that fits on the screen in
        // both dimensions.
        let ideal_scale = (monitor_size.width / 1280.0).min(monitor_size.height / 720.0);

        // Subtract 1 to make the window smaller than the monitor, round up,
        // then ensure the window is at least 1280x720.
        let scale = (ideal_scale - 1.0).ceil().max(1.0);

        winit::dpi::PhysicalSize::new(1280.0 * scale, 720.0 * scale)
      }
    };

    // Try to create a winit window with the given options and send the result
    // back to the original thread.
    let window = winit::WindowBuilder::new()
      .with_title(options.title)
      .with_resizable(options.resizable)
      .with_dimensions(size.to_logical(monitor.get_hidpi_factor()))
      .build(&events_loop);

    let created = window.is_ok();

    if send_window.send(window).is_err() || !created {
      return;
    }

    drop(send_window);

    // Run the event loop, sending events to the window handle, until the
    // channel is closed on the other end meaning all handles have been dropped
    // and the window should close.
    events_loop.run_forever(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        if send_events.send(event).is_err() {
          return winit::ControlFlow::Break;
        }
      }

      winit::ControlFlow::Continue
    });
  });

  // Receive the window from the background thread and wrap it.
  let window = recv_window
    .recv()
    .unwrap()
    .expect("could not createa window")
    .into();

  Handle {
    window,
    events: recv_events,
  }
}

impl AsRef<winit::Window> for Handle {
  fn as_ref(&self) -> &winit::Window {
    &self.window
  }
}
