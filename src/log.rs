//! Development log: set `KB_EDITOR_LOG` to a file path to record editor events.

use std::io::Write;
use std::sync::OnceLock;

fn path() -> Option<&'static str> {
    static PATH: OnceLock<Option<String>> = OnceLock::new();
    PATH.get_or_init(|| std::env::var("KB_EDITOR_LOG").ok())
        .as_deref()
}

pub fn write(line: std::fmt::Arguments) {
    let Some(path) = path() else { return };
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        _ = writeln!(file, "{line}");
    }
}

macro_rules! log {
    ($($arg:tt)*) => { $crate::log::write(format_args!($($arg)*)) };
}
pub(crate) use log;
