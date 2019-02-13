extern crate nova;

use nova::el;
use nova::engine;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  nova::log::set_as_default();

  let mut engine = nova::Engine::new();

  engine.on_event(engine::Event::TickEnding, el::BuildHierarchy::default());

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

impl el::PureElement for App {
  fn build(&self, _: el::ChildNodes) -> el::Node {
    el::node::list(vec![
      el::node(
        Child { id: 0 },
        vec![el::node(Grandchild { id: 0 }, vec![])],
      ),
      el::node(Child { id: 1 }, vec![]),
      el::node(Child { id: 2 }, vec![]),
    ])
  }
}

#[derive(Debug, PartialEq)]
struct Child {
  id: usize,
}

impl el::StatelessElement for Child {
  fn build(&self, children: el::ChildNodes) -> el::Node {
    if self.id == 2 {
      el::node::list(vec![
        el::node(Grandchild { id: 1 }, vec![]),
        el::node(Grandchild { id: 2 }, vec![]),
      ])
    } else {
      el::node::list(children.collect())
    }
  }
}

#[derive(Debug, PartialEq)]
struct Grandchild {
  id: usize,
}

impl el::StatelessElement for Grandchild {
  fn on_awake(&self) {
    println!("Grandchild {} is awake!", self.id);
  }

  fn build(&self, _children: el::ChildNodes) -> el::Node {
    el::node::empty()
  }
}
