use nova::ecs;
use nova::graphics::Color4;
use nova::ui;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  use nova::ecs::BuildEntity;

  nova::log::set_as_default();

  let mut engine = nova::Engine::new(Default::default());

  ecs::create_entity(engine.resources_mut())
    .with(ui::Layout {
      x: 100.0,
      y: 100.0,
      width: 1080.0,
      height: 520.0,
    })
    .with(ui::Background {
      color: Color4::new(0.0, 0.0, 1.0, 1.0),
    })
    .build();

  engine.run();

  Ok(())
}
