use std::sync::atomic::{AtomicU8, Ordering};

pub static LANG: AtomicU8 = AtomicU8::new(0); // 0 = EN, 1 = ES

pub fn set_language(is_es: bool) {
    LANG.store(if is_es { 1 } else { 0 }, Ordering::Relaxed);
}

pub fn is_spanish() -> bool {
    LANG.load(Ordering::Relaxed) == 1
}

pub fn toggle_language() {
    let current = LANG.load(Ordering::Relaxed);
    LANG.store(if current == 1 { 0 } else { 1 }, Ordering::Relaxed);
}

#[macro_export]
macro_rules! t {
    (en: $en:expr, es: $es:expr) => {
        if $crate::i18n::is_spanish() {
            $es
        } else {
            $en
        }
    };
}
