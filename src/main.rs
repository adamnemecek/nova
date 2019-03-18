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
    fonts::write(&app.resources)
      .create(include_bytes!("fonts/fira_sans_regular.ttf"))
      .unwrap();

    // Load a background image.
    let bg_image = images::write(&app.resources).load_asset_at_path(&"/do-it.jpg".into(), &assets);

    // Add a root `Game` element.
    ui::add_to_root(&app.resources, Game { bg_image });

    // Apply the default control map bindings.
    let mut controls = controls::write(&app.resources);
    let control_map_path = "/control_map.toml".into();

    match assets.lookup(&control_map_path) {
      Some(id) => match ControlMap::load_file(assets.fs_path_of(id)) {
        Ok(map) => controls.apply_bindings(&map),

        Err(err) => {
          log
            .error("Could not load control map.")
            .with("err", log::Display(err));
        }
      },

      None => {
        log
          .warn("No control map found. No input controls will be bound.")
          .with("path", control_map_path);
      }
    };
  }

  // Run the app until the window is closed.
  app.run();

  Ok(())
}
