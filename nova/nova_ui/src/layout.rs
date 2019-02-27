// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod screen_rect;
mod solve;

pub use self::screen_rect::ScreenRect;
pub use self::solve::SolveLayout;

use nova_core::ecs;
use nova_core::engine::{self, Engine};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Layout {
  pub top: Dimension,
  pub right: Dimension,
  pub bottom: Dimension,
  pub left: Dimension,
  pub width: Dimension,
  pub height: Dimension,
}

impl Layout {
  const DEFAULT: Self = Self {
    top: Dimension::Fixed(0.0),
    right: Dimension::Fixed(0.0),
    bottom: Dimension::Fixed(0.0),
    left: Dimension::Fixed(0.0),
    width: Dimension::Auto,
    height: Dimension::Auto,
  };
}

impl Default for Layout {
  fn default() -> Self {
    Self::DEFAULT
  }
}

impl ecs::Component for Layout {
  type Storage = ecs::HashMapStorage<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dimension {
  Auto,
  Fixed(f32),
  Fraction(f32),
}

impl Dimension {
  fn into_scalar(self, total: f32) -> Option<f32> {
    match self {
      Dimension::Auto => None,
      Dimension::Fixed(val) => Some(val),
      Dimension::Fraction(val) => Some(total * val),
    }
  }
}

pub fn setup(engine: &mut Engine) {
  engine.on_event(engine::Event::TickEnding, SolveLayout);
}
