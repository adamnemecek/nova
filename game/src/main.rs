//use nova::assets;
use nova::graphics;
use nova::graphics::panels;
//use nova::input;
use nova::time;
use nova::window;
//use std::sync::Arc;

mod prelude;
//mod stage;

use self::prelude::*;

/// Main entry point of the program.
pub fn main() {
  // let projection = math::Matrix4::new_orthographic(0.0, 1920.0, 0.0, 1080.0, -1.0, 1.0);

  // let transform = Matrix4::new_translation(&Vector3::new(0.5, 0.5, 0.0))
  //   .append_nonuniform_scaling(&Vector3::new(100.0, 540.0, 1.0));

  // let point = math::Point3::<f32>::new(-600.0, 600.0, 0.0);

  // let after = projection * point.to_homogeneous();

  // println!("{} => {}", point, after);

  // return ();

  let sink = bflog::LogSink::new(
    std::io::stdout(),
    bflog::Format::Modern,
    bflog::LevelFilter::Trace,
  );

  let log = bflog::Logger::new(&sink);

  let ctx = &mut engine::Context::new();

  time::init(ctx);
  //assets::init(ctx);
  window::init(ctx);
  graphics::init(ctx, &log);

  //stage::init(ctx);
  //stage::actors::driving::init(ctx);
  //stage::graphics::init(ctx);

  init(ctx);

  engine::init(ctx);
  engine::run_loop(ctx);
}

fn init(ctx: &mut engine::Context) {
  let parent = panels::create_panel(ctx);

  engine::edit_component(ctx, parent, |style: &mut panels::Style| {
    style.background = panels::Background::Solid;
    style.color = graphics::Color([0.8, 0.2, 0.2, 1.0]);
  });

  panels::add_to_root(ctx, parent);

  let child = panels::create_panel(ctx);

  engine::edit_component(ctx, child, |style: &mut panels::Style| {
    style.background = panels::Background::Solid;
    style.color = graphics::Color([0.2, 0.2, 0.8, 1.0]);
  });

  engine::edit_component(ctx, child, |layout: &mut panels::Layout| {
    layout.width = panels::Dimension::Fixed(500.0);
    layout.height = panels::Dimension::Fixed(500.0);
    layout.right = panels::Dimension::Fixed(100.0);
    layout.bottom = panels::Dimension::Fixed(100.0);
  });

  panels::set_parent(ctx, child, Some(parent));
}

/*
fn init(ctx: &mut engine::Context) {
  // Load actor templates.
  let hero_template =
    assets::load(ctx, &assets::PathBuf::from("hero-f/actor.yml")).expect("could not load hero");

  let monster_template = assets::load(ctx, &assets::PathBuf::from("004-fire-salamander/actor.yml"))
    .expect("could not load monster");

  // Create actor entities.
  let hero = stage::actors::build_entity(
    Arc::new(hero_template),
    stage::graphics::actors::build_entity(engine::build_entity(ctx)),
  ).build();

  let _monster = stage::actors::build_entity(
    Arc::new(monster_template),
    stage::graphics::actors::build_entity(engine::build_entity(ctx)),
  ).with(stage::Position {
    point: Point3::new(32.0, 24.0, 0.0),
  }).build();

  // Set the camera target to the hero.
  stage::graphics::set_camera_target(ctx, hero);

  // Set the hero to be input controlled.
  stage::actors::driving::drive(ctx, hero);

  // Load custom input mapping.
  if let Ok(mapping) = assets::load(ctx, &assets::PathBuf::from("input-mapping.yml")) {
    input::set_mapping(ctx, mapping);
  }

  {
    let image = Arc::new(
      assets::load::<graphics::Image>(ctx, &assets::PathBuf::from("solid-white.png"))
        .expect("could not load image"),
    );

    let parent = panels::create_panel(ctx);

    engine::edit_component(ctx, parent, |style: &mut panels::Style| {
      style.background = Some(image.clone());
      style.color = graphics::Color::new(0.8, 0.2, 0.2, 1.0);
    });

    panels::add_to_root(ctx, parent);

    let child = panels::create_panel(ctx);

    engine::edit_component(ctx, child, |style: &mut panels::Style| {
      style.background = Some(image.clone());
      style.color = graphics::Color::new(0.2, 0.2, 0.8, 1.0);

      style.set_custom_draw(
        move |_: &mut engine::Context, canvas: &mut graphics::Canvas, _: &Rect<f32>| {
          canvas.draw(&image, graphics::DrawParams::default());
        },
      );
    });

    engine::edit_component(ctx, child, |layout: &mut panels::Layout| {
      layout.width = panels::Dimension::Fixed(100.0);
      layout.left = panels::Dimension::Auto;
      layout.right = panels::Dimension::Fixed(0.0);
    });

    panels::set_parent(ctx, child, Some(parent));
  }
}
*/
