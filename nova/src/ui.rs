// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod layout;
mod painter;

use crate::ecs;
use crate::el;
use crate::graphics::Image;
use crate::renderer::TextureId;
use crate::Engine;

pub use self::layout::Layout;
pub use self::painter::Painter;
pub use crate::graphics::Color4 as Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Style {
  pub bg_color: Color,
  pub bg_image: Option<Image>,
}

impl Default for Style {
  fn default() -> Self {
    Self {
      bg_color: Color::WHITE,
      bg_image: None,
    }
  }
}

impl ecs::Component for Style {
  type Storage = ecs::BTreeStorage<Self>;
}

#[derive(Debug, Default)]
pub struct StyleCache {
  bg_texture: Option<TextureId>,
}

impl ecs::Component for StyleCache {
  type Storage = ecs::BTreeStorage<Self>;
}

#[derive(Debug, Default, PartialEq)]
pub struct Div {
  pub layout: Layout,
  pub style: Style,
}

impl el::Element for Div {
  type State = ();
  type Message = ();

  fn on_awake(&self, ctx: el::Context<Self>) {
    ctx.put_component(self.layout);
    ctx.put_component(self.style.clone());
    ctx.put_component(StyleCache::default());
  }

  fn on_change(&self, _: Self, ctx: el::Context<Self>) -> el::ShouldRebuild {
    self.on_awake(ctx);

    el::ShouldRebuild(true)
  }

  fn on_sleep(&self, ctx: el::Context<Self>) {
    ctx.remove_component::<Layout>();
    ctx.remove_component::<Style>();
    ctx.remove_component::<StyleCache>();
  }
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<Style>(engine.resources_mut());
  ecs::register::<StyleCache>(engine.resources_mut());

  layout::setup(engine);
}

/*
mod layout {
  use crate::ecs;
  use crate::el;
  use crate::window::Window;
  use atomic_refcell::AtomicRefCell;
  use rayon::prelude::*;
  use std::ops::Range;

  #[derive(Debug)]
  pub struct Layout {
    pub padding: Padding,
    pub arrange: Arrange,
  }

  const DEFAULT_LAYOUT: Layout = Layout {
    padding: Padding {
      top: 0,
      right: 0,
      bottom: 0,
      left: 0,
    },
    arrange: Arrange::Stacked,
  };

  impl ecs::Component for Layout {
    type Storage = ecs::BTreeStorage<Self>;
  }

  #[derive(Debug)]
  pub enum Arrange {
    Stacked,
    Horizontal,
    Vertical,
  }

  #[derive(Debug)]
  pub struct Padding {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
  }

  #[derive(Debug, Clone)]
  pub struct Constraints {
    pub width: Range<f32>,
    pub height: Range<f32>,
  }

  #[derive(Debug)]
  pub struct ComputedRect {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
  }

  impl ecs::Component for ComputedRect {
    type Storage = ecs::BTreeStorage<Self>;
  }

  #[derive(Debug, Default)]
  pub struct ComputeLayout {
    compute_stack: Vec<(ecs::Entity, Constraints)>,
    finish_stack: Vec<ecs::Entity>,
  }

  impl ComputeLayout {
    fn layout(
      &self,
      entity: ecs::Entity,
      constraints: &Constraints,
      nodes: &ecs::ReadComponents<el::hierarchy::Node>,
      layouts: &ecs::ReadComponents<Layout>,
      computed_rects: &mut ecs::WriteComponents<ComputedRect>,
    ) {
      let node = match nodes.get(entity) {
        Some(n) => n,
        None => return,
      };

      let layout = layouts.get(entity).unwrap_or(&DEFAULT_LAYOUT);

      match layout.arrange {
        _ => {
          for child in node.children() {
            self.layout(child, constraints, nodes, layouts, computed_rects);
          }
        }
      }
    }
  }

  impl<'a> ecs::System<'a> for ComputeLayout {
    type SystemData = (
      ecs::ReadResource<'a, Window>,
      ecs::ReadResource<'a, el::Hierarchy>,
      ecs::ReadComponents<'a, el::hierarchy::Node>,
      ecs::ReadComponents<'a, Layout>,
      ecs::ReadComponents<'a, ComputedRect>,
    );

    fn run(&mut self, (window, hierarchy, nodes, layouts, mut computed_rects): Self::SystemData) {
      let size = window.size();

      let constraints = Constraints {
        width: 0.0..size.width() as f32,
        height: 0.0..size.height() as f32,
      };

      hierarchy
        .roots()
        .map(move |root| (root, constraints.clone()))
    }
  }
}
*/
