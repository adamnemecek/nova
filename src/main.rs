use nova::assets;
use nova::el;
use nova::graphics::images::{self, ImageId};
use nova::log;
use nova::ui::text::fonts;
use nova::ui::{Align, Color, HorizontalAlign, Image, Text, VerticalAlign};

#[derive(Debug, PartialEq)]
struct Game {
  bg_image: ImageId,
}

impl el::Element for Game {
  type State = ();
  type Message = ();

  fn build(&self, _: el::spec::Children, _: el::Context<Self>) -> el::Spec {
    el::spec::list(vec![
      el::spec(
        Align(HorizontalAlign::Left, VerticalAlign::Bottom),
        el::spec(Image::new(self.bg_image), []),
      ),
      el::spec(
        Text {
          content: "Hello world.".into(),
          color: Color::WHITE,
          size: 24.0,
          h_align: HorizontalAlign::Right,
          v_align: VerticalAlign::Bottom,
        },
        [],
      ),
    ])
  }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Set up log macros to use nova logging.
  log::set_as_default();

  // Create a new nova app.
  let mut app = nova::App::new();

  // Load a default font.
  fonts::write(app.resources())
    .create(include_bytes!("fonts/fira_sans_regular.ttf"))
    .unwrap();

  // Load a background image.
  let bg_image = images::write(app.resources())
    .load_asset_at_path(&"/do-it.jpg".into(), &assets::read(app.resources()));

  // Add a root `Game` element.
  app.add_element(Game { bg_image });

  // Run the app until the window is closed.
  app.run();

  Ok(())
}
