//! Язык сообщений бэкенда (ошибки, трей). Переключается из настроек.

use std::sync::atomic::{AtomicBool, Ordering};

static EN: AtomicBool = AtomicBool::new(false);

pub fn set_lang(lang: &str) {
    EN.store(lang == "en", Ordering::Relaxed);
}

pub fn en() -> bool {
    EN.load(Ordering::Relaxed)
}

/// `tr!("по-русски {x}", "in English {x}")` → String на текущем языке.
#[macro_export]
macro_rules! tr {
    ($ru:literal, $en:literal) => {
        if $crate::i18n::en() {
            format!($en)
        } else {
            format!($ru)
        }
    };
}
