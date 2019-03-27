// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{WindowEvent, WriteWindow};
use nova_core::resources::Resources;
use nova_core::systems::System;
use nova_graphics::gpu::queues::{GpuQueueId, WriteGpuQueues};
use nova_graphics::gpu::{self, ReadGpu};
use nova_graphics::images::WriteImages;
use winit::EventsLoop;

#[derive(Debug)]
pub struct UpdateWindow {
  pub(crate) events_loop: EventsLoop,
}

impl<'a> System<'a> for UpdateWindow {
  type Data = (
    ReadGpu<'a>,
    WriteGpuQueues<'a>,
    WriteImages<'a>,
    WriteWindow<'a>,
  );

  fn run(&mut self, (gpu, mut queues, mut images, mut window): Self::Data) {
    if let Some(backbuffer) = window.backbuffer.take() {
      if let Some(surface) = window.surface.as_ref() {
        if let Some(swapchain) = window.swapchain.as_mut() {
          queues[surface.present_queue_id()].present(swapchain, backbuffer);
        }
      }
    }

    let mut resized = false;

    self.events_loop.poll_events(|event| {
      if let winit::Event::WindowEvent { event, .. } = event {
        match event {
          WindowEvent::Resized(_) => {
            resized = true;
          }

          WindowEvent::CloseRequested => {
            window.close_requested = true;
          }

          _ => {}
        };

        window.events.single_write(event);
      }
    });

    if resized {
      window.refresh_size();
      window.destroy_swapchain(&gpu, &mut images);
    }

    window.create_swapchain(&gpu, &mut images);
  }
}
