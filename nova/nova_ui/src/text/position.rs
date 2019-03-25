// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::screen::ScreenRect;
use crate::text::fonts::{FontId, ReadFonts};
use crate::text::{HorizontalAlign, Text, VerticalAlign};
use crate::{Color, Screen};
use glyph_brush_layout::{
  GlyphPositioner as _, Layout as GlyphBrushLayout, SectionGeometry, SectionText,
};
use nova_core::components::{
  self, Component, HashMapStorage, Join as _, ReadComponents, WriteComponents,
};
use nova_core::engine::{Engine, EnginePhase};
use nova_core::entities::Entities;
use nova_core::resources::ReadResource;
use nova_core::systems::System;

pub type PositionedGlyph = rusttype::PositionedGlyph<'static>;

#[derive(Debug, Default)]
pub struct PositionedText {
  pub glyphs: Vec<(PositionedGlyph, Color, FontId)>,
}

impl Component for PositionedText {
  type Storage = HashMapStorage<Self>;
}

#[derive(Debug)]
struct PositionText;

impl<'a> System<'a> for PositionText {
  type Data = (
    Entities<'a>,
    ReadResource<'a, Screen>,
    ReadComponents<'a, ScreenRect>,
    ReadComponents<'a, Text>,
    WriteComponents<'a, PositionedText>,
    ReadFonts<'a>,
  );

  fn run(&mut self, (entities, screen, rects, texts, mut positioned_texts, fonts): Self::Data) {
    for (entity, rect, text) in (&*entities, &rects, &texts).join() {
      let positioned = positioned_texts
        .entry(entity)
        .unwrap()
        .or_insert_with(PositionedText::default);

      let x = match text.h_align {
        HorizontalAlign::Left => rect.x1,
        HorizontalAlign::Center => rect.x1 + rect.width() / 2.0,
        HorizontalAlign::Right => rect.x2,
      };

      let y = match text.v_align {
        VerticalAlign::Top => rect.y1,
        VerticalAlign::Center => rect.y1 + rect.height() / 2.0,
        VerticalAlign::Bottom => rect.y2,
      };

      let result = GlyphBrushLayout::default_wrap()
        .h_align(text.h_align)
        .v_align(text.v_align)
        .calculate_glyphs(
          &*fonts,
          &SectionGeometry {
            screen_position: (x, y),
            bounds: (rect.width(), rect.height()),
          },
          &[SectionText {
            text: &text.content,
            scale: rusttype::Scale::uniform(text.size * screen.dpi()),
            color: text.color.into(),
            font_id: FontId::default(),
          }],
        );

      positioned.glyphs = result.into_iter().map(|g| (g.0, g.1.into(), g.2)).collect();
    }
  }
}

pub fn set_up(engine: &mut Engine) {
  components::register::<PositionedText>(&mut engine.resources);

  engine.schedule(EnginePhase::AfterUpdate, PositionText);
}
