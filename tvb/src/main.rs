extern crate nova;

use nova::assets;
use nova::el;
use nova::graphics;
use nova::log;
use nova::ui;

#[derive(Debug, Default, PartialEq)]
struct Game;

#[derive(Debug)]
struct State {
  image: Option<graphics::Image>,
  on_load: el::MessageFn<assets::LoadResult<graphics::Image>>,
}

impl el::ElementState for State {
  fn new(ctx: el::NodeContext) -> Self {
    State {
      image: None,
      on_load: ctx.message_fn(|result| Message::ImageLoaded(result.unwrap())),
    }
  }
}

#[derive(Debug)]
enum Message {
  ImageLoaded(graphics::Image),
}

impl el::Element for Game {
  type State = State;
  type Message = Message;

  fn on_message(&self, msg: Message, ctx: el::Context<Self>) -> el::ShouldRebuild {
    let Message::ImageLoaded(image) = msg;

    ctx.state.image = Some(image);

    el::ShouldRebuild(true)
  }

  fn build(&self, _: el::spec::Children, ctx: el::Context<Self>) -> el::Spec {
    /*
      <>
        <div
          layout: (x: 160, y: 90, width: 600, height: 634),
          style: (bg_color: #ffffffff, bg_image: state.image),
        />
        <Asset path: "do-it.jpg", on_load: |img| ImageLoaded(img) />
      </>
    */
    el::spec::list(vec![
      el::spec(
        ui::Div {
          layout: ui::Layout {
            x: 160.0,
            y: 90.0,
            width: 600.0,
            height: 634.0,
          },
          style: ui::Style {
            bg_color: ui::Color::WHITE,
            bg_image: ctx.state.image.clone(),
          },
        },
        [],
      ),
      el::spec(
        assets::Asset {
          path: "do-it.jpg".into(),
          on_load: ctx.state.on_load.clone(),
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
