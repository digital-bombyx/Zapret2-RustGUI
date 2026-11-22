use std::path::Path;
use std::process::{Command, Output};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// CREATE_NO_WINDOW: дочерние консольные процессы без мигающих окон.
pub const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Команда без консольного окна.
pub fn hidden(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    cmd
}

/// Запустить скрытую команду и вернуть вывод.
pub fn run_hidden(program: &str, args: &[&str]) -> std::io::Result<Output> {
    hidden(program).args(args).output()
}

/// Путь к системному exe (System32), чтобы не зависеть от PATH.
pub fn system32(exe: &str) -> String {
    let root = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into());
    let p = Path::new(&root).join("System32").join(exe);
    if p.is_file() {
        p.to_string_lossy().into_owned()
    } else {
        exe.to_string()
    }
}

/// Прочитать текстовый файл, терпимо к BOM и не-UTF8.
pub fn read_text(path: &Path) -> std::io::Result<String> {
    let bytes = std::fs::read(path)?;
    let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(&bytes);
    Ok(String::from_utf8_lossy(bytes).into_owned())
}

/// Запись с CRLF, как пишут bat-скрипты.
pub fn write_crlf(path: &Path, lines: &[String]) -> std::io::Result<()> {
    let mut s = String::new();
    for l in lines {
        s.push_str(l);
        s.push_str("\r\n");
    }
    std::fs::write(path, s)
}

/// Секунды Unix-времени.
pub fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
