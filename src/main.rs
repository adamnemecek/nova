use nova::el;
use nova::log;
use nova::ui::text::fonts;
use nova::ui::text::Text;

#[derive(Debug, PartialEq)]
struct Game;

impl el::Element for Game {
  type State = ();
  type Message = ();

  fn build(&self, _: el::spec::Children, _: el::Context<Self>) -> el::Spec {
    el::spec(
      Text {
        content: "Hello world!".into(),
        ..Default::default()
      },
      [],
    )
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

  // Add a root `Game` element.
  app.add_element(Game);

  // Run the app until the window is closed.
  app.run();

  Ok(())
}
