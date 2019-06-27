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
  pub width: f64,
  /// Height of the window in pixels.
  pub height: f64,
  /// Whether the window is freely resizable.
  #[serde(default = "default_resizable")]
  pub resizable: bool,
}

/// Parses configuration options from a string containing TOML.
pub fn from_toml(source: &str) -> Result<Options, TomlError> {
  toml::from_str(source)
}

/// Provides the default value of `WindowOptions::resizable`.
fn default_resizable() -> bool {
  true
}
