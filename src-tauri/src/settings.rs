use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,                  // "dark", "light", "auto"
    pub language: String,               // "tr", "en"
    pub default_flatpak_scope: String,  // "system", "user"
    pub auto_ping_on_load: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            language: "tr".to_string(),
            default_flatpak_scope: "system".to_string(),
            auto_ping_on_load: true,
        }
    }
}

pub fn get_settings_path() -> PathBuf {
    let mut p = dirs::config_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    p.push("lupus-repo-manager");
    let _ = fs::create_dir_all(&p);
    p.push("settings.json");
    p
}

pub fn load_settings() -> AppSettings {
    let p = get_settings_path();
    if let Ok(content) = fs::read_to_string(&p) {
        if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
            return settings;
        }
    }
    AppSettings::default()
}

pub fn save_settings(settings: &AppSettings) -> Result<(), String> {
    let p = get_settings_path();
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(&p, content).map_err(|e| e.to_string())?;
    Ok(())
}
