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
    pub toggle_exif: Shortcut,
    pub toggle_options: Shortcut,
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
    #[serde(default = "default_button_size")]
    pub button_size: String,
    #[serde(default = "default_shortcuts")]
    pub custom_shortcuts: Shortcuts,
    #[serde(default = "default_image_name_display_mode")]
    pub image_name_display_mode: String,
}

fn default_language() -> String {
    "en".to_string()
}

fn default_theme() -> String {
    "default".to_string()
}

fn default_button_size() -> String {
    "large".to_string()
}

fn default_image_name_display_mode() -> String {
    "fade".to_string()
}

pub fn default_shortcuts() -> Shortcuts {
    Shortcuts {
        open_file: Shortcut {
            keys: vec!["o".to_string()],
            label: "o".to_string(),
        },
        previous_image: Shortcut {
            keys: vec!["ArrowLeft".to_string()],
            label: "←".to_string(),
        },
        next_image: Shortcut {
            keys: vec!["ArrowRight".to_string()],
            label: "→".to_string(),
        },
        zoom_in: Shortcut {
            keys: vec!["+".to_string(), "=".to_string(), "ArrowUp".to_string()],
            label: "+ / ↑".to_string(),
        },
        zoom_out: Shortcut {
            keys: vec!["-".to_string(), "_".to_string(), "ArrowDown".to_string()],
            label: "- / ↓".to_string(),
        },
        toggle_exif: Shortcut {
            keys: vec!["i".to_string()],
            label: "i".to_string(),
        },
        toggle_options: Shortcut {
            keys: vec!["m".to_string()],
            label: "m".to_string(),
        },
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: default_language(),
            theme: default_theme(),
            button_size: default_button_size(),
            image_name_display_mode: default_image_name_display_mode(),
            shortcuts: default_shortcuts(),
            custom_shortcuts: default_shortcuts(),
        }
    }
}

fn get_config_path(app: &AppHandle) -> PathBuf {
    let mut config_path = app.path().app_data_dir().unwrap();
    config_path.push("simple-image-viewer");
    fs::create_dir_all(&config_path).unwrap();
    config_path.push("config.json");
    config_path
}

pub fn read_config(app: &AppHandle) -> String {
    let config_path = get_config_path(app);
    if !config_path.exists() {
        let default_config = Config::default();
        let default_config_str = serde_json::to_string(&default_config).unwrap();
        fs::write(&config_path, default_config_str).unwrap();
    }

    let config_str = fs::read_to_string(&config_path).unwrap_or_else(|_| "{}".to_string());
    let config: Config = serde_json::from_str(&config_str).unwrap_or_default();
    serde_json::to_string(&config).unwrap()
}

pub fn write_config(app: &AppHandle, content: &str) {
    let config_path = get_config_path(app);
    fs::write(config_path, content).unwrap();
}


