use nova::graphics;
use nova::graphics::rendering;
use nova::graphics::{Mesh, Vertex};
use nova::math::algebra::*;
use std::iter;

/// Main entry point of the program.
pub fn main() {
  let sink = bflog::LogSink::new(
    std::io::stdout(),
    bflog::Format::Modern,
    bflog::LevelFilter::Trace,
  );

  let mut log = bflog::Logger::new(&sink).with_src("game");

  let mut events_loop = winit::EventsLoop::new();
  let window = winit::Window::new(&events_loop).expect("could not create window");

  let gfx_device = rendering::init(&window).expect("could not initialize rendering");

  let render_pass = rendering::RenderPass::new(&gfx_device);

  let shaders = rendering::PipelineShaderSet::load_defaults(&gfx_device);

  let descriptor_set_layout = rendering::DescriptorSetLayout::new()
    .texture()
    .create(&gfx_device);

  let pipeline = rendering::Pipeline::new()
    .render_pass(&render_pass)
    .shaders(shaders)
    .vertex_buffer::<graphics::Vertex>()
    .push_constant::<graphics::Color>()
    .push_constant::<Matrix4<f32>>()
    .descriptor_set_layout(&descriptor_set_layout)
    .create();

  log.trace("Created quad mesh.");

  let mut renderer = rendering::Renderer::new(&gfx_device);
  let mut swapchain = rendering::Swapchain::new(&gfx_device);

  let command_pool = rendering::CommandPool::new(&gfx_device, gfx_device.queues().graphics());

  let quad = Mesh::new(
    &gfx_device,
    &[
      Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 0.0]),
      Vertex::new([0.5, -0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 0.0]),
      Vertex::new([0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [0.0, 1.0]),
      Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0, 1.0], [1.0, 1.0]),
    ],
    &[0, 1, 2, 2, 3, 0],
  );

  let mut texture_loader = rendering::TextureLoader::new(&gfx_device);

  let texture = texture_loader.load(
    &image::load_from_memory(include_bytes!("../assets/do-it.jpg"))
      .expect("could not load texture")
      .to_rgba(),
  );

  let sampler = rendering::TextureSampler::new(&gfx_device);

  let descriptor_pool = rendering::DescriptorPool::new(&descriptor_set_layout, 1);

  let descriptor_set = rendering::DescriptorSet::new(
    &descriptor_pool,
    &[rendering::Descriptor::SampledTexture(&texture, &sampler)],
  );

  loop {
    let mut close_requested = false;
    let mut resized = false;

    events_loop.poll_events(|event| match event {
      winit::Event::WindowEvent { event, .. } => match event {
        winit::WindowEvent::CloseRequested => {
          log.info("Close requested.");

          close_requested = true;
        }

        winit::WindowEvent::Resized(_) => {
          resized = true;
        }

        _ => {}
      },

      _ => {}
    });

    if close_requested {
      break;
    }

    if resized || swapchain.is_destroyed() {
      let size = window
        .get_inner_size()
        .expect("window destroyed")
        .to_physical(window.get_hidpi_factor());

      let width = size.width.round() as u32;
      let height = size.height.round() as u32;

      swapchain.create(&render_pass, width, height);

      log
        .info("Created swapchain.")
        .with("width", &width)
        .with("height", &height);
    }

    let result = renderer.render(&mut swapchain, |framebuffer| {
      let mut cmd = rendering::CommandBuffer::new(&command_pool);

      cmd.begin();

      cmd.begin_pass(&render_pass, &framebuffer);

      cmd.bind_pipeline(&pipeline);

      cmd.push_constant(0, &graphics::Color([1.0, 1.0, 1.0, 1.0]));
      cmd.push_constant(1, &Matrix4::<f32>::identity());

      cmd.bind_descriptor_set(0, &descriptor_set);
      cmd.bind_vertex_buffer(0, quad.vertex_buffer());
      cmd.bind_index_buffer(quad.index_buffer());
      cmd.draw_indexed(quad.indices());

      cmd.finish();

      iter::once(cmd)
    });

    match result {
      Err(rendering::RenderError::SwapchainOutOfDate) => {
        swapchain.destroy();
      }

      Err(rendering::RenderError::SurfaceLost) => {
        panic!("surface lost");
      }

      Ok(_) => {}
    }
  }
}

/*
fn init(ctx: &mut engine::Context) {
  let parent = panels::create_panel(ctx);

  engine::edit_component(ctx, parent, |style: &mut panels::Style| {
    style.background = panels::Background::Solid;
    style.color = graphics::Color([0.8, 0.6, 0.6, 1.0]);
  });

  panels::add_to_root(ctx, parent);

  let child = panels::create_panel(ctx);

  engine::edit_component(ctx, child, |style: &mut panels::Style| {
    style.background = panels::Background::Solid;
    style.color = graphics::Color([0.6, 0.6, 0.8, 1.0]);
  });

  engine::edit_component(ctx, child, |layout: &mut panels::Layout| {
    layout.width = panels::Dimension::Fixed(500.0);
    layout.height = panels::Dimension::Fixed(500.0);
    layout.right = panels::Dimension::Fixed(100.0);
    layout.bottom = panels::Dimension::Fixed(100.0);
  });

  panels::set_parent(ctx, child, Some(parent));
}

*/
/*
fn init(ctx: &mut engine::Context) {
  // Load actor templates.
  let hero_template =
    assets::load(ctx, &assets::PathBuf::from("hero-f/actor.yml")).expect("could not load hero");

  let monster_template = assets::load(ctx, &assets::PathBuf::from("004-fire-salamander/actor.yml"))
    .expect("could not load monster");

  // Create actor entities.
  let hero = stage::actors::build_entity(
    Arc::new(hero_template),
    stage::graphics::actors::build_entity(engine::build_entity(ctx)),
  ).build();

  let _monster = stage::actors::build_entity(
    Arc::new(monster_template),
    stage::graphics::actors::build_entity(engine::build_entity(ctx)),
  ).with(stage::Position {
    point: Point3::new(32.0, 24.0, 0.0),
  }).build();

  // Set the camera target to the hero.
  stage::graphics::set_camera_target(ctx, hero);

  // Set the hero to be input controlled.
  stage::actors::driving::drive(ctx, hero);

  // Load custom input mapping.
  if let Ok(mapping) = assets::load(ctx, &assets::PathBuf::from("input-mapping.yml")) {
    input::set_mapping(ctx, mapping);
  }

  {
    let image = Arc::new(
      assets::load::<graphics::Image>(ctx, &assets::PathBuf::from("solid-white.png"))
        .expect("could not load image"),
    );

    let parent = panels::create_panel(ctx);

    engine::edit_component(ctx, parent, |style: &mut panels::Style| {
      style.background = Some(image.clone());
      style.color = graphics::Color::new(0.8, 0.2, 0.2, 1.0);
    });

    panels::add_to_root(ctx, parent);

    let child = panels::create_panel(ctx);

    engine::edit_component(ctx, child, |style: &mut panels::Style| {
      style.background = Some(image.clone());
      style.color = graphics::Color::new(0.2, 0.2, 0.8, 1.0);

      style.set_custom_draw(
        move |_: &mut engine::Context, canvas: &mut graphics::Canvas, _: &Rect<f32>| {
          canvas.draw(&image, graphics::DrawParams::default());
        },
      );
    });

    engine::edit_component(ctx, child, |layout: &mut panels::Layout| {
      layout.width = panels::Dimension::Fixed(100.0);
      layout.left = panels::Dimension::Auto;
      layout.right = panels::Dimension::Fixed(0.0);
    });

    panels::set_parent(ctx, child, Some(parent));
  }
}
*/
