use serde::{Deserialize, Serialize};
use std::{fs, io};

use eframe::egui::Key;

use crate::hyprland::SearchOptions;
use crate::ui::types::{ColumnVisibility, Theme};

use super::paths::{config_dir, config_path};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UserConfig {
    pub theme: Theme,
    pub column_visibility: ColumnVisibility,
    pub search_options: SearchOptions,
    pub zen_mode: bool,
    pub search_keybind: Key,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            column_visibility: ColumnVisibility::default(),
            search_options: SearchOptions::default(),
            zen_mode: false,
            search_keybind: Key::Backslash,
        }
    }
}

pub fn load() -> Option<UserConfig> {
    let path = config_path();
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

pub fn save(cfg: &UserConfig) -> io::Result<()> {
    let dir = config_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }
    let path = config_path();
    let data = serde_json::to_string_pretty(cfg).map_err(|e| io::Error::other(e.to_string()))?;
    fs::write(path, data)
}
