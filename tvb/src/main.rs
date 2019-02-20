extern crate nova;

use nova::assets;
use nova::el;
use nova::graphics;
use nova::log;
use nova::ui;

#[derive(Debug, Default, PartialEq)]
struct Game;

#[derive(Debug)]
enum Message {
  ImageLoaded(graphics::Image),
}

impl el::Element for Game {
  type State = Option<graphics::Image>;
  type Message = Message;

  fn on_message(&self, msg: Message, ctx: el::Context<Self>) -> el::ShouldRebuild {
    let Message::ImageLoaded(image) = msg;

    *ctx.state = Some(image);

    el::ShouldRebuild(true)
  }

  fn build(&self, _: el::spec::Children, ctx: el::Context<Self>) -> el::Spec {
    /*
      <>
        <div
          layout: (x: 160, y: 90, width: 600, height: 634),
          style: (bg_color: #ffffffff, bg_image: state),
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
            bg_image: ctx.state.clone(),
          },
        },
        [],
      ),
      el::spec(
        assets::Asset {
          path: "do-it.jpg".into(),
          on_load: ctx.compose((), |_, result| Message::ImageLoaded(result.unwrap())),
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
