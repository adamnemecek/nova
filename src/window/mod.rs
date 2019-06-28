// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod handle;
mod options;

pub use self::handle::*;
pub use self::options::*;

use super::*;

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
pub fn open(options: Options) -> Result<Handle, OpenError> {
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
  let window = recv_window.recv()??;

  Ok(Handle::new(window, recv_events))
}

#[derive(Debug)]
pub enum OpenError {
  /// An unknown error occurred.
  Unknown,
  /// An error occurred while creating the window.
  CreationFailed(String),
}

impl fmt::Display for OpenError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      OpenError::CreationFailed(reason) => write!(f, "{}", reason),
      OpenError::Unknown => write!(f, "unknown error"),
    }
  }
}

impl From<channel::RecvError> for OpenError {
  fn from(_: channel::RecvError) -> Self {
    OpenError::Unknown
  }
}

impl From<winit::CreationError> for OpenError {
  fn from(err: winit::CreationError) -> Self {
    OpenError::CreationFailed(format!("{}", err))
  }
}
