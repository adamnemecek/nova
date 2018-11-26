// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events;
mod raw;
mod settings;

pub use self::events::*;
pub use self::raw::*;
pub use self::settings::*;
pub use winit::CreationError;

use crate::Engine;

/// Represents a platform-specfic window.
pub struct Window {
  event_source: EventSource,
  handle: RawHandle,
  settings: Settings,
}

impl Window {
  /// Creates a new platform-specific window.
  ///
  /// To control the window settings, add a [`Settings`] resource to the engine
  /// before calling this function.
  pub fn create(engine: &mut Engine) -> Result<Window, CreationError> {
    engine.put_resource(Events::default());

    let settings: &mut Settings = engine.ensure_resource();

    let events_loop = winit::EventsLoop::new();

    let size = logical_size(
      settings.size,
      events_loop.get_primary_monitor().get_hidpi_factor(),
    );

    let mut builder = winit::WindowBuilder::new()
      .with_title(settings.title.clone())
      .with_resizable(settings.resizable)
      .with_fullscreen(match settings.fullscreen {
        true => Some(events_loop.get_primary_monitor()),
        false => None,
      })
      .with_min_dimensions(size);

    if !settings.resizable {
      builder = builder.with_max_dimensions(size);
    }

    let handle: RawHandle = builder.build(&events_loop)?.into();

    settings.size = handle.get_inner_size();

    Ok(Window {
      event_source: events_loop.into(),
      handle,
      settings: settings.clone(),
    })
  }

  /// Gets a reference to the platform-specific window handle.
  pub fn handle(&self) -> &RawHandle {
    &self.handle
  }

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

    if settings.fullscreen != self.settings.fullscreen {
      self.handle.set_fullscreen(match settings.fullscreen {
        true => Some(self.handle.get_current_monitor()),
        false => None,
      });

      self.settings.fullscreen = settings.fullscreen;
    }

    if settings.size != self.settings.size {
      self.handle.set_inner_size(settings.size);
      self.settings.size = settings.size;
    }

    // Poll events into the events resource.
    let events: &mut Events = engine.get_resource_mut();

    events.latest.clear();

    self.event_source.poll_into(&mut events.latest);

    // Now update the settings resource with any changes to the window.
    if events.latest.contains(&Event::Resized) {
      let size = self.handle.get_inner_size();
      let settings: &mut Settings = engine.get_resource_mut();

      settings.size = size;
      self.settings.size = size;
    }
  }
}
