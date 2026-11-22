//! Пользовательские списки: list-general-user, list-exclude-user, ipset-exclude-user.
//! Значения по умолчанию — как в :load_user_lists (файлы нельзя оставлять пустыми).

use serde::Serialize;
use std::path::{Path, PathBuf};

pub const KINDS: [&str; 3] = ["general", "exclude", "ipset-exclude"];

fn path(dir: &Path, kind: &str) -> Result<PathBuf, String> {
    let name = match kind {
        "general" => "list-general-user.txt",
        "exclude" => "list-exclude-user.txt",
        "ipset-exclude" => "ipset-exclude-user.txt",
        _ => return Err(crate::tr!("неизвестный список", "unknown list")),
    };
    Ok(dir.join("lists").join(name))
}

fn default_lines(kind: &str) -> Vec<String> {
    match kind {
        "general" => vec!["# Never leave this file empty".into(), "domain.example.abc".into()],
        "exclude" => vec!["domain.example.abc".into()],
        _ => vec![crate::ipset::PLACEHOLDER.into()],
    }
}

/// Создать отсутствующие пользовательские списки (иначе winws2 не стартует).
pub fn ensure(dir: &Path) {
    for kind in KINDS {
        if let Ok(p) = path(dir, kind) {
            if !p.exists() {
                let _ = crate::util::write_crlf(&p, &default_lines(kind));
            }
        }
    }
}

pub fn read(dir: &Path, kind: &str) -> Result<String, String> {
    let p = path(dir, kind)?;
    if !p.exists() {
        return Ok(default_lines(kind).join("\n"));
    }
    crate::util::read_text(&p).map(|t| t.replace("\r\n", "\n")).map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct SaveResult {
    /// Строки, которые не похожи на домен / IP / CIDR (сохраняются всё равно).
    pub warnings: Vec<String>,
}

pub fn write(dir: &Path, kind: &str, content: &str) -> Result<SaveResult, String> {
    let p = path(dir, kind)?;
    let mut lines: Vec<String> = content.lines().map(|l| l.trim().to_string()).collect();
    while lines.last().is_some_and(|l| l.is_empty()) {
        lines.pop();
    }
    let meaningful = lines.iter().any(|l| !l.is_empty() && !l.starts_with('#'));
    if !meaningful {
        lines.extend(default_lines(kind).into_iter().filter(|l| !l.starts_with('#')));
    }
    let warnings = lines
        .iter()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter(|l| if kind == "ipset-exclude" { !is_ip_or_cidr(l) } else { !is_domain(l) })
        .cloned()
        .collect();
    crate::util::write_crlf(&p, &lines).map_err(|e| e.to_string())?;
    Ok(SaveResult { warnings })
}

fn is_domain(s: &str) -> bool {
    let s = s.trim_end_matches('.');
    s.contains('.')
        && s.len() <= 253
        && s.split('.').all(|l| {
            !l.is_empty()
                && l.len() <= 63
                && !l.starts_with('-')
                && !l.ends_with('-')
                && l.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        })
}

fn is_ip_or_cidr(s: &str) -> bool {
    let (ip, mask) = s.split_once('/').unwrap_or((s, ""));
    let Ok(addr) = ip.parse::<std::net::IpAddr>() else {
        return false;
    };
    if mask.is_empty() {
        return true;
    }
    let max = if addr.is_ipv4() { 32 } else { 128 };
    mask.parse::<u8>().is_ok_and(|m| m <= max)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validators() {
        assert!(is_domain("discord.com"));
        assert!(is_domain("xn--80ak6aa92e.xn--p1ai"));
        assert!(!is_domain("localhost"));
        assert!(!is_domain("bad domain.com"));
        assert!(is_ip_or_cidr("1.2.3.0/24"));
        assert!(is_ip_or_cidr("2606:4700::/32"));
        assert!(!is_ip_or_cidr("1.2.3.0/33"));
        assert!(!is_ip_or_cidr("example.com"));
    }
}
