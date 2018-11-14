use crossbeam::channel::Sender;

/// One of the possible window events.
pub enum Event {
  Resized,
  CloseRequested,
}

/// Processes events on a loop, sending them to the output channel.
///
/// This function continues until the channel is closed.
pub fn process(mut events_loop: winit::EventsLoop, output: Sender<Vec<Event>>) {
  loop {
    let mut events = Vec::new();

    events_loop.poll_events(|event| match event {
      winit::Event::WindowEvent { event, .. } => match event {
        winit::WindowEvent::CloseRequested => {
          events.push(Event::CloseRequested);
        }

        winit::WindowEvent::Resized(_) => {
          events.push(Event::Resized);
        }

        _ => {}
      },

      _ => {}
    });

    if output.send(events).is_err() {
      return;
    }
  }
}
