// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod events_loop;

pub use self::events_loop::*;
pub use winit::{CreationError, WindowEvent};

use crate::events;

pub struct Window {
  status: Status,
  events: events::Channel<WindowEvent>,
  title: String,
}

impl Window {
  pub fn new() -> Window {
    Window {
      status: Status::Closed,
      events: events::Channel::new(),
      title: "nova".to_owned(),
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
    Window::new()
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status {
  Closed,
  Opening,
  Open,
  Closing,
}
