pub mod storages;

mod components;
mod entities;
mod resources;

pub use self::components::*;
pub use self::entities::*;
pub use self::resources::*;
pub use self::storages::*;
pub use specs::{Dispatcher, DispatcherBuilder, System};
pub use specs_derive::*;

pub struct Context {
  world: specs::World,
}

impl Context {
  pub fn new() -> Self {
    Context {
      world: specs::World::new(),
    }
  }

  pub fn resources(&mut self) -> &mut Resources {
    &mut self.world.res
  }
}
