// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate nova;
extern crate specs;

use nova::prelude::*;
use std::error::Error;
use std::path::PathBuf;

/// Main entry point of the program.
pub fn main() -> Result<(), Box<dyn Error>> {
  let mut window = platform::Window::new("nova-game");
  let mut canvas = graphics::Canvas::new(&mut window);

  canvas.clear(graphics::Color::new(0.0, 0.0, 0.0, 1.0));

  let mut world = World::new();

  assets::setup(&mut world);
  graphics::setup(&mut world);
  input::setup(&mut world);
  time::setup(&mut world);

  let mut systems = DispatcherBuilder::default();

  stage::setup(&mut world, &mut systems);
  stage::draw::setup(&mut world, &mut systems);

  unstable::setup(&mut world, &mut systems);

  let mut systems = systems.build();

  setup(&mut world)?;

  // Run the main event loop.
  while !window.is_closing() {
    window.update();

    if window.was_resized() {
      canvas.resize(window.size());
    }

    time::tick(&mut world);
    input::update(&mut world, &mut window);

    systems.dispatch(&mut world.res);

    canvas.clear(graphics::Color::new(0.53, 0.87, 0.52, 1.0));

    stage::draw(&mut world, &mut canvas);

    canvas.present();
  }

  Ok(())
}

fn setup<'a, 'b>(world: &mut World) -> Result<(), Box<dyn Error>> {
  let (hero_template, monster_template) = {
    let fs = world.read_resource::<assets::OverlayFs>();

    (
      fs.load::<stage::actors::Template>(&PathBuf::from("hero-f/actor.yml"))?,
      fs.load::<stage::actors::Template>(&PathBuf::from("004-fire-salamander/actor.yml"))?,
    )
  };

  let hero = stage::actors::build_entity(Arc::new(hero_template), world.create_entity()).build();

  let _monster = stage::actors::build_entity(Arc::new(monster_template), world.create_entity())
    .with(stage::Position {
      point: Point3::new(32.0, 24.0, 0.0),
    })
    .build();

  // Set the camera target to the hero.
  world.write_resource::<stage::Camera>().set_target(hero);

  // Set the hero to be input controlled.
  world
    .write_storage()
    .insert(hero, unstable::InputControlled)?;

  // Set the object shadow texture.
  let circle = world
    .read_resource::<assets::OverlayFs>()
    .load(&PathBuf::from("circle.png"))
    .ok();

  world
    .write_resource::<stage::objects::draw::Settings>()
    .shadow_texture = circle;

  Ok(())
}
