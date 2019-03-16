// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::elements::{Element, ElementContext};
use crate::layout::{Constraints, Layout};
use nova_core::ecs::{self, Join as _};
use nova_core::engine::{Engine, EngineEvent};
use nova_graphics::images::{ImageSlice, ReadImages};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Image {
  pub slice: ImageSlice,
}

impl Image {
  pub fn new(slice: impl Into<ImageSlice>) -> Self {
    Image {
      slice: slice.into(),
    }
  }
}

impl ecs::Component for Image {
  type Storage = ecs::HashMapStorage<Self>;
}

impl Element for Image {
  type State = ();

  fn on_awake(&self, ctx: ElementContext<Self>) {
    ctx.put_component(*self);
  }

  fn on_change(&self, _: Self, mut ctx: ElementContext<Self>) {
    ctx.put_component(*self);
    ctx.rebuild();
  }

  fn on_sleep(&self, ctx: ElementContext<Self>) {
    ctx.remove_component::<Image>();
  }
}

#[derive(Debug)]
struct UpdateImageSizes;

impl<'a> ecs::System<'a> for UpdateImageSizes {
  type SystemData = (
    ecs::ReadEntities<'a>,
    ReadImages<'a>,
    ecs::ReadComponents<'a, Image>,
    ecs::WriteComponents<'a, Layout>,
  );

  fn run(&mut self, (entities, images, image_comps, mut layouts): Self::SystemData) {
    for (entity, image_comp) in (&*entities, &image_comps).join() {
      let size = images
        .get(image_comp.slice.image_id)
        .map(|i| i.size().into())
        .unwrap_or_default();

      layouts
        .insert(
          entity,
          Layout::Constrained(Constraints {
            min: size,
            max: size,
          }),
        )
        .unwrap();
    }
  }
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<Image>(engine.resources_mut());

  engine.on_event(EngineEvent::TickEnding, UpdateImageSizes);
}
