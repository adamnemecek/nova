extern crate nova;

use nova::prelude::*;

struct Game {
  window: platform::Window,
  canvas: graphics::Canvas,
}

impl Application for Game {
  // Set up world and systems.
  fn setup<'a, 'b>(&mut self, world: &mut World, systems: &mut DispatcherBuilder<'a, 'b>) {
    assets::setup(world);
    input::setup(world);
    time::setup(world);

    stage::setup(world, systems);
    stage::actors::driving::setup(world, systems);
    stage::visuals::setup(world, systems);

    setup(world).expect("could not load assets");
  }

  // Perform early update logic before systems are run.
  fn before_update(&mut self, world: &mut World) {
    self.window.update();

    if self.window.was_resized() {
      self.canvas.resize(self.window.size());
    }

    time::tick(world);
    input::update(world, &mut self.window);
  }

  // Perform update logic after systems are run.
  fn update(&mut self, world: &mut World) {
    self
      .canvas
      .clear(graphics::Color::new(0.53, 0.87, 0.52, 1.0));

    stage::visuals::draw(world, &mut self.canvas);

    self.canvas.present();
  }

  // Return `false` to exit the game.
  fn is_running(&self) -> bool {
    !self.window.is_closing()
  }
}

/// Main entry point of the program.
pub fn main() {
  let window = platform::Window::new("nova-game");
  let canvas = graphics::Canvas::new(&window);

  let game = Game { window, canvas };

  game.run();
}

/// Set up program test world.
fn setup<'a, 'b>(world: &mut World) -> Result<(), assets::Error> {
  // Load actor templates.
  let hero_template = assets::load(world, &assets::PathBuf::from("hero-f/actor.yml"))?;

  let monster_template = assets::load(
    world,
    &assets::PathBuf::from("004-fire-salamander/actor.yml"),
  )?;

  // Create actor entities.
  let hero = stage::actors::build_entity(
    Arc::new(hero_template),
    stage::visuals::actors::build_entity(world.create_entity()),
  ).build();

  let _monster = stage::actors::build_entity(
    Arc::new(monster_template),
    stage::visuals::actors::build_entity(world.create_entity()),
  ).with(stage::Position {
    point: Point3::new(32.0, 24.0, 0.0),
  })
    .build();

  // Set the camera target to the hero.
  stage::set_camera_target(world, hero);

  // Set the hero to be input controlled.
  stage::actors::driving::drive(world, hero);

  // Load custom input mapping.
  if let Ok(mapping) = assets::load(world, &assets::PathBuf::from("input-mapping.yml")) {
    input::set_mapping(world, mapping);
  }

  Ok(())
}
