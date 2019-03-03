// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use nova_core::ecs;
use nova_core::engine::Engine;

pub type PositionedGlyph = rusttype::PositionedGlyph<'static>;

#[derive(Debug)]
pub struct PositionedText {
  pub glyphs: Vec<PositionedGlyph>,
}

impl ecs::Component for PositionedText {
  type Storage = ecs::HashMapStorage<Self>;
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<PositionedText>(engine.resources_mut());
}
