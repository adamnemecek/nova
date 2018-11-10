use crate::graphics::backend;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Surface {
  surface: Mutex<backend::Surface>,
  backend: Arc<backend::Instance>,
}

impl Surface {
  pub fn new(backend: &Arc<backend::Instance>, window: &winit::Window) -> Surface {
    let surface = backend.create_surface(&window).into();

    Surface {
      surface,
      backend: backend.clone(),
    }
  }

  pub fn backend(&self) -> &Arc<backend::Instance> {
    &self.backend
  }

  pub fn lock(&self) -> MutexGuard<backend::Surface> {
    self.surface.lock().unwrap()
  }
}
