// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod backbuffer;

use crate::ecs;
use crate::math::Size;
use crate::thread;
use crate::time;
use crossbeam::channel;

pub use self::backbuffer::{acquire_backbuffer, present_backbuffer, Backbuffer};
pub use winit::WindowEvent as Event;

type EventChannel = crate::events::Channel<Event>;

pub fn setup(res: &mut ecs::Resources, options: Options) {
  if res.has_value::<Window>() {
    return;
  }

  let (send_event, recv_event) = channel::bounded(128);
  let (send_win, recv_win) = channel::bounded(0);

  {
    let options = options.clone();

    thread::spawn(move || {
      let mut events_loop = winit::EventsLoop::new();

      // TODO: Error handling.
      let window = winit::WindowBuilder::new()
        .with_title(options.title.clone())
        .build(&events_loop)
        .expect("Could not create window");

      send_win
        .send(window)
        .expect("Could not send window from events loop thread");

      loop {
        let mut exit = false;

        events_loop.poll_events(|event| {
          if let winit::Event::WindowEvent { event, .. } = event {
            if send_event.send(event).is_err() {
              exit = true;
            }
          }
        });

        if exit {
          break;
        }

        thread::sleep(time::Duration::from_millis(1));
      }
    });
  };

  let handle = recv_win
    .recv()
    // TODO: Error handling.
    .expect("Could not receive window from events loop thread");

  res.insert(Window {
    options,
    handle,
    events: EventChannel::new(),
    event_receiver: recv_event,
  });
}

pub fn poll_events() -> PollEvents {
  PollEvents
}

pub struct Window {
  options: Options,
  handle: winit::Window,
  events: EventChannel,
  event_receiver: channel::Receiver<Event>,
}

impl Window {
  pub(crate) fn handle(&self) -> &winit::Window {
    &self.handle
  }

  pub fn size(&self) -> Size<u32> {
    self.options.size
  }

  pub fn set_title(&mut self, title: &str) -> &mut Self {
    if title != self.options.title {
      self.options.set_title(title);
      self.handle.set_title(title);
    }

    self
  }

  pub fn poll_events(&mut self) {
    while let Ok(event) = self.event_receiver.try_recv() {
      self.events.single_write(event);
    }
  }
}

#[derive(Clone)]
pub struct Options {
  pub title: String,
  pub size: Size<u32>,
}

impl Options {
  pub fn new() -> Self {
    Options {
      title: String::new(),
      size: Size::new(1280, 720),
    }
  }

  pub fn set_title(&mut self, title: &str) {
    self.title.replace_range(.., title);
  }
}

impl Default for Options {
  fn default() -> Self {
    let mut options = Options::new();

    if let Ok(exe) = std::env::current_exe() {
      if let Some(stem) = exe.file_stem() {
        options.set_title(&stem.to_string_lossy());
      }
    }

    options
  }
}

pub struct PollEvents;

impl<'a> ecs::System<'a> for PollEvents {
  type SystemData = ecs::WriteResource<'a, Window>;

  fn run(&mut self, mut window: Self::SystemData) {
    while let Ok(event) = window.event_receiver.try_recv() {
      if let Event::Resized(size) = &event {
        let size: (u32, u32) = size.to_physical(window.handle.get_hidpi_factor()).into();

        window.options.size = size.into();
      }

      window.events.single_write(event);
    }
  }
}
