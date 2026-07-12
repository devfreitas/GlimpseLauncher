use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use winreg::enums::*;
use winreg::RegKey;

const REGISTRY_RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const APP_REGISTRY_NAME: &str = "GlimpseLauncher";

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
    pub position_x: Option<f32>,
    pub position_y: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ThemeConfig {
    pub background_rgba: Option<[u8; 4]>,
    pub blur_radius: Option<f32>,
    pub accent_color_index: Option<usize>,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        ThemeConfig {
            background_rgba: Some([20, 20, 22, 200]),
            blur_radius: None,
            accent_color_index: Some(0),
        }
    }
}

impl ThemeConfig {
    /// Creates the default dark theme.
    pub fn dark() -> Self {
        Self {
            background_rgba: Some([20, 20, 22, 200]),
            blur_radius: None,
            accent_color_index: Some(0),
        }
    }

    /// Creates the default light theme.
    pub fn light() -> Self {
        Self {
            background_rgba: Some([240, 240, 245, 230]),
            blur_radius: None,
            accent_color_index: Some(0),
        }
    }

    /// Returns `true` if this theme is considered dark.
    pub fn is_dark(&self) -> bool {
        self.background_rgba.map_or(true, |rgba| rgba[0] < 100)
    }

    /// Returns the opposite theme (dark ↔ light).
    pub fn toggle(&self) -> Self {
        if self.is_dark() {
            Self::light()
        } else {
            Self::dark()
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
                accent_color_index: Some(0),
            }),
            start_with_windows: Some(false),
            enable_calculator: Some(true),
            enable_web_search: Some(true),
            enable_commands: Some(true),
            position_x: Some(0.5),
            position_y: Some(0.25),
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
                if let Ok(run_key) = hkcu.open_subkey_with_flags(
                    "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
                    KEY_WRITE,
                ) {
                    let _ = run_key
                        .set_value("GlimpseLauncher", &exe_path.to_string_lossy().to_string());
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

pub fn is_autostart_enabled() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run) = hkcu.open_subkey(REGISTRY_RUN_KEY) {
        let val: Result<String, _> = run.get_value(APP_REGISTRY_NAME);
        return val.is_ok();
    }
    false
}

pub fn toggle_autostart(enable: bool) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(run) = hkcu.open_subkey_with_flags(REGISTRY_RUN_KEY, KEY_SET_VALUE) {
        if enable {
            if let Ok(exe) = std::env::current_exe() {
                let path_str = exe.to_string_lossy();
                let quoted_path = format!("\"{}\"", path_str);
                let _ = run.set_value(APP_REGISTRY_NAME, &quoted_path);
            }
        } else {
            let _ = run.delete_value(APP_REGISTRY_NAME);
        }
    }
}
