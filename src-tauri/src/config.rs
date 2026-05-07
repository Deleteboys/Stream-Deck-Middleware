// src/config.rs
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AppConfig {
    pub spotify_client_id: Option<String>,
}

pub fn get_config_path(app: &AppHandle) -> Option<PathBuf> {
    let mut path = app.path().app_data_dir().ok()?;
    let _ = fs::create_dir_all(&path);
    path.push("config.json");
    Some(path)
}

pub fn load_config(app: &AppHandle) -> AppConfig {
    get_config_path(app)
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}

pub fn save_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let path = get_config_path(app).ok_or("Konnte Konfigurationspfad nicht finden")?;
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}