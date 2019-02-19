extern crate nova;

use nova::assets;
use nova::el;
use nova::log;

#[derive(Debug, Default, PartialEq)]
struct Game;

#[derive(Debug)]
enum Message {
  AssetLoaded(String),
}

impl el::Element for Game {
  type State = ();
  type Message = Message;

  fn on_message(&self, msg: Message, _: el::Context<Self>) -> el::ShouldRebuild {
    let Message::AssetLoaded(content) = msg;

    print!("{}", &content);

    el::ShouldRebuild(false)
  }

  fn build(&self, _: el::spec::Children, ctx: el::Context<Self>) -> el::Spec {
    el::spec(
      assets::Asset {
        path: "test.txt".into(),
        on_load: ctx.compose((), |_, result| Message::AssetLoaded(result.unwrap())),
      },
      None,
    )
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
