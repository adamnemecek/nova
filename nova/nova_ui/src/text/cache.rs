// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::text::position::PositionedText;
use nova_core::ecs::{self, Join as _};
use nova_core::engine::{Engine, EngineEvent};

pub type GlyphCache = rusttype::gpu_cache::Cache<'static>;

#[derive(Debug)]
pub struct CacheGlyphs;

impl<'a> ecs::System<'a> for CacheGlyphs {
  type SystemData = (
    ecs::ReadEntities<'a>,
    ecs::ReadComponents<'a, PositionedText>,
    ecs::WriteResource<'a, GlyphCache>,
  );

  fn run(&mut self, (entities, texts, mut cache): Self::SystemData) {
    for (_, text) in (&*entities, &texts).join() {
      for (glyph, _, font_id) in text.glyphs.iter().cloned() {
        cache.queue_glyph(font_id.0, glyph);
      }
    }
  }
}

pub fn setup(engine: &mut Engine) {
  engine
    .res
    .entry()
    .or_insert_with(|| GlyphCache::builder().dimensions(1024, 1024).build());

  engine.on_event(EngineEvent::TickEnding, CacheGlyphs);
}
