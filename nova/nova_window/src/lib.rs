// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod options;

pub use self::options::Options;

use crossbeam_channel as channel;
use nova_math::Size;
use std::sync::Arc;
use std::thread;

/// Handle to a platform-specific window.
///
/// When this structure is dropped, the window is closed.
#[derive(Clone)]
pub struct Handle {
  window: Arc<winit::Window>,
  events: channel::Receiver<winit::WindowEvent>,
}

impl Handle {
  /// Gets the DPI scaling factor of the window.
  ///
  /// On standard DPI screens this returns `1.0`. On high definition screens
  /// it may return `2.0` or some other multiplier.
  pub fn dpi_scaling(&self) -> f64 {
    self.window.get_hidpi_factor()
  }

  /// Gets the size of the window in pixels.
  pub fn size(&self) -> Size<f64> {
    if let Some(size) = self.window.get_inner_size() {
      Size::new(size.width, size.height) * self.dpi_scaling()
    } else {
      Size::default()
    }
  }

  /// Returns the next available window event if one exists or `None` if there
  /// is no available event.
  ///
  /// This function does not block and is intended to be called during the
  /// main application loop to retrieve window events asynchronously.
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
  let (event_sender, events) = channel::unbounded();
  let (window_sender, window) = channel::bounded(0);

  // Start the event loop thread.
  thread::spawn(move || {
    // Set up an event loop and create the window.
    let mut events_loop = winit::EventsLoop::new();

    let size = winit::dpi::PhysicalSize::new(options.size.width, options.size.height)
      .to_logical(events_loop.get_primary_monitor().get_hidpi_factor());

    let window = winit::WindowBuilder::new()
      .with_title(options.title)
      .with_resizable(options.resizable)
      .with_dimensions(size)
      .build(&events_loop)
      .expect("Could not create window");

    // Send the window back to the original thread and drop the channel.
    if window_sender.send(window).is_err() {
      return;
    }

    drop(window_sender);

    // Run the event loop, sending events to the main thread, until the channel
    // is closed on the other end meaning all handles have been dropped and the
    // window should close.
    events_loop.run_forever(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        if event_sender.send(event).is_err() {
          return winit::ControlFlow::Break;
        }
      }

      winit::ControlFlow::Continue
    });
  });

  // Receive the window from the background thread and wrap it.
  let window = window.recv().expect("Could not create window").into();

  Handle { window, events }
}

impl AsRef<winit::Window> for Handle {
  fn as_ref(&self) -> &winit::Window {
    &self.window
  }
}
