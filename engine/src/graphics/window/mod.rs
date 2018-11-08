mod swapchain;

pub use self::swapchain::*;
pub use winit::CreationError;

use super::hal::*;
use crate::math::algebra::Vector2;
use std::sync::Arc;

pub struct Window {
  size: Vector2<f32>,
  closed: bool,
  events_loop: winit::EventsLoop,
  raw: winit::Window,
  surface: backend::Surface,
  _instance: Arc<backend::Instance>,
}

impl Window {
  pub fn new(instance: &Arc<backend::Instance>) -> Result<Window, CreationError> {
    let events_loop = winit::EventsLoop::new();

    let window = winit::WindowBuilder::new()
      .with_title("nova")
      .build(&events_loop)?;

    let surface = instance.create_surface(&window);

    Ok(Window {
      size: physical_size_of(&window),
      closed: false,
      events_loop,
      raw: window,
      surface,
      _instance: instance.clone(),
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

  pub fn is_closed(&self) -> bool {
    self.closed
  }

  pub fn size(&self) -> Vector2<f32> {
    self.size
  }

  pub fn raw_surface(&self) -> &backend::Surface {
    &self.surface
  }

  pub fn raw_surface_mut(&mut self) -> &mut backend::Surface {
    &mut self.surface
  }
}

fn physical_size_of(window: &winit::Window) -> Vector2<f32> {
  let size = window
    .get_inner_size()
    .expect("window destroyed")
    .to_physical(window.get_hidpi_factor());

  Vector2::new(size.width as f32, size.height as f32)
}
