//! Game Filter: порт game_switch_status / validate_game_filter_range /
//! save_game_filter_settings из service.bat. Файл utils\game_filter.enabled.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const DEFAULT_RANGE: &str = "1024-65535";
/// Порт-заглушка, когда фильтр выключен (как в bat).
pub const OFF_PORT: &str = "12";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameFilter {
    /// disabled | all | tcp | udp
    pub mode: String,
    pub tcp: String,
    pub udp: String,
}

impl Default for GameFilter {
    fn default() -> Self {
        GameFilter { mode: "disabled".into(), tcp: DEFAULT_RANGE.into(), udp: DEFAULT_RANGE.into() }
    }
}

fn file(dir: &Path) -> PathBuf {
    dir.join("utils").join("game_filter.enabled")
}

pub fn load(dir: &Path) -> GameFilter {
    let mut gf = GameFilter::default();
    let Ok(text) = crate::util::read_text(&file(dir)) else {
        return gf;
    };
    let mut tcp_c = String::new();
    let mut udp_c = String::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (k, v) = match line.split_once('=') {
            Some((k, v)) => (k.trim(), v.trim()),
            None => (line, ""),
        };
        match k.to_ascii_lowercase().as_str() {
            "mode" => gf.mode = v.to_ascii_lowercase(),
            "all" => gf.mode = "all".into(),
            "udp" if v.is_empty() => gf.mode = "udp".into(),
            "udp" => udp_c = v.into(),
            "tcp" if v.is_empty() => gf.mode = "tcp".into(),
            "tcp" => tcp_c = v.into(),
            _ => {}
        }
    }
    if let Some(r) = validate_range(&tcp_c) {
        gf.tcp = r;
    }
    if let Some(r) = validate_range(&udp_c) {
        gf.udp = r;
    }
    if !matches!(gf.mode.as_str(), "all" | "tcp" | "udp") {
        gf.mode = "disabled".into();
    }
    gf
}

pub fn save(dir: &Path, gf: &GameFilter) -> Result<(), String> {
    let tcp = validate_range(&gf.tcp).ok_or_else(|| crate::tr!("некорректный TCP-диапазон", "invalid TCP port range"))?;
    let udp = validate_range(&gf.udp).ok_or_else(|| crate::tr!("некорректный UDP-диапазон", "invalid UDP port range"))?;
    let mode = match gf.mode.as_str() {
        m @ ("disabled" | "all" | "tcp" | "udp") => m,
        _ => return Err(crate::tr!("некорректный режим", "invalid mode")),
    };
    crate::util::write_crlf(&file(dir), &[format!("mode={mode}"), format!("tcp={tcp}"), format!("udp={udp}")])
        .map_err(|e| e.to_string())
}

/// Значения %GameFilterTCP% и %GameFilterUDP%.
pub fn vars(gf: &GameFilter) -> (String, String) {
    match gf.mode.as_str() {
        "all" => (gf.tcp.clone(), gf.udp.clone()),
        "tcp" => (gf.tcp.clone(), OFF_PORT.into()),
        "udp" => (OFF_PORT.into(), gf.udp.clone()),
        _ => (OFF_PORT.into(), OFF_PORT.into()),
    }
}

/// `1024-1934,1936-65535` → Some(без пробелов), иначе None.
pub fn validate_range(input: &str) -> Option<String> {
    let s: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    if s.is_empty() {
        return None;
    }
    for item in s.split(',') {
        let (a, b) = item.split_once('-').unwrap_or((item, item));
        let parse = |x: &str| -> Option<u32> {
            if x.is_empty() || x.len() > 5 || x.starts_with('0') || !x.bytes().all(|c| c.is_ascii_digit()) {
                return None;
            }
            x.parse().ok().filter(|n| *n <= 65535)
        };
        let (a, b) = (parse(a)?, parse(b)?);
        if a > b {
            return None;
        }
    }
    Some(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranges() {
        assert_eq!(validate_range(" 1024-1934, 1936-65535 ").as_deref(), Some("1024-1934,1936-65535"));
        assert_eq!(validate_range("443").as_deref(), Some("443"));
        assert!(validate_range("0-10").is_none());
        assert!(validate_range("70000").is_none());
        assert!(validate_range("200-100").is_none());
        assert!(validate_range("1-2-3").is_none());
        assert!(validate_range("").is_none());
    }

    #[test]
    fn vars_by_mode() {
        let mut gf = GameFilter::default();
        assert_eq!(vars(&gf), ("12".into(), "12".into()));
        gf.mode = "udp".into();
        assert_eq!(vars(&gf), ("12".into(), DEFAULT_RANGE.into()));
    }
}
