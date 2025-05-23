use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub theme: Theme,
    pub heartbeat_interval: u64,
    pub afk_timeout: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            theme: Theme::System,
            heartbeat_interval: 10,
            afk_timeout: 120,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

static CONFIG_FILENAME: &str = "config.json";
pub static CONFIG: LazyLock<Mutex<AppConfig>> = LazyLock::new(|| Mutex::new(AppConfig::default()));

impl AppConfig {
    pub fn load<R: Runtime>(handle: &AppHandle<R>) -> io::Result<Self> {
        let path = Self::get_config_path(handle);
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            let config: Self = serde_json::from_str(&contents)?;
            Ok(config)
        } else {
            let default = Self::default();
            default.save(handle)?;
            Ok(default)
        }
    }

    pub fn save<R: Runtime>(&self, handle: &AppHandle<R>) -> io::Result<()> {
        let path = Self::get_config_path(handle);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let serialized = serde_json::to_string_pretty(&self)?;
        let mut file = fs::File::create(path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn get_config_path<R: Runtime>(handle: &AppHandle<R>) -> PathBuf {
        handle
            .path()
            .app_config_dir()
            .unwrap_or_else(|_| std::env::temp_dir())
            .join(CONFIG_FILENAME)
    }

    pub fn get() -> AppConfig {
        CONFIG.lock().unwrap().clone()
    }

    pub fn update<R: Runtime, F>(handle: &AppHandle<R>, updater: F) -> io::Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = CONFIG.lock().unwrap();
        updater(&mut config);
        config.save(handle)
    }
}

#[tauri::command]
pub fn get_config() -> AppConfig {
    AppConfig::get()
}

#[tauri::command]
pub fn set_heartbeat_interval<R: Runtime>(interval: u64, app: AppHandle<R>) -> Result<(), String> {
    AppConfig::update(&app, |config| {
        config.heartbeat_interval = interval;
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_theme<R: Runtime>(theme: Theme, app: AppHandle<R>) -> Result<(), String> {
    AppConfig::update(&app, |config| {
        config.theme = theme;
    })
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_afk_timeout<R: Runtime>(timeout: u64, app: AppHandle<R>) -> Result<(), String> {
    AppConfig::update(&app, |config| {
        config.afk_timeout = timeout;
    })
    .map_err(|e| e.to_string())
}
