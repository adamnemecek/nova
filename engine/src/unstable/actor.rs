// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use prelude::*;

use std::error::Error;
use std::path::PathBuf;

pub fn load(core: &mut Core, path: impl Into<PathBuf>) -> Result<Entity, Box<dyn Error>> {
  let mut path = path.into();

  let atlas = {
    path.push("atlas.png");

    let image = core::assets::load_image(core, &path)?;

    path.set_extension("yml");

    let data = core.world.read_resource::<core::Assets>().load(&path)?;

    path.pop();

    Arc::new(graphics::sprite::Atlas { image, data })
  };

  path.push("sequences.yml");

  let sequences = core
    .world
    .read_resource::<core::Assets>()
    .load::<Vec<graphics::sprite::animation::Sequence>>(&path)?;

  Ok(
    core
      .world
      .create_entity()
      .with(graphics::sprite::Sprite::new(atlas))
      .with(graphics::sprite::Animation {
        sequence: sequences.into_iter().next().map(Arc::new),
        elapsed: 0.0,
      })
      .with(stage::Render::default())
      .with(stage::Position::default())
      .with(stage::Velocity::default())
      .build(),
  )
}
