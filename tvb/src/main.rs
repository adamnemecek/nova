extern crate nova;

use nova::el;
use nova::engine;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  nova::log::set_as_default();

  let mut engine = nova::Engine::new();

  engine.on_event(engine::Event::TickEnding, el::BuildHierarchy::default());

  el::create::<App>(engine.resources(), ());

  engine.tick();

  el::print_all(engine.resources());

  Ok(())
}

#[derive(Debug, Default)]
struct App;

impl el::Element for App {
  type Props = ();

  fn new(_: &Self::Props) -> Self {
    App
  }

  fn build(&mut self, _: &Self::Props, children: el::ChildNodes) -> el::Node {
    el::node::list(vec![
      el::node::<Child>(
        ChildProps { id: 0 },
        vec![el::node::<Grandchild>(GrandchildProps { id: 0 }, vec![])],
      ),
      el::node::<Child>(ChildProps { id: 1 }, vec![]),
      el::node::<Child>(ChildProps { id: 2 }, vec![]),
    ])
  }
}

#[derive(Debug)]
struct Child;

#[derive(Debug, Default, PartialEq)]
struct ChildProps {
  id: usize,
}

impl el::Element for Child {
  type Props = ChildProps;

  fn new(_props: &Self::Props) -> Self {
    Child
  }

  fn build(&mut self, props: &Self::Props, children: el::ChildNodes) -> el::Node {
    if props.id == 2 {
      el::node::list(vec![
        el::node::<Grandchild>(GrandchildProps { id: 1 }, vec![]),
        el::node::<Grandchild>(GrandchildProps { id: 2 }, vec![]),
      ])
    } else {
      el::node::list(children.collect())
    }
  }
}

#[derive(Debug)]
struct Grandchild;

#[derive(Debug, Default, PartialEq)]
struct GrandchildProps {
  id: usize,
}

impl el::Element for Grandchild {
  type Props = GrandchildProps;

  fn new(_props: &Self::Props) -> Self {
    Grandchild
  }

  fn build(&mut self, _props: &Self::Props, children: el::ChildNodes) -> el::Node {
    el::node::empty()
  }
}
