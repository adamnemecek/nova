// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use glyph_brush_layout::FontId;
pub use rusttype::Error as FontError;

use nova_core::engine::Engine;
use nova_core::quick_error;
use nova_core::resources::{self, ReadResource, Resources, WriteResource};
use std::fs;
use std::io;
use std::ops::Index;
use std::path::Path;

pub type Font = rusttype::Font<'static>;
pub type ReadFonts<'a> = ReadResource<'a, Fonts>;
pub type WriteFonts<'a> = WriteResource<'a, Fonts>;

#[derive(Debug, Default)]
pub struct Fonts {
  list: Vec<Font>,
}

impl Fonts {
  pub fn add(&mut self, font: Font) -> FontId {
    self.list.push(font);

    FontId(self.list.len() - 1)
  }

  pub fn load_file(&mut self, path: impl AsRef<Path>) -> Result<FontId, FontLoadError> {
    let bytes = fs::read(path)?;
    let font = Font::from_bytes(bytes)?;

    Ok(self.add(font))
  }
}

impl Index<FontId> for Fonts {
  type Output = Font;

  fn index(&self, id: FontId) -> &Font {
    &self.list[id.0]
  }
}

impl glyph_brush_layout::FontMap<'static> for Fonts {
  fn font(&self, id: glyph_brush_layout::FontId) -> &Font {
    &self.list[id.0]
  }
}

pub fn setup(engine: &mut Engine) {
  engine.resources.entry().or_insert_with(Fonts::default);
}

pub fn borrow(res: &Resources) -> ReadFonts {
  resources::borrow(res)
}

pub fn borrow_mut(res: &Resources) -> WriteFonts {
  resources::borrow_mut(res)
}

quick_error! {
  #[derive(Debug)]
  pub enum FontLoadError {
    Io(err: io::Error) {
      from()
      description("font io error")
      display("could not load font: {}", err)
    }
    Font(err: FontError) {
      from()
      description("font error")
      display("could not create font: {}", err)
    }
  }
}
