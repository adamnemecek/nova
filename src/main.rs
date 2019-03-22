use nova::graphics::images::ImageData;
use nova::input::controls::{self, ControlMap};
use nova::log;
use nova::ui;
use nova::ui::text::fonts;
use nova::ui::{Align, Color, HorizontalAlign, Image, Text, VerticalAlign};

#[derive(Debug, PartialEq)]
struct Game {
  bg_image: ImageData,
}

impl ui::Element for Game {
  type State = ();

  fn build(&self, _: ui::ChildSpecs, _: ui::ElementContext<Self>) -> ui::Spec {
    ui::Spec::from(vec![
      ui::Spec::new(
        Align(HorizontalAlign::Left, VerticalAlign::Bottom),
        Image::new(&self.bg_image),
      ),
      ui::Spec::from(Text {
        content: "Hello world.".into(),
        color: Color::WHITE,
        size: 24.0,
        h_align: HorizontalAlign::Right,
        v_align: VerticalAlign::Bottom,
      }),
    ])
  }
}

#[derive(Clone)]
struct TestMessage(&'static str);

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Set up log macros to use nova logging.
  log::set_as_default();

  // Create a new nova app.
  let app = nova::App::new();
  let log = log::Logger::new(module_path!());

  {
    // Load a default font.
    let default_font_path = "assets/fonts/fira_sans_regular.ttf";

    if let Err(err) = fonts::borrow_mut(&app.resources).load_file(default_font_path) {
      log
        .error("Could not load default font.")
        .with("path", default_font_path)
        .with("err", err);
    }

    // Apply the default control map bindings.
    let control_map_path = "assets/control_map.toml";

    match ControlMap::load_file(control_map_path) {
      Ok(map) => controls::borrow_mut(&app.resources).apply_bindings(&map),

      Err(err) => {
        log
          .error("Could not load control map.")
          .with("path", control_map_path)
          .with("err", log::Display(err));
      }
    };

    // Load "do_it.jpg" and display it on the root `Game` UI element.
    let bg_image_path = "assets/do_it.jpg";

    match ImageData::load_file(bg_image_path) {
      Ok(bg_image) => {
        ui::add_to_root(&app.resources, Game { bg_image });
      }

      Err(err) => {
        log
          .error("Could not load background image.")
          .with("path", bg_image_path)
          .with("err", err);
      }
    };
  }

  // Run the app until the window is closed.
  app.run();

  Ok(())
}
