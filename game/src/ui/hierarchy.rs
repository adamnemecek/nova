use crate::ecs;
use crate::ecs::derive::*;

pub struct Hierarchy {
  pub root: Option<ecs::Entity>,
  pub sorted: Vec<ecs::Entity>,
}

#[derive(Component)]
#[storage(BTreeStorage)]
pub struct Node {
  parent: Option<ecs::Entity>,
  children: Vec<ecs::Entity>,
}

pub fn set_root(ctx: &mut ecs::Context, node: ecs::Entity) {
  let hierarchy: &mut Hierarchy = ecs::get_resource_mut(ctx);

  hierarchy.root = Some(node);
}
