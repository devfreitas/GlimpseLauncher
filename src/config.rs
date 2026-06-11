// src/config.rs
//! Configuration handling for Glimpse Launcher.
//!
//! The launcher can be customized via a simple `config.toml` placed in the user
//! configuration directory (`%APPDATA%/GlimpseLauncher`).  The struct is
//! deserialized with `serde` + `toml`.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Top‑level configuration options.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Hot‑key definition, e.g. "Alt+S".  The format is parsed by the
    /// `hotkey` module.
    pub hotkey: Option<String>,
    /// Path to a custom icon (ICO file).  If `None` the default bundled icon is
    /// used.
    pub icon_path: Option<String>,
    /// Theme settings – currently only background opacity.
    pub theme: Option<ThemeConfig>,
}

/// Simple theme configuration.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeConfig {
    /// Background colour in RGBA (0‑255).  The last component is the alpha.
    pub background_rgba: Option<[u8; 4]>,
    /// Optional blur radius (currently unused – placeholder for future
    /// extensions).
    pub blur_radius: Option<f32>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            background_rgba: Some([20, 20, 22, 200]),
            blur_radius: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hotkey: Some("Alt+S".to_string()),
            icon_path: None,
            theme: Some(ThemeConfig {
                background_rgba: Some([20, 20, 22, 200]),
                blur_radius: None,
            }),
        }
    }
}

/// Resolve the configuration file location.
fn config_path() -> PathBuf {
    let mut base = dirs::config_dir().unwrap_or_else(|| Path::new(".").to_path_buf());
    base.push("GlimpseLauncher");
    base.push("config.toml");
    base
}

/// Load the configuration, falling back to defaults if the file does not exist
/// or cannot be parsed.
pub fn load() -> Config {
    let path = config_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(toml_str) => match toml::from_str::<Config>(&toml_str) {
                Ok(cfg) => cfg,
                Err(err) => {
                    eprintln!("Failed to parse config.toml: {err}. Using defaults.");
                    Config::default()
                }
            },
            Err(err) => {
                eprintln!("Failed to read config.toml: {err}. Using defaults.");
                Config::default()
            }
        }
    } else {
        // Write a default file so the user can edit it.
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(
            &path,
            toml::to_string_pretty(&Config::default()).unwrap_or_default(),
        );
        Config::default()
    }
}
