use bflog;

mod backend;
mod commands;
mod context;
mod pipeline;
mod render_pass;
mod render_target;
mod renderer;
mod shader;

use self::backend::Backend;
use self::context::Context;
use self::pipeline::{Pipeline, PipelineShaders};
use self::render_pass::RenderPass;
use self::render_target::RenderTarget;
use self::shader::Shader;

pub fn main() {
  let sink = bflog::LogSink::new(
    std::io::stdout(),
    bflog::Format::Modern,
    bflog::LevelFilter::Trace,
  );

  let mut log = bflog::Logger::new(&sink);

  let mut events_loop = winit::EventsLoop::new();

  let window = winit::WindowBuilder::new()
    .with_title("nova")
    .build(&events_loop)
    .expect("could not create window");

  let window_size = window
    .get_inner_size()
    .expect("window was destroyed")
    .to_physical(window.get_hidpi_factor());

  let context = Context::new(&window, &log);

  let render_pass = RenderPass::new(&context);

  let _render_target = RenderTarget::new(
    &render_pass,
    window_size.width.round() as u32,
    window_size.height.round() as u32,
    3,
  );

  let _pipeline = Pipeline::new(
    &render_pass,
    PipelineShaders {
      vertex: Shader::new(&context, include_bytes!("shaders/spirv/default.vert.spv")),
      fragment: Shader::new(&context, include_bytes!("shaders/spirv/default.frag.spv")),
    },
  );

  events_loop.run_forever(|event| {
    match event {
      winit::Event::WindowEvent { event, .. } => match event {
        winit::WindowEvent::CloseRequested { .. } => {
          log.info("Close requested.");

          return winit::ControlFlow::Break;
        }

        _ => {}
      },

      _ => {}
    };

    winit::ControlFlow::Continue
  });
}
