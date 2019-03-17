use nova::assets;
use nova::graphics::images::{self, ImageId};
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

  // Load a default font.
  fonts::write(&app.res)
    .create(include_bytes!("fonts/fira_sans_regular.ttf"))
    .unwrap();

  // Load a background image.
  let bg_image =
    images::write(&app.res).load_asset_at_path(&"/do-it.jpg".into(), &assets::read(&app.res));

  // Add a root `Game` element.
  ui::add_to_root(&app.res, Game { bg_image });

  // Test message delivery.
  ui::nodes::build(&app.res);
  ui::messages::write(&app.res).broadcast(TestMessage("Broadcasted!"));

  // Run the app until the window is closed.
  app.run();

  Ok(())
}
