use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Hot‑key "Alt+S".
    pub hotkey: Option<String>,
    pub icon_path: Option<String>,
    pub theme: Option<ThemeConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeConfig {
    pub background_rgba: Option<[u8; 4]>,
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

fn config_path() -> PathBuf {
    let mut base = dirs::config_dir().unwrap_or_else(|| Path::new(".").to_path_buf());
    base.push("GlimpseLauncher");
    base.push("config.toml");
    base
}

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