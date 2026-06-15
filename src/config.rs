use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Hot‑key "Alt+S".
    pub hotkey: Option<String>,
    pub start_with_windows: Option<bool>,
    pub icon_path: Option<String>,
    pub theme: Option<ThemeConfig>,
    pub enable_calculator: Option<bool>,
    pub enable_web_search: Option<bool>,
    pub enable_commands: Option<bool>,
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
            start_with_windows: Some(false),
            enable_calculator: Some(true),
            enable_web_search: Some(true),
            enable_commands: Some(true),
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
        let cfg = Config::default();
        if cfg.start_with_windows.unwrap_or(false) {
            if let Ok(exe_path) = std::env::current_exe() {
                let hkcu = RegKey::predef(HKEY_CURRENT_USER);
                if let Ok(run_key) = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE) {
                    let _ = run_key.set_value("GlimpseLauncher", &exe_path.to_string_lossy().to_string());
                }
            }
        }
        cfg
    }
}

pub fn save(config: &Config) -> std::io::Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let toml_str = toml::to_string_pretty(config).unwrap_or_default();
    fs::write(&path, toml_str)
}