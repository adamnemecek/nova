use nova::component::{Component, NullStorage};
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  let mut instance = nova::Instance::new();

  instance.register_component::<Test>();

  let x: u64 = std::i64::MAX as u64 + 1;

  dbg!(x);
  dbg!(x as i64);

  let entity = instance.entities().create();

  instance.components_mut().insert(entity, Test);

  instance.commit_entities();

  Ok(())
}

#[derive(Default)]
struct Test;

impl Component for Test {
  type Storage = NullStorage<Self>;
}
