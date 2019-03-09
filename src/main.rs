use nova::clock;
use nova::ecs;
use nova::el;
use nova::log;
use nova::ui::text::fonts;
use nova::ui::text::position::PositionedText;
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

  // Tick the app a few times to update things.
  app.tick(clock::DeltaTime::Fixed(clock::Duration::from_hz(60)));
  app.tick(clock::DeltaTime::Fixed(clock::Duration::from_hz(60)));
  app.tick(clock::DeltaTime::Fixed(clock::Duration::from_hz(60)));

  use nova::ecs::Join;

  for (positioned) in (&ecs::read_components::<PositionedText>(app.resources())).join() {
    println!("{:#?}", positioned);
  }

  Ok(())
}
