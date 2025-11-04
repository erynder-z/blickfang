use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone)]
pub struct Shortcut {
    pub keys: Vec<String>,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shortcuts {
    pub open_file: Shortcut,
    pub previous_image: Shortcut,
    pub next_image: Shortcut,
    pub zoom_in: Shortcut,
    pub zoom_out: Shortcut,
    pub toggle_fullscreen: Shortcut,
    pub toggle_exif: Shortcut,
    pub toggle_options: Shortcut,
    pub zoom_modifier_up: Shortcut,
    pub zoom_modifier_down: Shortcut,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_shortcuts")]
    pub shortcuts: Shortcuts,
    #[serde(default = "default_toolbar_button_size")]
    pub toolbar_button_size: String,
    #[serde(default = "default_shortcuts")]
    pub custom_shortcuts: Shortcuts,
    #[serde(default = "default_image_name_display_mode")]
    pub image_name_display_mode: String,
    #[serde(default = "default_edge_indicators_visible")]
    pub edge_indicators_visible: bool,
    #[serde(default = "default_remember_window_size")]
    pub remember_window_size: bool,
    #[serde(default)]
    pub window_width: Option<f64>,
    #[serde(default)]
    pub window_height: Option<f64>,
    #[serde(default)]
    pub window_x: Option<f64>,
    #[serde(default)]
    pub window_y: Option<f64>,
}

fn default_language() -> String {
    "en".into()
}
fn default_theme() -> String {
    "default".into()
}
fn default_toolbar_button_size() -> String {
    "large".into()
}
fn default_image_name_display_mode() -> String {
    "fade".into()
}
fn default_edge_indicators_visible() -> bool {
    false
}
fn default_remember_window_size() -> bool {
    false
}

pub fn default_shortcuts() -> Shortcuts {
    Shortcuts {
        open_file: Shortcut {
            keys: vec!["o".into()],
            label: "o".into(),
        },
        previous_image: Shortcut {
            keys: vec!["ArrowLeft".into()],
            label: "←".into(),
        },
        next_image: Shortcut {
            keys: vec!["ArrowRight".into()],
            label: "→".into(),
        },
        zoom_in: Shortcut {
            keys: vec!["+".into(), "=".into(), "ArrowUp".into()],
            label: "+ / ↑".into(),
        },
        zoom_out: Shortcut {
            keys: vec!["-".into(), "_".into(), "ArrowDown".into()],
            label: "- / ↓".into(),
        },
        toggle_fullscreen: Shortcut {
            keys: vec!["f".into()],
            label: "f".into(),
        },
        toggle_exif: Shortcut {
            keys: vec!["i".into()],
            label: "i".into(),
        },
        toggle_options: Shortcut {
            keys: vec!["m".into()],
            label: "m".into(),
        },
        zoom_modifier_up: Shortcut {
            keys: vec!["Control".into()],
            label: "Control".into(),
        },
        zoom_modifier_down: Shortcut {
            keys: vec!["Alt".into()],
            label: "Alt / Command".into(),
        },
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: default_language(),
            theme: default_theme(),
            toolbar_button_size: default_toolbar_button_size(),
            image_name_display_mode: default_image_name_display_mode(),
            shortcuts: default_shortcuts(),
            custom_shortcuts: default_shortcuts(),
            edge_indicators_visible: default_edge_indicators_visible(),
            remember_window_size: default_remember_window_size(),
            window_width: None,
            window_height: None,
            window_x: None,
            window_y: None,
        }
    }
}

fn get_config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let mut config_path = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    config_path.push("blickfang");
    fs::create_dir_all(&config_path)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;
    config_path.push("config.json");
    Ok(config_path)
}

pub fn read_config(app: &AppHandle) -> Result<String, String> {
    let config_path = get_config_path(app)?;

    if !config_path.exists() {
        let default_config = Config::default();
        let default_str = serde_json::to_string_pretty(&default_config)
            .map_err(|e| format!("Failed to serialize default config: {}", e))?;
        fs::write(&config_path, &default_str)
            .map_err(|e| format!("Failed to write default config file: {}", e))?;
        return Ok(default_str);
    }

    let raw = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: Config = serde_json::from_str(&raw).unwrap_or_else(|_| Config::default());

    let normalized = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to reserialize normalized config: {}", e))?;

    if normalized != raw {
        let _ = fs::write(&config_path, &normalized);
    }

    Ok(normalized)
}

pub fn write_config(app: &AppHandle, content: &str) -> Result<(), String> {
    let config_path = get_config_path(app)?;
    fs::write(config_path, content).map_err(|e| format!("Failed to write config file: {}", e))?;
    Ok(())
}
