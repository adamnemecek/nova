use nova::el;
use nova::log;
use nova::ui::text::fonts;
use nova::ui::text::{HorizontalAlign, Text, VerticalAlign};
use nova::ui::Color;

#[derive(Debug, PartialEq)]
struct Game;

impl el::Element for Game {
  type State = ();
  type Message = ();

  fn build(&self, _: el::spec::Children, _: el::Context<Self>) -> el::Spec {
    el::spec(
      Text {
        content: "What the fuck did you just fucking say about me, you little bitch? I'll have you know I graduated top of my class in the Navy Seals, and I've been involved in numerous secret raids on Al-Quaeda, and I have over 300 confirmed kills. I am trained in gorilla warfare and I'm the top sniper in the entire US armed forces. You are nothing to me but just another target. I will wipe you the fuck out with precision the likes of which has never been seen before on this Earth, mark my fucking words. You think you can get away with saying that shit to me over the Internet? Think again, fucker. As we speak I am contacting my secret network of spies across the USA and your IP is being traced right now so you better prepare for the storm, maggot. The storm that wipes out the pathetic little thing you call your life. You're fucking dead, kid. I can be anywhere, anytime, and I can kill you in over seven hundred ways, and that's just with my bare hands. Not only am I extensively trained in unarmed combat, but I have access to the entire arsenal of the United States Marine Corps and I will use it to its full extent to wipe your miserable ass off the face of the continent, you little shit. If only you could have known what unholy retribution your little \"clever\" comment was about to bring down upon you, maybe you would have held your fucking tongue. But you couldn't, you didn't, and now you're paying the price, you goddamn idiot. I will shit fury all over you and you will drown in it. You're fucking dead, kiddo.".into(),
        color: Color::WHITE,
        size: 24.0,
        h_align: HorizontalAlign::Right,
        v_align: VerticalAlign::Bottom,
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
