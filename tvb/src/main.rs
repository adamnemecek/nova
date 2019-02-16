extern crate nova;

use nova::el;
use nova::log;

#[derive(Debug, Default, PartialEq)]
struct App;

impl el::Element for App {
  type State = usize;
  type Message = usize;

  fn on_awake(&self, ctx: el::Context<Self>) {
    *ctx.state = 1;
  }

  fn on_message(&self, msg: Self::Message, ctx: el::Context<Self>) -> el::ShouldRebuild {
    println!("Got message: {:#?}.", msg);

    *ctx.state = msg + 1;

    el::ShouldRebuild(true)
  }

  fn build(&self, _: el::spec::Children, ctx: el::Context<Self>) -> el::Spec {
    println!("Rebuilt App.");

    (1..=*ctx.state)
      .map(|id| {
        el::spec(
          Child {
            id,
            on_awake: Some(ctx.compose((), |_, id| id)),
          },
          None,
        )
      })
      .collect()
  }
}

#[derive(Debug, PartialEq)]
struct Child {
  id: usize,
  on_awake: Option<el::MessageComposer<usize>>,
}

impl el::Element for Child {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    if let Some(ref on_awake) = self.on_awake {
      ctx.send(on_awake.compose(self.id));
    }
  }

  fn build(&self, children: el::spec::Children, _: el::Context<Self>) -> el::Spec {
    println!("Rebuilt Child {{ id: {:?} }}.", self.id);

    children.into()
  }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Boilerplate.
  log::set_as_default();

  let mut engine = nova::Engine::new();

  // Add App element.
  engine.add_element(App);

  // Tick five times to propagate messages.
  for _ in 0..5 {
    log::debug!("Ticking…");
    engine.tick();
  }

  // Print out the entire element graph.
  el::print_all(engine.resources());

  Ok(())
}
