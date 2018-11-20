/// One of the possible window events.
pub enum Event {
  Resized,
  CloseRequested,
}

/// A source that window events can be pulled from.
///
/// This is returned along with [`Window`] from [`Window::new`] and will poll
/// events for that window.
pub struct EventSource {
  events: Vec<Event>,
  events_loop: winit::EventsLoop,
}

impl EventSource {
  /// Events that occurred on the last update.
  pub fn events(&self) -> &[Event] {
    &self.events
  }

  /// Updates the event source by polling for new events.
  ///
  /// Previous events are removed. This structure is intended to store only one
  /// “frame” of events at a time.
  pub fn poll(&mut self) {
    let events = &mut self.events;

    events.clear();

    self.events_loop.poll_events(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        match event {
          winit::WindowEvent::CloseRequested => {
            events.push(Event::CloseRequested);
          }

          winit::WindowEvent::Resized(_) => {
            events.push(Event::Resized);
          }

          _ => {}
        }
      }
    });
  }
}

// Implement `From` to create event sources from winit events loops.
impl From<winit::EventsLoop> for EventSource {
  fn from(events_loop: winit::EventsLoop) -> Self {
    EventSource {
      events: Vec::new(),
      events_loop,
    }
  }
}
