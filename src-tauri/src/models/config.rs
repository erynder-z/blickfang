use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Shortcut {
    pub keys: Vec<String>,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shortcuts {
    pub open_file: Shortcut,
    pub save_image_as: Shortcut,
    pub previous_image: Shortcut,
    pub next_image: Shortcut,
    pub zoom_in: Shortcut,
    pub zoom_out: Shortcut,
    pub rotate_clockwise: Shortcut,
    pub rotate_counterclockwise: Shortcut,
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
    #[serde(default = "default_has_configured_initial_settings")]
    pub has_configured_initial_settings: bool,
}

fn default_has_configured_initial_settings() -> bool {
    false
}


fn default_language() -> String {
    "en".into()
}
fn default_theme() -> String {
    "lowest-common-denominator".into()
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
        save_image_as: Shortcut {
            keys: vec!["c".into()],
            label: "c".into(),
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
        rotate_clockwise: Shortcut {
            keys: vec!["t".into()],
            label: "t".into(),
        },
        rotate_counterclockwise: Shortcut {
            keys: vec!["r".into()],
            label: "r".into(),
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
            has_configured_initial_settings: default_has_configured_initial_settings(),
        }
    }
}
