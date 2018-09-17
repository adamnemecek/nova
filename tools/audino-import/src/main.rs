// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate clap;
extern crate nova_engine;
#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

use clap::{App, Arg};
use nova_engine::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

mod audino;

/// Main entry point for the tool.
fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create a clap app to parse arguments.
  let app = App::new("audino-import")
    .about("Imports sprite sheets and animations from Audino's PMD animation dump.")
    .arg(
      Arg::with_name("src")
        .help("Path to a directory containing source files for a monster.")
        .index(1)
        .required(true),
    )
    .arg(
      Arg::with_name("dest")
        .help("Path to save imported Nova assets to.")
        .index(2)
        .required(true),
    );

  // Get all matching arguments from the command line.
  let matches = app.get_matches();

  let src_path = PathBuf::from(matches.value_of_os("src").unwrap());
  let dest_path = PathBuf::from(matches.value_of_os("dest").unwrap());

  // Load audino AnimData.
  let anim_data = audino::AnimData::load(&src_path.join("animations.xml"))?;

  // Create `graphics::atlas::Data` from the AnimData.
  let atlas_data = graphics::atlas::Data {
    texture: "texture.png".into(),
    cell_width: anim_data.frame_width,
    cell_height: anim_data.frame_height,
  };

  // Create `stage::object::template::Data` from the AnimData.
  let object_template_data = {
    let mut animations = Vec::new();

    for (i, name) in audino::GROUP_NAMES.iter().enumerate() {
      let anim_group = &anim_data.group_table.groups[i];
      let mut sequences = HashMap::new();

      for (i, direction) in audino::DIRECTION_NAMES.iter().enumerate() {
        let anim_sequence_index = anim_group.sequence_indices[i];
        let anim_sequence = &anim_data.sequence_table.sequences[anim_sequence_index];

        let mut frames = Vec::new();

        for anim_frame in anim_sequence.frames.iter() {
          let cell = anim_frame.meta_frame_group_index;

          frames.push(stage::object::animation::Frame {
            length: anim_frame.duration as f64,
            cell: (cell % 8, cell / 8),
            hflip: anim_frame.hflip != 0,
          });
        }

        sequences.insert((*direction).to_owned(), frames);
      }

      animations.push(stage::object::animation::Data {
        name: (*name).to_owned(),
        sequences,
      });
    }

    stage::object::template::Data {
      atlas: "atlas.yml".into(),
      animations,
      cardinal_dirs_only: false,
    }
  };

  // Ensure dest path exists.
  fs::create_dir_all(&dest_path)?;

  // Copy the monster's sprite sheet.
  fs::copy(&src_path.join("sheet.png"), &dest_path.join("texture.png"))?;

  // Create an `Assets` resource to save assets.
  let assets = core::Assets::new(std::env::current_dir().unwrap());

  // Save the sprite atlas metadata.
  assets.save(&dest_path.join("atlas.yml"), &atlas_data)?;

  // Save the sprite atlas metadata.
  assets.save(&dest_path.join("object.yml"), &object_template_data)?;

  Ok(())
}
