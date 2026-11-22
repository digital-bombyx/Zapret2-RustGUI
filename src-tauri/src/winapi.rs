//! Тонкие обёртки над WinAPI: процессы, ожидание, права, ICMP-пинг.

use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::System::Threading::{
    OpenProcess, TerminateProcess, WaitForSingleObject, INFINITE, PROCESS_SYNCHRONIZE, PROCESS_TERMINATE,
};

/// Запущено ли приложение с правами администратора.
pub fn is_admin() -> bool {
    unsafe { windows_sys::Win32::UI::Shell::IsUserAnAdmin() != 0 }
}

/// PID всех процессов с указанными именами exe (без учёта регистра).
pub fn find_processes(names: &[&str]) -> Vec<u32> {
    let mut out = Vec::new();
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap == INVALID_HANDLE_VALUE {
            return out;
        }
        let mut e: PROCESSENTRY32W = std::mem::zeroed();
        e.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;
        if Process32FirstW(snap, &mut e) != 0 {
            loop {
                let len = e.szExeFile.iter().position(|c| *c == 0).unwrap_or(e.szExeFile.len());
                let name = String::from_utf16_lossy(&e.szExeFile[..len]);
                if names.iter().any(|n| n.eq_ignore_ascii_case(&name)) {
                    out.push(e.th32ProcessID);
                }
                if Process32NextW(snap, &mut e) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snap);
    }
    out
}

/// Завершить процессы по имени. Возвращает число завершённых.
pub fn kill_by_name(names: &[&str]) -> usize {
    let mut n = 0;
    for pid in find_processes(names) {
        unsafe {
            let h = OpenProcess(PROCESS_TERMINATE | PROCESS_SYNCHRONIZE, 0, pid);
            if !h.is_null() {
                if TerminateProcess(h, 1) != 0 {
                    WaitForSingleObject(h, 3000);
                    n += 1;
                }
                CloseHandle(h);
            }
        }
    }
    n
}

/// Дескриптор процесса для ожидания его завершения в отдельном потоке.
pub struct ProcWait(HANDLE);
unsafe impl Send for ProcWait {}

impl ProcWait {
    pub fn open(pid: u32) -> Option<Self> {
        let h = unsafe { OpenProcess(PROCESS_SYNCHRONIZE, 0, pid) };
        (!h.is_null()).then_some(ProcWait(h))
    }
    /// Блокирует поток до завершения процесса.
    pub fn wait(&self) {
        unsafe {
            WaitForSingleObject(self.0, INFINITE);
        }
    }
}

impl Drop for ProcWait {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

/// ICMP-пинг IPv4, время в мс или None при таймауте.
pub fn ping(addr: std::net::Ipv4Addr, timeout_ms: u32) -> Option<u32> {
    use windows_sys::Win32::NetworkManagement::IpHelper::{
        IcmpCloseHandle, IcmpCreateFile, IcmpSendEcho, ICMP_ECHO_REPLY,
    };
    unsafe {
        let h = IcmpCreateFile();
        if h == INVALID_HANDLE_VALUE {
            return None;
        }
        let data = [0x61u8; 32];
        let mut reply = vec![0u8; std::mem::size_of::<ICMP_ECHO_REPLY>() + data.len() + 8 + 64];
        // IcmpSendEcho ждёт адрес в сетевом порядке байт.
        let ip = u32::from_ne_bytes(addr.octets());
        let n = IcmpSendEcho(
            h,
            ip,
            data.as_ptr() as *const _,
            data.len() as u16,
            std::ptr::null(),
            reply.as_mut_ptr() as *mut _,
            reply.len() as u32,
            timeout_ms,
        );
        IcmpCloseHandle(h);
        if n == 0 {
            return None;
        }
        let r = &*(reply.as_ptr() as *const ICMP_ECHO_REPLY);
        (r.Status == 0).then_some(r.RoundTripTime)
    }
}
