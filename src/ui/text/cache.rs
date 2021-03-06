// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::text::position::PositionedText;
use nova_core::components::{Join as _, ReadComponents};
use nova_core::engine::{Engine, EnginePhase};
use nova_core::entities::Entities;
use nova_core::resources::WriteResource;
use nova_core::systems::System;

pub type GlyphCache = rusttype::gpu_cache::Cache<'static>;

#[derive(Debug)]
pub struct CacheGlyphs;

impl<'a> System<'a> for CacheGlyphs {
  type Data = (
    Entities<'a>,
    ReadComponents<'a, PositionedText>,
    WriteResource<'a, GlyphCache>,
  );

  fn run(&mut self, (entities, texts, mut cache): Self::Data) {
    for (_, text) in (&*entities, &texts).join() {
      for (glyph, _, font_id) in text.glyphs.iter().cloned() {
        cache.queue_glyph(font_id.0, glyph);
      }
    }
  }
}

pub fn set_up(engine: &mut Engine) {
  engine
    .resources
    .entry()
    .or_insert_with(|| GlyphCache::builder().dimensions(1024, 1024).build());

  engine.schedule(EnginePhase::AfterUpdate, CacheGlyphs);
}
