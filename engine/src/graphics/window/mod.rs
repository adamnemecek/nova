pub mod swapchain;

mod surface;

pub use self::surface::Surface;
pub use self::swapchain::Swapchain;
pub use winit::CreationError;

use crate::graphics::backend;
use crate::math::algebra::Vector2;
use std::sync::Arc;

pub struct Window {
  events_loop: winit::EventsLoop,
  raw: winit::Window,
  surface: Arc<Surface>,
  size: Vector2<u32>,
  closed: bool,
}

impl Window {
  pub fn new(backend: &Arc<backend::Instance>) -> Result<Window, CreationError> {
    let events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
      .with_title("nova")
      .build(&events_loop)?;

    let size = physical_size_of(&window);

    let surface = Surface::new(backend, &window).into();

    Ok(Window {
      events_loop,
      raw: window,
      surface,
      size,
      closed: false,
    })
  }

  pub fn update(&mut self) {
    let mut closing = false;
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

    if closing {
      self.closed = true;
    }

    if resized {
      self.size = physical_size_of(&self.raw);
    }
  }

  pub fn surface(&self) -> &Arc<Surface> {
    &self.surface
  }

  pub fn is_closed(&self) -> bool {
    self.closed
  }

  pub fn size(&self) -> Vector2<u32> {
    self.size
  }
}

fn physical_size_of(window: &winit::Window) -> Vector2<u32> {
  let size = window
    .get_inner_size()
    .expect("window destroyed")
    .to_physical(window.get_hidpi_factor());

  Vector2::new(size.width.round() as u32, size.height.round() as u32)
}
