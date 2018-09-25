use gfx_hal::{
  command::{ClearColor, ClearValue},
  format::{Aspects, ChannelType, Format, Swizzle},
  image::{Access, Layout, SubresourceRange, ViewKind},
  pass::{
    Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
    SubpassDesc, SubpassRef,
  },
  pool::CommandPoolCreateFlags,
  pso::{
    BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
    PipelineStage, Rasterizer, Rect, Viewport,
  },
  queue::Submission,
  window::PresentMode,
  Backbuffer, Device, FrameSync, Graphics, Instance, Primitive, Surface, SwapImageIndex, Swapchain,
  SwapchainConfig,
};
use winit;

mod gfx_back;
mod pipeline;
mod render_pass;
mod shaders;
mod swapchain;

use self::pipeline::GraphicsPipeline;
use self::render_pass::RenderPass;
use self::shaders::Shaders;
use self::swapchain::SwapchainContext;

/// State of the graphics engine.
//
// Because Rust fields are dropped in the order they are declared, fields
// should be declared in the REVERSE order that resources are created in.
pub struct Context {
  /// Current render context created by the engine.
  render: Option<RenderContext>,
  /// Default shader modules.
  shaders: Shaders,
  frame_fence: Option<gfx_back::Fence>,
  frame_semaphore: Option<gfx_back::Semaphore>,
  /// Pool of command queues used by the engine.
  command_pool: Option<gfx_hal::CommandPool<gfx_back::Backend, gfx_hal::Graphics>>,
  /// Group of command queues used by the engine.
  queue_group: gfx_hal::QueueGroup<gfx_back::Backend, gfx_hal::Graphics>,
  /// Logical device for the graphics API.
  device: gfx_back::Device,
  /// Physical device adapter used by the engine.
  adapter: gfx_hal::Adapter<gfx_back::Backend>,
  /// Rendering surface used by the engine.
  surface: gfx_back::Surface,
  /// Instance of the graphics API backend used by the engine.
  instance: gfx_back::Instance,
}

struct RenderContext {
  pipeline: GraphicsPipeline,
  swapchain: SwapchainContext,
  render_pass: RenderPass,
}

impl Context {
  /// Creates a new context for drawing on the given window.
  pub fn new(window: &winit::Window) -> Context {
    use gfx_hal::Instance;

    // Create an instance of the backend.
    //
    // The app name and version params are not important. Not _yet_ anyway…
    let instance = gfx_back::Instance::create("nova", 1);

    // Create a surface from the window.
    let surface = instance.create_surface(&window);

    // Find the adapter (graphics card) to render with.
    let mut adapter = {
      // Get a list of available adapters.
      let mut adapters = instance.enumerate_adapters();

      // Take the first available adapter.
      //
      // TODO: Find the best available adapter.
      adapters.remove(0)
    };

    // Open a logical device and a graphics queue group supported by the
    // surface.
    //
    // The queue group contains pools of queues that will be used to
    // send commands to the card.

    const QUEUE_COUNT: usize = 1;

    let (device, queue_group) = adapter
      .open_with(QUEUE_COUNT, |family| surface.supports_queue_family(family))
      .expect("could not open device");

    // Create a command pool to get graphics command queues from.

    const MAX_QUEUES: usize = 16;

    let command_pool = device.create_command_pool_typed(
      &queue_group,
      gfx_hal::pool::CommandPoolCreateFlags::empty(),
      MAX_QUEUES,
    );

    let frame_semaphore = device.create_semaphore();
    let frame_fence = device.create_fence(false);

    // Load default shaders.
    let shaders = shaders::create(&device);

    // Return the completed context.
    Context {
      instance,
      surface,
      adapter,
      device,
      queue_group,
      command_pool: Some(command_pool),
      frame_semaphore: Some(frame_semaphore),
      frame_fence: Some(frame_fence),
      shaders,
      render: None,
    }
  }
}

// Implement `Drop` to destroy resources.
impl Drop for Context {
  fn drop(&mut self) {
    // Destroy the current render context.
    if let Some(render) = self.render.take() {
      render.destroy(self);
    }

    if let Some(frame_fence) = self.frame_fence.take() {
      self.device.destroy_fence(frame_fence);
    }

    if let Some(frame_semaphore) = self.frame_semaphore.take() {
      self.device.destroy_semaphore(frame_semaphore);
    }

    // Destroy the command pool.
    if let Some(pool) = self.command_pool.take() {
      self.device.destroy_command_pool(pool.into_raw());
    }
  }
}

impl RenderContext {
  pub fn new(ctx: &mut Context) -> RenderContext {
    // Get the available capabilities, color formats, and present modes.
    let (surface_caps, surface_formats, present_modes) =
      ctx.surface.compatibility(&ctx.adapter.physical_device);

    // Find a good SRGB color format, preferably `Rgba8Srgb`, or default to the
    // first available format.
    let surface_format = select_surface_format(surface_formats);

    // Select best available present mode.
    let mode = select_present_mode(present_modes);

    // Create a render pass.
    let render_pass = render_pass::create(ctx, surface_format);

    // Create a swapchain context.
    let swapchain = swapchain::create(
      &ctx.device,
      &mut ctx.surface,
      &surface_caps,
      surface_format,
      &render_pass,
    );

    // Create the graphics pipeline.
    let pipeline = pipeline::create(&ctx.device, &ctx.shaders, &render_pass);

    // Return the completed `RenderContext`.
    RenderContext {
      render_pass,
      swapchain,
      pipeline,
    }
  }

  pub fn destroy(self, ctx: &mut Context) {
    ctx.device.destroy_render_pass(self.render_pass);

    swapchain::destroy(&ctx.device, self.swapchain);
  }
}

pub fn main() {
  // Create an events loop. This is neither `Send` nor `Sync`.
  let mut events_loop = winit::EventsLoop::new();

  // Create a window. This probably only looks good because of my tiling WM.
  let window = winit::WindowBuilder::new()
    .with_title("nova")
    .build(&events_loop)
    .unwrap();

  // Create the graphics context.
  let mut ctx = Context::new(&window);

  // Create the initial render context.
  ctx.render = Some(RenderContext::new(&mut ctx));

  // Run until close is requested.
  let mut running = true;

  while running {
    events_loop.poll_events(|event| match event {
      winit::Event::WindowEvent { event, .. } => match event {
        winit::WindowEvent::CloseRequested => {
          running = false;
        }

        _ => {}
      },

      _ => {}
    });

    let frame_fence = ctx.frame_fence.as_ref().expect("frame fence vanished");
    let frame_semaphore = ctx.frame_semaphore.as_ref().expect("frame fence vanished");
    let command_pool = ctx.command_pool.as_mut().expect("command pool vanished");
    let render = ctx.render.as_mut().expect("render_ctx vanished");

    ctx.device.reset_fence(frame_fence);
    command_pool.reset();

    // A swapchain contains multiple images - which one should we draw on? This
    // returns the index of the image we'll use. The image may not be ready for
    // rendering yet, but will signal frame_semaphore when it is.
    let frame_index = swapchain::acquire_frame(&mut render.swapchain, frame_semaphore);

    // We have to build a command buffer before we send it off to draw.
    // We don't technically have to do this every frame, but if it needs to
    // change every frame, then we do.
    let finished_command_buffer = {
      let mut command_buffer = command_pool.acquire_command_buffer(false);

      // Define a rectangle on screen to draw into.
      // In this case, the whole screen.
      let viewport = Viewport {
        rect: Rect {
          x: 0,
          y: 0,
          w: render.swapchain.extent.width as i16,
          h: render.swapchain.extent.height as i16,
        },
        depth: 0.0..1.0,
      };

      command_buffer.set_viewports(0, &[viewport.clone()]);
      command_buffer.set_scissors(0, &[viewport.rect]);

      // Choose a pipeline to use.
      command_buffer.bind_graphics_pipeline(&render.pipeline);

      {
        // Clear the screen and begin the render pass.
        let mut encoder = command_buffer.begin_render_pass_inline(
          &render.render_pass,
          &render.swapchain.framebuffers[frame_index as usize],
          viewport.rect,
          &[ClearValue::Color(ClearColor::Float([1.0, 0.0, 0.0, 1.0]))],
        );

        // Draw some geometry! In this case 0..3 means that we're drawing
        // the range of vertices from 0 to 3. We have no vertex buffer so
        // this really just tells our shader to draw one triangle. The
        // specific vertices to draw are encoded in the vertex shader which
        // you can see in `source_assets/shaders/part00.vert`.
        //
        // The 0..1 is the range of instances to draw. It's not relevant
        // unless you're using instanced rendering.
        encoder.draw(0..3, 0..1);
      }

      // Finish building the command buffer - it's now ready to send to the
      // GPU.
      command_buffer.finish()
    };

    // This is what we submit to the command queue. We wait until frame_semaphore
    // is signalled, at which point we know our chosen image is available to draw
    // on.
    let submission = Submission::new()
      .wait_on(&[(frame_semaphore, PipelineStage::BOTTOM_OF_PIPE)])
      .submit(vec![finished_command_buffer]);

    // We submit the submission to one of our command queues, which will signal
    // frame_fence once rendering is completed.
    ctx.queue_group.queues[0].submit(submission, Some(&frame_fence));

    // We first wait for the rendering to complete...
    ctx.device.wait_for_fence(&frame_fence, !0);

    // ...and then present the image on screen!
    swapchain::present(
      &mut render.swapchain,
      &mut ctx.queue_group.queues[0],
      frame_index,
    );
  }
}

fn select_surface_format(formats: Option<Vec<Format>>) -> Format {
  formats.map_or(Format::Rgba8Srgb, |formats| {
    formats
      .iter()
      .find(|format| format.base_format().1 == ChannelType::Srgb)
      .map(|format| *format)
      .unwrap_or(formats[0])
  })
}

fn select_present_mode(modes: Vec<PresentMode>) -> PresentMode {
  // `Fifo` is regular, blocking vsync. It's always available but caps the
  // speed of the engine.
  let mut best = PresentMode::Fifo;

  for mode in modes {
    // `Mailbox` allows the engine to run as fast as possible but prevents
    // screen tearing with at least one extra buffer (triple-buffering).
    if mode == PresentMode::Mailbox {
      return mode;
    }

    // `Immediate` allows the engine to run as fast as possible and does not
    // prevent screen tearing. Apparently FIFO support isn't great however so
    // immediate is preferred.
    //
    // TODO: Verify that this is true (it was about Vulkan).
    if mode == PresentMode::Immediate {
      best = mode;
    }
  }

  return best;
}

/*
impl SwapchainContext {
  fn new(ctx: &mut Context) -> Self {
  }
}

fn main() {
  let mut events_loop = EventsLoop::new();
  let window = WindowBuilder::new()
    .with_title("Part 01: Resizing")
    .with_dimensions((256, 256).into())
    .build(&events_loop)
    .unwrap();

  let mut ctx = Context::new(&window);

  let physical_device = &ctx.adapter.physical_device;

  // We don't need the capabilities just yet, since we use that for the swapchain.
  let (_caps, formats, _) = ctx.surface.compatibility(physical_device);

  // This could theoretically change between swapchain creations, but we're going
  // to ignore that for now so that we only have to build our render pass and
  // pipeline once.
  let surface_color_format = {
    match formats {
      Some(choices) => choices
        .into_iter()
        .find(|format| format.base_format().1 == ChannelType::Srgb)
        .unwrap(),
      None => Format::Rgba8Srgb,
    }
  };

  let render_pass = {
    let color_attachment = Attachment {
      format: Some(surface_color_format),
      samples: 1,
      ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
      stencil_ops: AttachmentOps::DONT_CARE,
      layouts: Layout::Undefined..Layout::Present,
    };

    let subpass = SubpassDesc {
      colors: &[(0, Layout::ColorAttachmentOptimal)],
      depth_stencil: None,
      inputs: &[],
      resolves: &[],
      preserves: &[],
    };

    let dependency = SubpassDependency {
      passes: SubpassRef::External..SubpassRef::Pass(0),
      stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
      accesses: Access::empty()..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
    };

    ctx
      .device
      .create_render_pass(&[color_attachment], &[subpass], &[dependency])
  };

  let pipeline_layout = ctx.device.create_pipeline_layout(&[], &[]);

  let vertex_shader_module = {
    let spirv = include_bytes!("../assets/shader.vert.spv");
    ctx.device.create_shader_module(spirv).unwrap()
  };

  let fragment_shader_module = {
    let spirv = include_bytes!("../assets/shader.frag.spv");
    ctx.device.create_shader_module(spirv).unwrap()
  };

  let pipeline = {
    let vs_entry = EntryPoint::<backend::Backend> {
      entry: "main",
      module: &vertex_shader_module,
      specialization: Default::default(),
    };

    let fs_entry = EntryPoint::<backend::Backend> {
      entry: "main",
      module: &fragment_shader_module,
      specialization: Default::default(),
    };

    let shader_entries = GraphicsShaderSet {
      vertex: vs_entry,
      hull: None,
      domain: None,
      geometry: None,
      fragment: Some(fs_entry),
    };

    let subpass = Subpass {
      index: 0,
      main_pass: &render_pass,
    };

    let mut pipeline_desc = GraphicsPipelineDesc::new(
      shader_entries,
      Primitive::TriangleList,
      Rasterizer::FILL,
      &pipeline_layout,
      subpass,
    );

    pipeline_desc
      .blender
      .targets
      .push(ColorBlendDesc(ColorMask::ALL, BlendState::ALPHA));

    ctx
      .device
      .create_graphics_pipeline(&pipeline_desc, None)
      .unwrap()
  };

  let frame_semaphore = ctx.device.create_semaphore();
  let frame_fence = ctx.device.create_fence(false);

  // We're going to defer the construction of our swapchain, extent, image views,
  // and framebuffers until the mainloop, because we will need to repeat it
  // whenever the window resizes. For now we leave them empty.
  //
  // We're using an Option containing a tuple so that we can drop all four items
  // together. We're also taking advantage of type inference as much as possible
  // so we don't have to know the specific type names just yet.
  let mut swapchain_stuff: Option<(_, _, _, _)> = None;

  let mut rebuild_swapchain = false;

  loop {
    let mut quitting = false;

    events_loop.poll_events(|event| {
      if let Event::WindowEvent { event, .. } = event {
        match event {
          WindowEvent::CloseRequested => quitting = true,
          WindowEvent::KeyboardInput {
            input:
              KeyboardInput {
                virtual_keycode: Some(VirtualKeyCode::Escape),
                ..
              },
            ..
          } => quitting = true,

          // We need to recreate our swapchain if we resize, so we'll set
          // a flag when that happens.
          WindowEvent::Resized(_) => {
            rebuild_swapchain = true;
          }

          _ => {}
        }
      }
    });

    // We need to destroy things if we're resizing because we'll recreate them.
    // We also need to destroy them if we're quitting, so we can clean them up.
    if (rebuild_swapchain || quitting) && swapchain_stuff.is_some() {
      // Take ownership over the old stuff so we can destroy it.
      // The value of swapchain_stuff is now `None`.
      let (swapchain, _extent, frame_views, framebuffers) = swapchain_stuff.take().unwrap();

      // We want to wait for all queues to be idle and reset the command pool,
      // so that we know that no commands are being executed while we destroy
      // the swapchain.
      ctx.device.wait_idle().unwrap();
      command_pool.reset();

      // Destroy all the old stuff.
      for framebuffer in framebuffers {
        ctx.device.destroy_framebuffer(framebuffer);
      }

      for image_view in frame_views {
        ctx.device.destroy_image_view(image_view);
      }

      ctx.device.destroy_swapchain(swapchain);
    }

    if quitting {
      // At this point, our swapchain is destroyed and will not be recreated.
      break;
    }

    // If we don't have a swapchain here, we destroyed it and we need to
    // recreate it.
    if swapchain_stuff.is_none() {
      rebuild_swapchain = false;
      let (caps, _, _) = ctx.surface.compatibility(physical_device);

      // Here we just create the swapchain, image views, and framebuffers
      // like we did in part 00, and store them in swapchain_stuff.
      let swap_config = SwapchainConfig::from_caps(&caps, surface_color_format);
      let extent = swap_config.extent.to_extent();
      let (swapchain, backbuffer) =
        ctx
          .device
          .create_swapchain(&mut ctx.surface, swap_config, None);

      let (frame_views, framebuffers) = match backbuffer {
        Backbuffer::Images(images) => {
          let color_range = SubresourceRange {
            aspects: Aspects::COLOR,
            levels: 0..1,
            layers: 0..1,
          };

          let image_views = images
            .iter()
            .map(|image| {
              ctx
                .device
                .create_image_view(
                  image,
                  ViewKind::D2,
                  surface_color_format,
                  Swizzle::NO,
                  color_range.clone(),
                ).unwrap()
            }).collect::<Vec<_>>();

          let fbos = image_views
            .iter()
            .map(|image_view| {
              ctx
                .device
                .create_framebuffer(&render_pass, vec![image_view], extent)
                .unwrap()
            }).collect();

          (image_views, fbos)
        }
        Backbuffer::Framebuffer(fbo) => (Vec::new(), vec![fbo]),
      };

      // Store the new stuff.
      swapchain_stuff = Some((swapchain, extent, frame_views, framebuffers));
    }

    // To access the swapchain, we need to get a mutable reference to the
    // contents of swapchain_stuff. We know it's safe to unwrap because we just
    // checked it wasn't `None`.
    let (swapchain, extent, _frame_views, framebuffers) = swapchain_stuff.as_mut().unwrap();

    // The rest of our rendering happens exactly the same way as before.

    ctx.device.reset_fence(&frame_fence);
    command_pool.reset();

    let frame_index: SwapImageIndex = {
      match swapchain.acquire_image(!0, FrameSync::Semaphore(&frame_semaphore)) {
        Ok(i) => i,
        Err(_) => {
          rebuild_swapchain = true;
          continue;
        }
      }
    };

    let finished_command_buffer = {
      let mut command_buffer = command_pool.acquire_command_buffer(false);

      let viewport = Viewport {
        rect: Rect {
          x: 0,
          y: 0,
          w: extent.width as i16,
          h: extent.height as i16,
        },
        depth: 0.0..1.0,
      };

      command_buffer.set_viewports(0, &[viewport.clone()]);
      command_buffer.set_scissors(0, &[viewport.rect]);

      command_buffer.bind_graphics_pipeline(&pipeline);

      {
        let mut encoder = command_buffer.begin_render_pass_inline(
          &render_pass,
          &framebuffers[frame_index as usize],
          viewport.rect,
          &[ClearValue::Color(ClearColor::Float([0.0, 0.0, 0.0, 1.0]))],
        );

        encoder.draw(0..3, 0..1);
      }

      command_buffer.finish()
    };

    let submission = Submission::new()
      .wait_on(&[(&frame_semaphore, PipelineStage::BOTTOM_OF_PIPE)])
      .submit(vec![finished_command_buffer]);

    ctx.queue_group.queues[0].submit(submission, Some(&frame_fence));

    ctx.device.wait_for_fence(&frame_fence, !0);

    let result = swapchain.present(&mut ctx.queue_group.queues[0], frame_index, &[]);

    if result.is_err() {
      rebuild_swapchain = true;
    }

    // Return the command pool to the ctx.
    ctx.command_pool = Some(command_pool);
  }

  // Cleanup
  // Note that we don't have to destroy the swapchain, frame images, or
  // framebuffers, because they will already have been destroyed before breaking
  // out of the mainloop.
  let command_pool = ctx.command_pool.take().expect("command_pool went missing");

  ctx.device.destroy_graphics_pipeline(pipeline);
  ctx.device.destroy_pipeline_layout(pipeline_layout);
  ctx.device.destroy_render_pass(render_pass);
  ctx.device.destroy_shader_module(vertex_shader_module);
  ctx.device.destroy_shader_module(fragment_shader_module);
  ctx.device.destroy_command_pool(command_pool.into_raw());
  ctx.device.destroy_fence(frame_fence);
  ctx.device.destroy_semaphore(frame_semaphore);
}

*/
