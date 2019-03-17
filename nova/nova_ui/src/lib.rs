// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod elements;
pub mod layout;
pub mod messages;
pub mod nodes;
pub mod specs;
pub mod text;

mod image;
mod painter;
mod screen;

pub use self::elements::{Element, ElementContext};
pub use self::image::Image;
pub use self::layout::elements::{Align, AspectRatioFill, Fill};
pub use self::layout::{HorizontalAlign, VerticalAlign};
pub use self::nodes::NodeContext;
pub use self::painter::Painter;
pub use self::screen::Screen;
pub use self::specs::{ChildSpecs, Spec};
pub use self::text::Text;
pub use nova_graphics::Color4 as Color;

use self::elements::ElementInstance;
use nova_core::ecs;
use nova_core::engine::{Engine, Resources};
use nova_core::shred;

pub fn setup(engine: &mut Engine) {
  image::setup(engine);
  layout::setup(engine);
  messages::setup(engine);
  nodes::setup(engine);
  screen::setup(engine);
  text::setup(engine);
}

pub fn add_to_root(res: &Resources, element: impl Element + 'static) -> ecs::Entity {
  let entities = ecs::entities::read(res);
  let entity = entities.create();
  let mut nodes = nodes::write(res);

  nodes.create_on_entity(
    entity,
    ElementInstance::new(
      element,
      NodeContext {
        resources: res,
        entities: &entities,
        entity,
        parent: None,
        // Ignored because the element is new.
        should_rebuild: &mut true,
      },
    ),
    None,
  );

  nodes.hierarchy.roots.push(entity);

  entity
}
