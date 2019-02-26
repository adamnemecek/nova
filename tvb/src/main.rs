extern crate nova;

use nova::el;
use nova::log;
use nova::ui;

#[derive(Debug, Default, PartialEq)]
struct Game;

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
            bg_color: ui::Color::new(0.0, 0.0, 1.0, 0.8),
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
  app.add_element(Game);

  // Run the app until exit.
  app.run();

  Ok(())
}
