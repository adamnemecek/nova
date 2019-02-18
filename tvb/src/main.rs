extern crate nova;

use nova::el;
use nova::engine;
use nova::log;
use nova::renderer::Renderer;
use nova::ui;
use nova::window;

#[derive(Debug, Default, PartialEq)]
struct App;

impl el::Element for App {
  type State = ();
  type Message = ();

  fn build(&self, _: el::spec::Children, _: el::Context<Self>) -> el::Spec {
    el::spec(
      ui::Div {
        layout: ui::Layout {
          x: 100.0,
          y: 100.0,
          width: 1000.0,
          height: 500.0,
        },
        style: ui::Style {
          background: ui::Color::new(0.2, 0.2, 0.8, 1.0),
        },
      },
      None,
    )
  }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Set up log macros to use nova logging.
  log::set_as_default();

  // Create a new nova engine instance.
  let mut engine = nova::Engine::new();

  ui::setup(&mut engine);
  window::setup(&mut engine, Default::default());

  // Create a renderer.
  let mut renderer = Renderer::new(&engine.resources().fetch());
  let mut ui_painter = ui::Painter::new(&renderer);

  // Render at the end of each frame.
  engine.on_event_fn(engine::Event::TickEnding, move |res, _| {
    let cmd = renderer.begin();

    ui_painter.draw(cmd.into(), res);

    renderer.finish();
  });

  // Add an `App` element and run the engine until exit.
  engine.add_element(App);
  engine.run();

  Ok(())
}
