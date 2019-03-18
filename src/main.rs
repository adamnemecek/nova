use nova::assets;
use nova::graphics::images::{self, ImageId};
use nova::input::controls::{self, ControlMap};
use nova::log;
use nova::ui;
use nova::ui::text::fonts;
use nova::ui::{Align, Color, HorizontalAlign, Image, Text, VerticalAlign};

#[derive(Debug, PartialEq)]
struct Game {
  bg_image: ImageId,
}

impl Game {
  fn on_test_message(&self, _: ui::ElementContext<Self>, message: TestMessage) {
    println!("{}", message.0);
  }
}

impl ui::Element for Game {
  type State = ();

  fn on_awake(&self, mut ctx: ui::ElementContext<Self>) {
    ctx.subscribe(Self::on_test_message);
  }

  fn build(&self, _: ui::ChildSpecs, _: ui::ElementContext<Self>) -> ui::Spec {
    ui::Spec::from(vec![
      ui::Spec::new(
        Align(HorizontalAlign::Left, VerticalAlign::Bottom),
        Image::new(self.bg_image),
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
    let assets = &assets::read(&app.resources);

    // Load a default font.
    let default_font_path = "/fonts/fira_sans_regular.ttf".into();

    match assets.lookup(&default_font_path) {
      Some(id) => {
        if let Err(err) = fonts::write(&app.resources).load_asset(id, assets) {
          log
            .error("Could not load default font.")
            .with("path", &default_font_path)
            .with("err", log::Display(err));
        }
      }

      None => {
        log
          .warn("Default font does not exist.")
          .with("path", &default_font_path);
      }
    }

    // Apply the default control map bindings.
    let control_map_path = "/control_map.toml".into();

    match assets.lookup(&control_map_path) {
      Some(id) => match ControlMap::load_file(assets.fs_path_of(id)) {
        Ok(map) => controls::write(&app.resources).apply_bindings(&map),

        Err(err) => {
          log
            .error("Could not load control map.")
            .with("path", &control_map_path)
            .with("err", log::Display(err));
        }
      },

      None => {
        log
          .warn("Control map does not exist.")
          .with("path", control_map_path);
      }
    };

    // Load "/do_it.jpg" and display it on the root `Game` UI element.
    let bg_image_path = "/do_it.jpg".into();

    match assets.lookup(&bg_image_path) {
      Some(id) => {
        let bg_image = images::write(&app.resources).load_asset(id, assets);

        ui::add_to_root(&app.resources, Game { bg_image });
      }

      None => {
        log
          .error("Background image does not exist.")
          .with("path", bg_image_path);
      }
    };
  }

  // Run the app until the window is closed.
  app.run();

  Ok(())
}
