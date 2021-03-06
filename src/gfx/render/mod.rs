// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod framebuffer;
mod pass;
mod pipeline;
mod surface;

pub use self::{framebuffer::*, pass::*, pipeline::*};

use self::surface::*;
use super::*;

/// Renders graphics onto a window on a background thread.
pub struct Renderer {
  messages: channel::Sender<RendererMsg>,
  thread: thread::JoinHandle<()>,
}

/// Control message for a renderer background thread.
enum RendererMsg {
  ShutDown,
  ResizeSurface(Size<f64>),
}

impl Renderer {
  /// Resizes the render surface.
  ///
  /// Call this function each time the window resizes.
  pub fn resize_surface(&self, size: Size<f64>) {
    let _ = self.messages.send(RendererMsg::ResizeSurface(size));
  }

  /// Shuts down the renderer, blocking until the background thread completes.
  pub fn shut_down(self) {
    let _ = self.messages.send(RendererMsg::ShutDown);
    let _ = self.thread.join();
  }
}

/// Starts a renderer on a background thread, returning a `Renderer` to
/// represent it.
pub fn start(
  context: &Context,
  window: &window::Handle,
  loader: &Loader,
  logger: &log::Logger,
) -> Result<Renderer, OutOfMemoryError> {
  let context = context.clone();
  let loader = loader.clone();
  let logger = logger.clone();

  // Create a channel to send and receive control messages.
  let (send_messages, recv_messages) = channel::unbounded();

  // Create resources needed for rendering.
  let graphics_queue_id = context.queues().find_graphics_queue();
  let command_pool = cmd::Pool::new(&context, graphics_queue_id)?;
  let render_pass = Pass::new(&context);
  let mut surface = Surface::new(&context, &window);
  let mut framebuffer = Framebuffer::new(&context);

  framebuffer.set_render_pass(&render_pass);

  let frame_fence = cmd::Fence::new(&context, true)?;
  let acquire_semaphore = cmd::Semaphore::new(&context)?;
  let render_semaphore = cmd::Semaphore::new(&context)?;

  let vertex_shader =
    shader::compile_hlsl(&context, shader::Stage::Vertex, include_str!("./shaders/quad.vert"));

  let fragment_shader =
    shader::compile_hlsl(&context, shader::Stage::Fragment, include_str!("./shaders/color.frag"));

  let pipeline = PipelineBuilder::new()
    .set_render_pass(&render_pass)
    .set_vertex_shader(&vertex_shader.expect("failed to create vertex shader"))
    .set_fragment_shader(&fragment_shader.expect("failed to create fragment shader"))
    .set_push_constants::<Color>()
    .add_vertex_buffer::<Vertex>()
    .add_descriptor_set(&DescriptorLayout::new(
      &context,
      vec![DescriptorKind::UniformBuffer, DescriptorKind::SampledImage],
    )?)
    .build(&context)
    .expect("failed to create graphics pipeline");

  let mut submission = cmd::Submission::new(graphics_queue_id);

  let descriptor_pool = DescriptorPool::new(&pipeline.descriptor_layouts()[0], 1)?;
  let surface_size = surface.size();

  // Spawn the renderer on a background thread.
  let thread = thread::spawn(move || {
    let vertex_buffer = loader
      .load_buffer(
        BufferKind::Vertex,
        vec![
          Vertex((-640.0, -360.0, 0.0), (0.0, 0.0), Color::WHITE),
          Vertex((-640.0, 360.0, 0.0), (0.0, 1.0), Color::WHITE),
          Vertex((640.0, -360.0, 0.0), (1.0, 0.0), Color::WHITE),
          Vertex((640.0, 360.0, 0.0), (1.0, 1.0), Color::WHITE),
        ],
      )
      .recv()
      .expect("failed to load buffer");

    let uniform_buffer = loader
      .load_buffer(
        BufferKind::Uniform,
        vec![Matrix4::new_orthographic(
          -surface_size.width as f32 / 2.0,
          surface_size.width as f32 / 2.0,
          -surface_size.height as f32 / 2.0,
          surface_size.height as f32 / 2.0,
          -1.0,
          1.0,
        )],
      )
      .recv()
      .expect("failed to load uniform buffer");

    let image_data = ImageData::load_file("assets/do_it.jpg").expect("failed to load image data");

    let image =
      loader.load_image(image_data.size(), image_data).recv().expect("failed to load image");

    let sampler = Sampler::new(&context, SamplerFilter::Nearest).expect("failed to create sampler");

    let descriptor_set = DescriptorSet::new(
      &descriptor_pool,
      vec![Descriptor::UniformBuffer(uniform_buffer), Descriptor::SampledImage(image, sampler)],
    )
    .expect("failed to create descriptor set");

    // Try to render at 60 fps maximum.
    time::loop_at_frequency(60.0, |render_loop| {
      // Process incoming messages.
      loop {
        let message = match recv_messages.try_recv() {
          Ok(message) => message,
          Err(channel::TryRecvError::Disconnected) => return render_loop.stop(),
          Err(channel::TryRecvError::Empty) => break,
        };

        match message {
          RendererMsg::ShutDown => return render_loop.stop(),
          RendererMsg::ResizeSurface(size) => surface.resize(size),
        }
      }

      // Wait for the previous frame to finish.
      frame_fence.wait_and_reset();

      // Clear resources used by the previous frame.
      submission.clear();

      // Acquire a backbuffer from the surface.
      let backbuffer = match surface.acquire(&acquire_semaphore) {
        Ok(backbuffer) => backbuffer,

        Err(err) => {
          log::error!(logger, "failed to acquire surface backbuffer";
            "cause" => log::Display(err),
          );

          return render_loop.stop();
        }
      };

      submission.wait_for(&acquire_semaphore, PipelineStage::COLOR_ATTACHMENT_OUTPUT);

      // Attach the backbuffer to the framebuffer.
      framebuffer.set_attachment(backbuffer.image());

      if let Err(err) = framebuffer.ensure_created() {
        log::error!(logger, "failed to create framebuffer";
          "error" => log::Display(err),
        );

        return render_loop.stop();
      }

      // Record rendering commands.
      let mut cmd_list = cmd::List::new(&command_pool);
      let mut cmd = cmd_list.begin();

      cmd.begin_render_pass(&mut framebuffer);

      cmd.bind_pipeline(&pipeline);
      cmd.push_constants(&Color::new(1.0, 1.0, 1.0, 1.0));
      cmd.bind_descriptor_sets(0, iter::once(&descriptor_set));
      cmd.bind_vertex_buffer(0, &vertex_buffer);

      cmd.draw(0..4);

      cmd.finish();

      // Submit rendering commands.
      submission.command_buffers.push(cmd_list);
      submission.signal(&render_semaphore);

      context.queues().submit(&submission, &frame_fence);

      // Present the backbuffer.
      if let Err(err) = backbuffer.present(&[&render_semaphore]) {
        log::error!(logger, "failed to present surface backbuffer";
          "error" => log::Display(err),
        );

        return render_loop.stop();
      }
    });

    // Wait for everything to flush out before shutting down.
    context.wait_idle();
  });

  Ok(Renderer { messages: send_messages, thread })
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex((f32, f32, f32), (f32, f32), Color);

impl vertex::Data for Vertex {
  const ATTRIBUTES: &'static [vertex::Attribute] =
    &[vertex::Attribute::Vector3f32, vertex::Attribute::Vector2f32, vertex::Attribute::Vector4f32];
}
