use crate::math::algebra::Vector2;

pub struct Window {
  events_loop: winit::EventsLoop,
  raw: winit::Window,
  size: Vector2<f32>,
  resized: bool,
  closing: bool,
}

impl Window {
  pub fn new(title: &str) -> Window {
    let events_loop = winit::EventsLoop::new();

    let raw = winit::WindowBuilder::new()
      .with_title(title)
      .build(&events_loop)
      .expect("could not create window");

    let mut window = Window {
      events_loop,
      raw,
      size: Vector2::zeros(),
      resized: false,
      closing: false,
    };

    window.update_size();
    window
  }

  pub fn raw(&self) -> &winit::Window {
    &self.raw
  }

  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  pub fn was_resized(&self) -> bool {
    self.resized
  }

  pub fn is_closing(&self) -> bool {
    self.closing
  }

  pub fn poll_events(&mut self) {
    let mut closing = self.closing;
    let mut resized = false;

    self.events_loop.poll_events(|event| match event {
      winit::Event::WindowEvent { event, .. } => match event {
        winit::WindowEvent::CloseRequested => {
          closing = true;
        }

        winit::WindowEvent::Resized(_) => {
          resized = true;
        }

        _ => {}
      },

      _ => {}
    });

    self.closing = closing;
    self.resized = resized;

    if resized {
      self.update_size();
    }
  }

  fn update_size(&mut self) {
    let size = self
      .raw
      .get_inner_size()
      .map(|s| s.to_physical(self.raw.get_hidpi_factor()))
      .map(|s| Vector2::new(s.width as f32, s.height as f32));

    if let Some(size) = size {
      self.size = size;
    }
  }
}
