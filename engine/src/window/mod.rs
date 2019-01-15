// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::math::Size;
pub use winit::CreationError;

#[derive(Debug)]
pub struct Window {
  raw: winit::Window,
  events_loop: winit::EventsLoop,
  options: Options,
}

impl Window {
  pub fn create(options: Options) -> Result<Self, CreationError> {
    let mut builder = winit::WindowBuilder::new()
      .with_title(options.title.clone())
      .with_resizable(options.resizable)
      .with_dimensions(winit::dpi::LogicalSize::from((
        options.size.width(),
        options.size.height(),
      )));

    builder = match options.mode {
      Mode::BorderlessWindowed | Mode::Fullscreen => builder.with_decorations(false),
      _ => builder,
    };

    let events_loop = winit::EventsLoop::new();
    let raw = builder.build(&events_loop)?;

    raw.set_fullscreen(match options.mode {
      Mode::Fullscreen => Some(raw.get_current_monitor()),
      _ => None,
    });

    Ok(Window {
      raw,
      events_loop,
      options,
    })
  }

  pub fn options(&self) -> &Options {
    &self.options
  }

  pub fn set_options(&mut self, options: &Options) {
    if options.title != self.options.title {
      self.raw.set_title(&options.title);
      self.options.title.replace_range(.., &options.title);
    }

    if options.resizable != self.options.resizable {
      self.raw.set_resizable(options.resizable);
      self.options.resizable = true;
    }

    if options.size != self.options.size {
      self.raw.set_inner_size(winit::dpi::LogicalSize::from((
        options.size.width(),
        options.size.height(),
      )));

      self.options.size = options.size;
    }

    if options.mode != self.options.mode {
      self.raw.set_decorations(match options.mode {
        Mode::Windowed => true,
        _ => false,
      });

      self.raw.set_fullscreen(match options.mode {
        Mode::Fullscreen => Some(self.raw.get_current_monitor()),
        _ => None,
      });

      self.options.mode = options.mode;
    }
  }

  pub fn update(&mut self) {
    let options = &mut self.options;

    self.events_loop.poll_events(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        if let winit::WindowEvent::Resized(size) = event {
          let size: (u32, u32) = size.into();

          options.size = Size::new(size.0, size.1);
        }
      }
    });
  }
}

#[derive(Debug, Clone)]
pub struct Options {
  title: String,
  resizable: bool,
  mode: Mode,
  size: Size<u32>,
}

impl Default for Options {
  fn default() -> Options {
    let exe_name = std::env::current_exe().ok().and_then(|path| {
      path
        .file_stem()
        .and_then(|stem| stem.to_str().map(String::from))
    });

    Options {
      title: exe_name.unwrap_or_else(|| "nova".into()),
      resizable: true,
      mode: Mode::Windowed,
      size: Size::new(640, 360),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Mode {
  Windowed,
  BorderlessWindowed,
  Fullscreen,
}
