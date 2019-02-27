use nova::el;
use nova::graphics::{Image, ImageSlice};
use nova::log;
use nova::math::Rect;
use nova::ui;

#[derive(Debug, PartialEq)]
struct Game {
  image: ImageSlice,
}

impl el::Element for Game {
  type State = ();
  type Message = ();

  fn build(&self, _: el::spec::Children, _: el::Context<Self>) -> el::Spec {
    el::spec::list(vec![
      el::spec(
        ui::Container {
          layout: ui::Layout {
            right: ui::layout::Dimension::Auto,
            ..Default::default()
          },
          style: ui::Style {
            bg_image: Some(self.image.clone()),
            ..Default::default()
          },
        },
        [],
      ),
      el::spec(
        ui::Container {
          layout: ui::Layout {
            left: ui::layout::Dimension::Auto,
            width: ui::layout::Dimension::Fraction(0.3),
            ..Default::default()
          },
          style: ui::Style {
            bg_color: ui::Color::new(1.0, 0.0, 0.0, 0.8),
            ..Default::default()
          },
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

  // Add a root `Game` element.
  app.add_element(Game {
    image: ImageSlice::new(
      Image::from_bytes(include_bytes!("../assets/do-it.jpg"))?,
      Rect {
        x1: 0.0,
        y1: 0.25,
        x2: 1.0,
        y2: 0.75,
      },
    ),
  });

  // Run the app until exit.
  app.run();

  Ok(())
}