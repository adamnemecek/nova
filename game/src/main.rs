extern crate nova;

use nova::prelude::*;

/// Main entry point of the program.
pub fn main() {
  let ctx = &mut engine::Context::new();

  engine::create_window(ctx, "nova-game");

  assets::init(ctx);
  graphics::init(ctx);
  input::init(ctx);
  time::init(ctx);

  stage::init(ctx);
  stage::actors::driving::init(ctx);
  stage::graphics::init(ctx);

  init(ctx);

  engine::run(ctx);
}

fn init(ctx: &mut engine::Context) {
  // Load actor templates.
  let hero_template =
    assets::load(ctx, &assets::PathBuf::from("hero-f/actor.yml")).expect("could not load hero");

  let monster_template = assets::load(ctx, &assets::PathBuf::from("004-fire-salamander/actor.yml"))
    .expect("could not load monster");

  // Create actor entities.
  let hero = stage::actors::build_entity(
    Arc::new(hero_template),
    stage::graphics::actors::build_entity(engine::create_entity(ctx)),
  ).build();

  let _monster = stage::actors::build_entity(
    Arc::new(monster_template),
    stage::graphics::actors::build_entity(engine::create_entity(ctx)),
  ).with(stage::Position {
    point: Point3::new(32.0, 24.0, 0.0),
  })
    .build();

  // Set the camera target to the hero.
  stage::graphics::set_camera_target(ctx, hero);

  // Set the hero to be input controlled.
  stage::actors::driving::drive(ctx, hero);

  // Load custom input mapping.
  if let Ok(mapping) = assets::load(ctx, &assets::PathBuf::from("input-mapping.yml")) {
    input::set_mapping(ctx, mapping);
  }
}
