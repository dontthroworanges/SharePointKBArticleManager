//! The page: the editor (Tiptap, in `web/`) in a WebView2, and the message channel to it.
//!
//! The app and the page exchange JSON messages, listed at the top of `web/editor.js`. Until
//! the page has loaded and said it is ready, messages for it wait here, in order.

use crate::webview::*;
use serde_json::Value;
use std::path::PathBuf;
use windows_core::{Interface, Result};

/// The page is served from its folder under this name, a reserved test domain that never
/// collides with a real site.
const HOST: &str = "kb-editor.example";
const START: &str = "https://kb-editor.example/index.html";

#[derive(Default)]
pub struct Page {
    core: Option<CoreWebView2>,
    /// The WebView2 has been asked for its core.
    pub requested: bool,
    ready: bool,
    outbox: Vec<Value>,
    _events: Vec<windows_core::EventRevoker>,
}

impl Page {
    /// Sets the page up once the WebView2 has its core: what it may do, where its files are,
    /// and where its messages go (`on_message`, with the message as JSON).
    pub fn attach(
        &mut self,
        core: CoreWebView2,
        on_message: impl Fn(String) + 'static,
    ) -> Result<()> {
        let settings = core.Settings()?;
        // The browser's own menu stays: it has the spelling suggestions and the clipboard.
        settings.SetAreDefaultContextMenusEnabled(true)?;
        // A link's address shows at the bottom while the pointer is on it.
        settings.SetIsStatusBarEnabled(true)?;
        // Zoom is the app's (the page lays itself out zoomed); Ctrl+wheel comes to the app.
        settings.SetIsZoomControlEnabled(false)?;
        settings.SetAreDevToolsEnabled(cfg!(debug_assertions) || crate::log::enabled())?;
        // No browser keys (find, print, reload); editing keys still work.
        if let Ok(settings) = settings.cast::<ICoreWebView2Settings3>() {
            settings.SetAreBrowserAcceleratorKeysEnabled(false)?;
        }
        let folder = web_folder();
        crate::log::log!("page served from {}", folder.display());
        core.cast::<ICoreWebView2_3>()?.SetVirtualHostNameToFolderMapping(
            HOST,
            &folder.to_string_lossy(),
            CoreWebView2HostResourceAccessKind::Allow,
        )?;
        self._events.push(core.WebMessageReceived(move |_, args| {
            if let Some(args) = args.as_ref()
                && let Ok(json) = args.WebMessageAsJson()
            {
                on_message(json);
            }
        })?);
        // The page never leaves the editor: a link or a dropped file opens outside it.
        self._events.push(core.NavigationStarting(|_, args| {
            let Some(args) = args.as_ref() else { return };
            let Ok(uri) = args.Uri() else { return };
            if !uri.starts_with(&format!("https://{HOST}/")) {
                _ = args.SetCancel(true);
                super::open_url(&uri);
            }
        })?);
        self._events.push(core.NewWindowRequested(|_, args| {
            let Some(args) = args.as_ref() else { return };
            _ = args.SetHandled(true);
            if let Ok(uri) = args.Uri() {
                super::open_url(&uri);
            }
        })?);
        core.Navigate(START)?;
        self.core = Some(core);
        Ok(())
    }

    /// The page has loaded: what was waiting for it goes now.
    pub fn set_ready(&mut self) {
        self.ready = true;
        for message in std::mem::take(&mut self.outbox) {
            self.post(message);
        }
    }

    pub fn post(&mut self, message: Value) {
        match (&self.core, self.ready) {
            (Some(core), true) => {
                if let Err(error) = core.PostWebMessageAsJson(&message.to_string()) {
                    crate::log::log!("posting to the page failed: {error:?}");
                }
            }
            _ => self.outbox.push(message),
        }
    }
}

/// Before the window opens: where WebView2 keeps its data (next to the exe would fail under
/// Program Files), and a clear background so the window shows around the page.
pub fn prepare_environment() {
    let data = PathBuf::from(std::env::var_os("LOCALAPPDATA").unwrap_or_default())
        .join(super::APP_NAME)
        .join("WebView2");
    // SAFETY: set before any other thread starts (the window and WebView2 come later).
    unsafe {
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", data);
        std::env::set_var("WEBVIEW2_DEFAULT_BACKGROUND_COLOR", "0");
    }
}

/// The editor's files, built into the exe.
const FILES: [(&str, &[u8]); 4] = [
    ("index.html", include_bytes!("../../web/index.html")),
    ("editor.css", include_bytes!("../../web/editor.css")),
    ("editor.js", include_bytes!("../../web/editor.js")),
    ("vendor/tiptap.js", include_bytes!("../../web/vendor/tiptap.js")),
];

/// Where the page is served from: `KB_WEB_DIR` if set, the source tree's `web/` in a debug
/// build (edit and reload, no rebuild), or else a copy of the built-in files under
/// %LOCALAPPDATA%, written when they differ.
fn web_folder() -> PathBuf {
    if let Some(folder) = std::env::var_os("KB_WEB_DIR") {
        return PathBuf::from(folder);
    }
    let tree = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/web"));
    if cfg!(debug_assertions) && tree.join("index.html").exists() {
        return tree;
    }
    let folder = PathBuf::from(std::env::var_os("LOCALAPPDATA").unwrap_or_default())
        .join(super::APP_NAME)
        .join("web");
    for (name, contents) in FILES {
        let path = folder.join(name);
        if std::fs::read(&path).ok().as_deref() != Some(contents) {
            if let Some(parent) = path.parent() {
                _ = std::fs::create_dir_all(parent);
            }
            if let Err(error) = std::fs::write(&path, contents) {
                crate::log::log!("could not write {}: {error}", path.display());
            }
        }
    }
    folder
}
