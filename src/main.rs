use nova::component::{self, Join};
use nova::time;
use std::error::Error;

pub fn main() -> Result<(), Box<dyn Error>> {
  let mut instance = nova::Instance::new();

  instance.register_component::<Test>();

  let entity = instance.entities().create();

  instance.components_mut().insert(entity, Test);

  instance.commit_entities();

  Ok(())
}

#[derive(Default)]
struct Test;

impl nova::Component for Test {
  type Storage = component::NullStorage<Self>;
}
