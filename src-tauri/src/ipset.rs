//! IPSet: порт ipset_switch_status / ipset_switch из service.bat.
//! loaded — реальный список; none — заглушка 203.0.113.113/32; any — пустой файл.

use std::path::{Path, PathBuf};

pub const PLACEHOLDER: &str = "203.0.113.113/32";

fn list_file(dir: &Path) -> PathBuf {
    dir.join("lists").join("ipset-all.txt")
}
fn backup_file(dir: &Path) -> PathBuf {
    dir.join("lists").join("ipset-all.txt.backup")
}
/// Бэкап, который делает тест на время DPI-проверки (как в test zapret.ps1).
pub fn test_backup_file(dir: &Path) -> PathBuf {
    dir.join("lists").join("ipset-all.test-backup.txt")
}

pub fn status(dir: &Path) -> &'static str {
    match crate::util::read_text(&list_file(dir)) {
        Err(_) => "none",
        Ok(t) if t.trim().is_empty() => "any",
        Ok(t) if t.contains(PLACEHOLDER) => "none",
        Ok(_) => "loaded",
    }
}

pub fn has_backup(dir: &Path) -> bool {
    backup_file(dir).is_file()
}

/// Перевести в нужный режим. Реальный список при уходе из loaded сохраняется в .backup.
pub fn set(dir: &Path, target: &str) -> Result<(), String> {
    let cur = status(dir);
    if cur == target {
        return Ok(());
    }
    let list = list_file(dir);
    let backup = backup_file(dir);
    let io = |e: std::io::Error| e.to_string();
    if cur == "loaded" && target != "loaded" {
        std::fs::copy(&list, &backup).map_err(io)?;
    }
    match target {
        "none" => crate::util::write_crlf(&list, &[PLACEHOLDER.to_string()]).map_err(io),
        "any" => std::fs::write(&list, b"").map_err(io),
        "loaded" => {
            if !backup.is_file() {
                return Err(crate::tr!("нет сохранённого списка (ipset-all.txt.backup). Обновите IPSet через service.bat", "no saved list (ipset-all.txt.backup). Update IPSet via service.bat"));
            }
            std::fs::copy(&backup, &list).map_err(io)?;
            std::fs::remove_file(&backup).map_err(io)
        }
        _ => Err(crate::tr!("некорректный режим", "invalid mode")),
    }
}

/// На время DPI-теста: сохранить текущий файл и сделать его пустым (any).
pub fn test_switch_any(dir: &Path) -> Result<(), String> {
    let list = list_file(dir);
    let tb = test_backup_file(dir);
    if !tb.exists() {
        if list.is_file() {
            std::fs::copy(&list, &tb).map_err(|e| e.to_string())?;
        } else {
            std::fs::write(&tb, b"").map_err(|e| e.to_string())?;
        }
    }
    std::fs::write(&list, b"").map_err(|e| e.to_string())
}

/// Вернуть файл после теста или после падения приложения во время теста.
pub fn test_restore(dir: &Path) {
    let tb = test_backup_file(dir);
    if tb.is_file() {
        let _ = std::fs::copy(&tb, list_file(dir)).and_then(|_| std::fs::remove_file(&tb));
    }
}
