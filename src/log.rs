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
        // Milliseconds since the first entry, to see how long things take.
        static START: OnceLock<std::time::Instant> = OnceLock::new();
        let elapsed = START.get_or_init(std::time::Instant::now).elapsed().as_millis();
        _ = writeln!(file, "{elapsed:>7} {line}");
    }
}

/// Whether a diagnostic switch is on: the environment variable `name` is set to anything but
/// `0`. Used to turn suspects off one at a time while measuring typing speed.
pub fn switch(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| value != "0")
}

/// Whether timing is being recorded (`KB_EDITOR_LOG` is set).
pub fn enabled() -> bool {
    path().is_some()
}

macro_rules! log {
    ($($arg:tt)*) => { $crate::log::write(format_args!($($arg)*)) };
}
pub(crate) use log;
