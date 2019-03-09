// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::layout::ScreenRect;
use crate::text::fonts::{FontId, ReadFonts};
use crate::text::Text;
use glyph_brush_layout::{
  GlyphPositioner as _, Layout as GlyphBrushLayout, SectionGeometry, SectionText,
};
use nova_core::ecs::{self, Join as _};
use nova_core::engine::{Engine, EngineEvent};

pub type PositionedGlyph = rusttype::PositionedGlyph<'static>;

#[derive(Debug, Default)]
pub struct PositionedText {
  pub glyphs: Vec<(FontId, PositionedGlyph)>,
}

impl ecs::Component for PositionedText {
  type Storage = ecs::HashMapStorage<Self>;
}

#[derive(Debug)]
struct PositionText;

impl<'a> ecs::System<'a> for PositionText {
  type SystemData = (
    ecs::ReadEntities<'a>,
    ReadFonts<'a>,
    ecs::ReadComponents<'a, ScreenRect>,
    ecs::ReadComponents<'a, Text>,
    ecs::WriteComponents<'a, PositionedText>,
  );

  fn run(&mut self, (entities, fonts, rects, texts, mut positioned_texts): Self::SystemData) {
    for (entity, rect, text) in (&*entities, &rects, &texts).join() {
      let positioned = positioned_texts
        .entry(entity)
        .unwrap()
        .or_insert_with(PositionedText::default);

      let result = GlyphBrushLayout::default_wrap()
        .h_align(text.h_align)
        .v_align(text.v_align)
        .calculate_glyphs(
          &*fonts,
          &SectionGeometry {
            screen_position: ((rect.x1 + rect.x2) / 2.0, (rect.y1 + rect.y2) / 2.0),
            bounds: (rect.width(), rect.height()),
          },
          &[SectionText {
            text: &text.content,
            scale: rusttype::Scale::uniform(64.0),
            color: [0.0, 0.0, 0.0, 1.0],
            font_id: FontId::default(),
          }],
        );

      positioned.glyphs = result.into_iter().map(|g| (g.2, g.0)).collect();
    }
  }
}

pub fn setup(engine: &mut Engine) {
  ecs::register::<PositionedText>(engine.resources_mut());

  engine.on_event(EngineEvent::TickEnding, PositionText);
}
