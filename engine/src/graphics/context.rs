use super::device::{self, Device};
use super::hal::backend;
use super::window::{self, Window};
use crate::utils::quick_error;
use std::sync::Arc;

pub struct Context {
  pub window: Window,
  pub device: Arc<Device>,
  pub queues: device::DefaultQueueSet,
}

impl Context {
  pub fn new(app_name: &str, app_version: u32) -> Result<Context, CreationError> {
    let instance = Arc::new(backend::Instance::create(app_name, app_version));
    let window = Window::new(&instance)?;
    let (device, queues) = Device::new(&instance, window.raw_surface())?;

    Ok(Context {
      window,
      device,
      queues,
    })
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum CreationError {
    CreateWindowFailed(err: window::CreationError) {
      display("Could not create window: {}", err)
      from()
    }
    CreateDeviceFailed(err: device::CreationError) {
      display("Could not create device: {}", err)
      from()
    }
  }
}
