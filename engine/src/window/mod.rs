// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod update;

pub use self::update::*;
pub use winit::{CreationError, WindowEvent};

use crate::events;

pub struct Window {
  handle: Option<winit::Window>,
  status: Status,
  events: events::Channel<WindowEvent>,
  title: String,
}

impl Window {
  pub fn new() -> Window {
    Window {
      handle: None,
      status: Status::Opening,
      events: events::Channel::new(),
      title: String::new(),
    }
  }

  pub fn status(&self) -> Status {
    self.status
  }

  pub fn is_closing(&self) -> bool {
    self.status == Status::Closing
  }

  pub fn is_closed(&self) -> bool {
    self.status == Status::Closed
  }

  pub fn open(&mut self) {
    match &self.status {
      Status::Closed => {
        self.status = Status::Opening;
      }

      Status::Closing => {
        self.status = Status::Open;
      }

      _ => {}
    }
  }

  pub fn close(&mut self) {
    match &self.status {
      Status::Open => {
        self.status = Status::Closing;
      }

      Status::Opening => {
        self.status = Status::Closed;
      }

      _ => {}
    }
  }

  pub fn title(&self) -> &str {
    &self.title
  }

  pub fn set_title(&mut self, title: &str) -> &mut Self {
    self.title.replace_range(.., title);
    self
  }
}

impl Default for Window {
  fn default() -> Window {
    let mut window = Window::new();

    if let Ok(exe) = std::env::current_exe() {
      if let Some(stem) = exe.file_stem() {
        window.set_title(&stem.to_string_lossy());
      }
    }

    window
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
  Closed,
  Opening,
  Open,
  Closing,
}
