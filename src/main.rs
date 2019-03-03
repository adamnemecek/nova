use nova::assets::{AssetTable, ReadAssets};
use nova::el;
use nova::graphics::images::{ImageSlice, WriteImages};
use nova::log;
use nova::math::Rect;
use nova::ui;

#[derive(Debug, PartialEq)]
struct Game {
  image: ImageSlice,
}

impl el::Element for Game {
  type State = ();
  type Message = ();

  fn build(&self, _: el::spec::Children, _: el::Context<Self>) -> el::Spec {
    el::spec::list(vec![
      el::spec(
        ui::Container {
          layout: ui::Layout {
            right: ui::layout::Dimension::Auto,
            ..Default::default()
          },
          style: ui::Style {
            bg_image: Some(self.image),
            ..Default::default()
          },
        },
        [],
      ),
      el::spec(
        ui::Container {
          layout: ui::Layout {
            left: ui::layout::Dimension::Auto,
            width: ui::layout::Dimension::Fraction(0.3),
            ..Default::default()
          },
          style: ui::Style {
            bg_color: ui::Color::new(1.0, 0.0, 0.0, 0.8),
            ..Default::default()
          },
        },
        [],
      ),
    ])
  }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Set up log macros to use nova logging.
  log::set_as_default();

  // Create a new nova app.
  let mut app = nova::App::new();

  let image_id = {
    use nova::ecs::SystemData as _;

    let asset_table = app.resources().fetch::<AssetTable>();
    let assets = ReadAssets::fetch(app.resources());
    let mut images = WriteImages::fetch(app.resources());
    let path = "/do-it.jpg".into();

    images.load_asset(asset_table.get(&path).unwrap(), &assets)
  };

  // Add a root `Game` element.
  app.add_element(Game {
    image: ImageSlice {
      image_id,
      rect: Rect {
        x1: 0.0,
        y1: 0.25,
        x2: 1.0,
        y2: 0.75,
      },
    },
  });

  // Run the app until exit.
  app.run();

  Ok(())
}
