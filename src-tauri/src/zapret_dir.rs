use std::path::{Path, PathBuf};

/// Папка сборки zapret-discord-youtube валидна, если в ней есть winws2 и списки.
pub fn is_valid(p: &Path) -> bool {
    p.join("bin").join("winws2.exe").is_file() && p.join("lists").is_dir() && p.join("lua").is_dir()
}

/// Поиск папки: своя из настроек → встроенная → рядом с exe → известные пути.
pub fn detect(configured: Option<&str>, bundled: Option<&Path>) -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Some(c) = configured.filter(|c| !c.trim().is_empty()) {
        candidates.push(PathBuf::from(c));
    }
    if let Some(b) = bundled {
        candidates.push(b.to_path_buf());
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("zapret"));
            candidates.push(dir.join("zapret-discord-youtube"));
            candidates.push(dir.to_path_buf());
            if let Some(parent) = dir.parent() {
                candidates.push(parent.join("zapret-discord-youtube"));
            }
        }
    }
    candidates.push(PathBuf::from(r"D:\zapret-discord-youtube"));
    candidates.push(PathBuf::from(r"C:\zapret-discord-youtube"));
    candidates.into_iter().find(|p| is_valid(p))
}

/// Версия сборки из .service/version.txt.
pub fn version(dir: &Path) -> Option<String> {
    crate::util::read_text(&dir.join(".service").join("version.txt"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
