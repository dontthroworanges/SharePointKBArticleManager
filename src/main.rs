#![windows_subsystem = "windows"]

//! SharePoint Article Manager: a native, Word-style editor for SharePoint Knowledge Base
//! Markdown pages.

mod app;
mod bridge;
mod log;
mod markdown;
mod webview;
mod winui;

fn main() {
    app::prepare();
    // A file passed on the command line (for example from "Open with") is opened at startup.
    let path = std::env::args_os().nth(1).map(std::path::PathBuf::from);
    if let Err(error) = windows_reactor::App::run_component::<app::Editor>(path) {
        eprintln!("SharePoint Article Manager failed to start: {error}");
    }
}
