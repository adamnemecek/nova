// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub(crate) use gfx_hal::Surface as HalSurfaceExt;
pub(crate) use gfx_hal::SurfaceCapabilities;

use crate::backend::Backend;
use crate::gpu::queues::QueueId;
use crate::gpu::{self, Gpu};
use nova_core::quick_error;
use nova_core::resources::Resources;
use winit::Window;

pub(crate) type HalSurface = <Backend as gfx_hal::Backend>::Surface;

pub struct Surface {
  surface: HalSurface,
  present_queue_id: QueueId,
}

impl Surface {
  pub fn new(res: &Resources, window: &Window) -> Result<Self, CreateSurfaceError> {
    let gpu = gpu::borrow(res);

    let surface = gpu.backend.create_surface(window);

    let present_queue_id = gpu::queues::borrow(res)
      .find_present_queue(&surface)
      .ok_or(CreateSurfaceError::PresentNotSupported)?;

    Ok(Self {
      surface,
      present_queue_id,
    })
  }

  pub fn capabilities(&self, gpu: &Gpu) -> SurfaceCapabilities {
    let (caps, _, _, _) = self.surface.compatibility(&gpu.adapter.physical_device);

    caps
  }

  pub(crate) fn as_hal_mut(&mut self) -> &mut HalSurface {
    &mut self.surface
  }
}

quick_error! {
  #[derive(Debug)]
  pub enum CreateSurfaceError {
    PresentNotSupported {
      display("the graphics device does not support presentation to this window")
    }
  }
}
