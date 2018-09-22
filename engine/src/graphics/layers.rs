use super::*;

/// Resource storing a list of layers to draw on the screen.
#[derive(Default)]
pub struct DrawLayers {
  pub vec: Vec<Arc<dyn DrawLayer>>,
}

/// Trait implemented by types to add a draw layer to the engine.
pub trait DrawLayer: 'static + Send + Sync {
  /// Draws the contents of the draw layer onto the given canvas.
  fn draw(&self, ctx: &mut engine::Context, canvas: &mut Canvas);
}

/// Engine process that draws layers from `DrawLayers` onto the screen.
pub struct LayerDrawer {
  /// Canvas created from the window of the engine contex.
  canvas: Canvas,
  /// Buffer for storing the list of draw layers while drawing.
  buffer: Vec<Arc<dyn DrawLayer>>,
}

impl LayerDrawer {
  /// Creates a new layer drawer for the given canvas.
  pub fn new(canvas: Canvas) -> Self {
    LayerDrawer {
      canvas,
      buffer: Vec::new(),
    }
  }
}

impl engine::Process for LayerDrawer {
  fn late_update(&mut self, ctx: &mut engine::Context) {
    // Resize canvas to match window size.
    {
      let window = engine::fetch_resource::<engine::Window>(ctx);

      if window.was_resized() {
        self.canvas.resize(window.size());
      }
    }

    if engine::has_resource::<DrawLayers>(ctx) {
      {
        let layers = engine::fetch_resource::<DrawLayers>(ctx);

        for layer in &layers.vec {
          self.buffer.push(layer.clone());
        }
      }

      self
        .canvas
        .clear(graphics::Color::new(0.53, 0.87, 0.52, 1.0));

      for layer in &self.buffer {
        layer.draw(ctx, &mut self.canvas);
      }

      self.buffer.clear();
      self.canvas.present();
    }
  }
}

/// Adds a draw layer to the given engine context.
pub fn add_draw_layer(ctx: &mut engine::Context, layer: impl DrawLayer) {
  // Create the `DrawLayers` resource if it has not yet been created.
  if !engine::has_resource::<DrawLayers>(ctx) {
    engine::add_resource(ctx, DrawLayers::default());
  }

  // Add the layer.
  engine::fetch_resource_mut::<DrawLayers>(ctx)
    .vec
    .push(Arc::new(layer));
}
