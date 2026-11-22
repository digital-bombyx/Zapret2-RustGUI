use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub zapret_dir: Option<String>,
    /// Выбранная стратегия (имя bat-файла).
    pub strategy: Option<String>,
    /// Обход был включён при выходе — включить снова при старте.
    pub last_enabled: bool,
    /// process | service
    pub mode: String,
    pub close_to_tray: bool,
    pub test_timeout: u32,
    pub test_parallel: usize,
    pub hosts_custom: Vec<crate::hosts::Entry>,
    /// ru | en
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            zapret_dir: None,
            strategy: None,
            last_enabled: false,
            mode: "process".into(),
            close_to_tray: true,
            test_timeout: 4,
            test_parallel: 16,
            hosts_custom: vec![],
            language: "ru".into(),
        }
    }
}

pub fn path(config_dir: &Path) -> PathBuf {
    config_dir.join("settings.json")
}

pub fn load(config_dir: &Path) -> Settings {
    std::fs::read(path(config_dir))
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .unwrap_or_default()
}

pub fn save(config_dir: &Path, s: &Settings) {
    let _ = std::fs::create_dir_all(config_dir);
    if let Ok(json) = serde_json::to_vec_pretty(s) {
        let tmp = path(config_dir).with_extension("json.tmp");
        if std::fs::write(&tmp, json).is_ok() {
            let _ = std::fs::rename(&tmp, path(config_dir));
        }
    }
}
