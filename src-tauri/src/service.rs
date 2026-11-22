//! Служба `zapret`: установка и удаление по образцу service.bat
//! (service_install / service_remove), но через SCM API, без экранирования для sc.

use std::ffi::OsString;
use std::path::Path;
use std::time::{Duration, Instant};
use windows_service::service::{
    ServiceAccess, ServiceErrorControl, ServiceInfo, ServiceStartType, ServiceState, ServiceType,
};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

pub const NAME: &str = "zapret";
const REG_KEY: &str = r"HKLM\System\CurrentControlSet\Services\zapret";
const REG_VALUE: &str = "zapret-discord-youtube";

fn manager(create: bool) -> Result<ServiceManager, String> {
    let mut access = ServiceManagerAccess::CONNECT;
    if create {
        access |= ServiceManagerAccess::CREATE_SERVICE;
    }
    ServiceManager::local_computer(None::<&str>, access).map_err(|e| format!("SCM: {e}"))
}

/// installed, running
pub fn state(name: &str) -> (bool, bool) {
    let Ok(m) = manager(false) else { return (false, false) };
    match m.open_service(name, ServiceAccess::QUERY_STATUS) {
        Ok(s) => {
            let running = s
                .query_status()
                .map(|st| matches!(st.current_state, ServiceState::Running | ServiceState::StartPending))
                .unwrap_or(false);
            (true, running)
        }
        Err(_) => (false, false),
    }
}

/// Имя стратегии, которое service.bat (и мы) пишет в реестр при установке.
pub fn installed_strategy() -> Option<String> {
    let out = crate::util::run_hidden(&crate::util::system32("reg.exe"), &["query", REG_KEY, "/v", REG_VALUE]).ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    text.lines()
        .find(|l| l.contains("REG_SZ"))
        .and_then(|l| l.split("REG_SZ").nth(1))
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// Остановить и удалить службу, дождаться фактического удаления.
pub fn stop_and_delete(name: &str) -> Result<bool, String> {
    let m = manager(false)?;
    let Ok(s) = m.open_service(name, ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE)
    else {
        return Ok(false);
    };
    if let Ok(st) = s.query_status() {
        if st.current_state != ServiceState::Stopped {
            let _ = s.stop();
            let t = Instant::now();
            while t.elapsed() < Duration::from_secs(8) {
                if s.query_status().map(|x| x.current_state == ServiceState::Stopped).unwrap_or(true) {
                    break;
                }
                std::thread::sleep(Duration::from_millis(150));
            }
        }
    }
    s.delete().map_err(|e| crate::tr!("удаление службы {name}: {e}", "deleting service {name}: {e}"))?;
    drop(s);
    let t = Instant::now();
    while t.elapsed() < Duration::from_secs(5) && m.open_service(name, ServiceAccess::QUERY_STATUS).is_ok() {
        std::thread::sleep(Duration::from_millis(150));
    }
    Ok(true)
}

/// Установить и запустить службу zapret с аргументами стратегии.
pub fn install(dir: &Path, strategy_name: &str, args: &[String]) -> Result<(), String> {
    let _ = stop_and_delete(NAME);
    let exe = dir.join("bin").join("winws2.exe");
    let mut launch: Vec<OsString> = vec!["--chdir".into()];
    launch.extend(args.iter().map(OsString::from));
    let info = ServiceInfo {
        name: NAME.into(),
        display_name: "zapret".into(),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: exe,
        launch_arguments: launch,
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };
    let m = manager(true)?;
    let s = m
        .create_service(&info, ServiceAccess::CHANGE_CONFIG | ServiceAccess::START | ServiceAccess::QUERY_STATUS)
        .map_err(|e| crate::tr!("создание службы: {e}", "creating service: {e}"))?;
    let _ = s.set_description("Zapret DPI bypass software");
    let _ = crate::util::run_hidden(
        &crate::util::system32("reg.exe"),
        &["add", REG_KEY, "/v", REG_VALUE, "/t", "REG_SZ", "/d", strategy_name, "/f"],
    );
    s.start::<&str>(&[]).map_err(|e| crate::tr!("запуск службы: {e}", "starting service: {e}"))?;
    Ok(())
}

/// Полное удаление, как «Remove Services»: zapret, процессы winws, драйвер WinDivert.
pub fn remove_all() -> Result<(), String> {
    stop_and_delete(NAME)?;
    crate::winapi::kill_by_name(&["winws.exe", "winws2.exe"]);
    let _ = stop_and_delete("WinDivert");
    let _ = stop_and_delete("WinDivert14");
    Ok(())
}
