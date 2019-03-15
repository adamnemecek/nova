// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod asset;
mod path;
mod read;
mod roots;
mod table;

pub use self::asset::Asset;
pub use self::path::AssetPath;
pub use self::read::ReadAssets;
pub use self::roots::AssetRoots;
pub use self::table::AssetTable;

// TODO: Remove after creating a custom `SystemData` derive macro.
use nova_core::shred;

use nova_core::ecs;
use nova_core::engine::{Engine, Resources};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetId(ecs::Entity);

impl From<AssetId> for ecs::Entity {
  fn from(id: AssetId) -> Self {
    id.0
  }
}

pub fn setup(engine: &mut Engine, roots: AssetRoots) {
  ecs::register::<Asset>(engine.resources_mut());

  engine
    .resources_mut()
    .entry()
    .or_insert_with(AssetTable::default);

  {
    let entities = engine.resources().fetch();
    let mut table = engine.resources().fetch_mut();
    let mut assets = ecs::write_components(engine.resources());

    for path in roots.fs_paths() {
      let result = create_assets(
        path,
        &entities,
        &mut AssetPath::from("/"),
        &mut table,
        &mut assets,
      );

      if result.is_err() {
        panic!("Could not create assets for root {:?}", path)
      }
    }
  }

  engine.resources_mut().insert(roots);
}

pub fn read(res: &Resources) -> ReadAssets {
  ecs::SystemData::fetch(res)
}

fn create_assets(
  path: &Path,
  entities: &ecs::Entities,
  asset_path: &mut AssetPath,
  table: &mut AssetTable,
  assets: &mut ecs::WriteComponents<Asset>,
) -> io::Result<()> {
  for entry in fs::read_dir(path)? {
    let entry = entry?;
    let file_type = entry.file_type()?;

    asset_path.push_component(&entry.file_name().to_string_lossy());

    if file_type.is_dir() {
      create_assets(&entry.path(), entities, asset_path, table, assets)?;
    } else if file_type.is_file() && !table.contains(asset_path) {
      let entity = entities.create();

      let asset = Asset {
        fs_path: entry.path(),
      };

      assets.insert(entity, asset).unwrap();
      table.insert(asset_path.clone(), AssetId(entity));
    }

    asset_path.pop_component();
  }

  Ok(())
}
