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
use nova_core::ecs::{self, Join as _};
use nova_core::engine::{Engine, EngineEvent};

pub type PositionedGlyph = rusttype::PositionedGlyph<'static>;

#[derive(Debug, Default)]
pub struct PositionedText {
  pub glyphs: Vec<(PositionedGlyph, Color, FontId)>,
}

impl ecs::Component for PositionedText {
  type Storage = ecs::HashMapStorage<Self>;
}

#[derive(Debug)]
struct PositionText;

impl<'a> ecs::System<'a> for PositionText {
  type SystemData = (
    ecs::ReadEntities<'a>,
    ecs::ReadResource<'a, Screen>,
    ecs::ReadComponents<'a, ScreenRect>,
    ecs::ReadComponents<'a, Text>,
    ecs::WriteComponents<'a, PositionedText>,
    ReadFonts<'a>,
  );

  fn run(
    &mut self,
    (entities, screen, rects, texts, mut positioned_texts, fonts): Self::SystemData,
  ) {
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

pub fn setup(engine: &mut Engine) {
  ecs::components::register::<PositionedText>(&mut engine.resources);

  engine.on_event(EngineEvent::TickEnding, PositionText);
}
