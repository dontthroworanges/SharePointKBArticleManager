#![doc = include_str!("../readme.md")]

mod app;
mod core;

/// kb-editor patch: timing lines appended to the file named by `KB_EDITOR_LOG` (the app's
/// development log), so the pump's work per keystroke can be measured in a release build.
pub(crate) fn perf_enabled() -> bool {
    perf_path().is_some()
}

fn perf_path() -> Option<&'static str> {
    static PATH: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
    PATH.get_or_init(|| std::env::var("KB_EDITOR_LOG").ok()).as_deref()
}

pub(crate) fn perf_log(line: std::fmt::Arguments) {
    use std::io::Write;
    let Some(path) = perf_path() else { return };
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        _ = writeln!(file, "        [reactor] {line}");
    }
}
mod element;
mod generated;
mod native;
mod reference;
#[cfg(any(test, feature = "test"))]
pub mod test;

use generated::*;
#[cfg(test)]
pub(crate) use test::*;

pub use app::*;
pub use core::public::*;
pub use element::*;
pub use generated::public::*;
pub use reference::{
    set_window_close_guard, CompositionHostError, CompositionHostEvent, ElementObservation, ElementRef, FocusControl,
    FocusError, ImageSourceError, IntegrationError, ReferenceControl, SwapChainPanelError,
    SwapChainPanelEvent, WebView2Error, WindowRef,
};
pub use windows_time::{DateTime, TimeSpan};
