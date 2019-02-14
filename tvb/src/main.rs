extern crate nova;

use nova::el;
use nova::engine;
use nova::engine::dispatch::seq;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  nova::log::set_as_default();

  let mut engine = nova::Engine::new();

  engine.on_event(
    engine::Event::TickEnding,
    seq![el::BuildHierarchy::default(), el::DeliverMessages::new(),],
  );

  el::create(engine.resources(), App);

  for _ in 0..5 {
    engine.tick();

    std::thread::sleep(std::time::Duration::from_millis(15));
  }

  el::print_all(engine.resources());

  Ok(())
}

#[derive(Debug, Default, PartialEq)]
struct App;

impl el::Element for App {
  type State = ();
  type Message = String;

  fn on_message(&self, msg: Self::Message, _: el::Context<Self>) -> el::ShouldRebuild {
    println!("Got message: {:#?}.", msg);

    el::ShouldRebuild(false)
  }

  fn build(&self, children: el::ChildNodes, ctx: el::Context<Self>) -> el::Node {
    el::node::list(vec![
      el::node(
        Child { id: 0 },
        el::node(
          Grandchild {
            id: 0,
            on_awake: Some(ctx.compose_with("World", |_, who| format!("Hello {}!", who))),
          },
          None,
        ),
      ),
      el::node(Child { id: 1 }, None),
      el::node(Child { id: 2 }, None),
      children.into(),
    ])
  }
}

#[derive(Debug, PartialEq)]
struct Child {
  id: usize,
}

impl el::Element for Child {
  type State = ();
  type Message = ();

  fn build(&self, children: el::ChildNodes, _: el::Context<Self>) -> el::Node {
    if self.id == 2 {
      el::node::list(vec![
        el::node(
          Grandchild {
            id: 1,
            on_awake: None,
          },
          None,
        ),
        el::node(
          Grandchild {
            id: 2,
            on_awake: None,
          },
          None,
        ),
        children.into(),
      ])
    } else {
      children.into()
    }
  }
}

#[derive(Debug, PartialEq)]
struct Grandchild {
  id: usize,
  on_awake: Option<el::MessageComposer<()>>,
}

impl el::Element for Grandchild {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    if let Some(ref on_awake) = self.on_awake {
      ctx.send(on_awake.compose(()));
    }
  }
}
