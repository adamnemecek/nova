// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod handle;
mod settings;

pub use self::events::*;
pub use self::handle::*;
pub use self::settings::*;
pub use winit::CreationError;

use crate::math::Size;
use crate::Engine;

/// Represents a platform-specfic window.
pub struct Window {
  event_source: EventSource,
  handle: Handle,
  settings: Settings,
}

impl Window {
  /// Creates a new platform-specific window.
  ///
  /// To control the window settings, add a [`Settings`] resource to the engine
  /// before calling this function.
  pub fn create(engine: &mut Engine) -> Result<Window, CreationError> {
    engine.ensure_resource::<Events>();

    let settings: &mut Settings = engine.ensure_resource();

    let events_loop = winit::EventsLoop::new();

    let size = logical_size(
      settings.size,
      events_loop.get_primary_monitor().get_hidpi_factor(),
    );

    let mut builder = winit::WindowBuilder::new()
      .with_title(settings.title.clone())
      .with_resizable(settings.resizable)
      .with_min_dimensions(size);

    if !settings.resizable {
      builder = builder.with_max_dimensions(size);
    }

    let handle = Handle::from(builder.build(&events_loop)?);

    handle.set_fullscreen(settings.fullscreen);

    settings.size = handle.get_size();

    Ok(Window {
      event_source: events_loop.into(),
      handle,
      settings: settings.clone(),
    })
  }

  /// Gets a handle to the underlying window which can be cloned to share
  /// access.
  pub fn handle(&self) -> &Handle {
    &self.handle
  }

  /// Updates the window using resources from the given engine instance.
  ///
  /// This function updates the underlying window from any changes made to the
  /// [`Settings`] resource, polls events and stores them in the [`Events`]
  /// resource, then updates the [`Settings`] resource with the modified state of
  /// the underlying window.
  pub fn update(&mut self, engine: &mut Engine) {
    // Update the window with any changes to settings.
    let settings: &mut Settings = engine.get_resource_mut();

    if settings.title != self.settings.title {
      self.handle.set_title(&settings.title);
      self.settings.set_title(&settings.title);
    }

    if settings.resizable != self.settings.resizable {
      self.handle.set_resizable(settings.resizable);
      self.settings.resizable = settings.resizable;
    }

    if settings.size != self.settings.size {
      self.handle.set_size(settings.size);
      self.settings.size = settings.size;
    }

    if settings.fullscreen != self.settings.fullscreen {
      self.handle.set_fullscreen(self.settings.fullscreen);
      self.settings.fullscreen = settings.fullscreen;
    }

    // Poll events into the events resource.
    let events: &mut Events = engine.get_resource_mut();

    events.latest.clear();

    self.event_source.poll_into(&mut events.latest);

    // Now update the settings resource with any changes to the window.
    if events.latest.contains(&Event::Resized) {
      let size = self.handle.get_size();
      let settings: &mut Settings = engine.get_resource_mut();

      settings.size = size;
      self.settings.size = size;
    }
  }
}

/// Converts the given winit logical size to a pixel [`Size`].
fn physical_size(size: winit::dpi::LogicalSize, dpi: f64) -> Size<u32> {
  let size: (u32, u32) = size.to_physical(dpi).into();

  Size::new(size.0, size.1)
}

/// Converts the given pixel [`Size`] to a winit logical size.
fn logical_size(size: Size<u32>, dpi: f64) -> winit::dpi::LogicalSize {
  winit::dpi::LogicalSize::from_physical((size.width(), size.height()), dpi)
}
