use specs;
use specs::prelude::*;

use super::Position;

pub struct MovementSystem {
  target: Entity,
}

impl<'a> System<'a> for MovementSystem {
  type SystemData = WriteStorage<'a, Position>;

  fn run(&mut self, mut positions: Self::SystemData) {
    positions.get_mut(self.target);
  }
}
