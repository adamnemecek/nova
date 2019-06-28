use nova::math::Size;
use serde_derive::*;

/// An error that occurred while parsing a TOML config file.
pub type TomlError = toml::de::Error;

/// Configuration options for the game.
#[derive(Debug, Serialize, Deserialize)]
pub struct Options {
  /// Configuration options for the main window.
  pub window: WindowOptions,
}

/// Configuration options for the main window.
#[derive(Debug, Serialize, Deserialize)]
pub struct WindowOptions {
  /// Width of the window in pixels.
  pub width: Option<f64>,
  /// Height of the window in pixels.
  pub height: Option<f64>,
  /// Whether the window is freely resizable.
  #[serde(default = "resizable_default_value")]
  pub resizable: bool,
}

impl WindowOptions {
  /// Returns the configured window size if both width and height are set.
  pub fn size(&self) -> Option<Size<f64>> {
    Some(Size::new(self.width?, self.height?))
  }
}

/// Parses configuration options from a string containing TOML.
pub fn from_toml(source: &str) -> Result<Options, TomlError> {
  toml::from_str(source)
}

/// Provides the default value of `WindowOptions::resizable`.
fn resizable_default_value() -> bool {
  true
}
