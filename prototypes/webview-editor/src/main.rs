#![windows_subsystem = "windows"]

//! Prototype: the KB article editor's text surface as Tiptap (ProseMirror) in a WebView2,
//! with the toolbar, files and Markdown kept native.
//!
//! The app's Markdown model is compiled in read-only from `src/markdown`; `bridge` turns it
//! into the editor's JSON and back. `web/` holds the editor page.
//!
//! `kb-webview-prototype [file.md]` opens a file. `--self-test file.md` opens it, checks the
//! round trip, runs a typing benchmark, writes the results to the log and exits.

#[allow(dead_code, unused_imports)]
#[path = "../../../src/markdown/mod.rs"]
mod markdown;

mod bridge;
mod webview;

use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use webview::*;
use windows_core::Interface;
use windows_reactor::*;

/// The editor page is served from `web/` under this name (a reserved test domain, so it never
/// collides with a real site).
const HOST: &str = "kb-editor.example";

fn main() {
    let mut args = std::env::args_os().skip(1);
    let mut input = Input::default();
    while let Some(arg) = args.next() {
        if arg == "--self-test" {
            input.self_test = true;
        } else {
            input.path = Some(PathBuf::from(arg));
        }
    }
    log(&format!("start: {input:?}"));
    if let Err(error) = App::run_component::<Prototype>(input) {
        log(&format!("failed to start: {error}"));
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Input {
    path: Option<PathBuf>,
    self_test: bool,
}

#[derive(Clone)]
enum Msg {
    Init,
    Core(CoreWebView2),
    Failed(String),
    Web(String),
    Command(&'static str),
    New,
    Open,
    Save,
    SaveAs,
    Check,
    Bench,
    DevTools,
}

/// What a requested copy of the editor's document is for.
enum Purpose {
    Save(PathBuf),
    Check,
}

struct Prototype {
    web: ElementRef<WebView2>,
    core: Option<CoreWebView2>,
    requested: bool,
    _message_handler: Option<windows_core::EventRevoker>,
    /// The page has loaded and listens for messages.
    ready: bool,
    /// Messages for the page until it is ready.
    outbox: Vec<Value>,
    path: Option<PathBuf>,
    /// The open file's front matter, which the editor does not show; put back on save.
    properties: markdown::Properties,
    /// The open file as read, and as the current app would save it unchanged.
    original: String,
    baseline: String,
    dirty: bool,
    requests: HashMap<u64, Purpose>,
    next_request: u64,
    self_test: bool,
    status: String,
    selection: String,
    typing: String,
}

impl Component for Prototype {
    type Input = Input;
    type Message = Msg;

    fn create(input: &Input, _: &ComponentContext<Self>) -> Self {
        let mut prototype = Self {
            web: ElementRef::new(),
            core: None,
            requested: false,
            _message_handler: None,
            ready: false,
            outbox: Vec::new(),
            path: None,
            properties: markdown::Properties::None,
            original: String::new(),
            baseline: String::new(),
            dirty: false,
            requests: HashMap::new(),
            next_request: 1,
            self_test: input.self_test,
            status: "Starting the editor…".into(),
            selection: String::new(),
            typing: String::new(),
        };
        match &input.path {
            Some(path) => prototype.load(path),
            None => prototype.new_document(),
        }
        prototype
    }

    fn update(&mut self, message: Msg, context: &ComponentContext<Self>) {
        match message {
            Msg::Init => self.init(context),
            Msg::Core(core) => {
                if let Err(error) = self.attach(core, context) {
                    self.fail(format!("The editor page could not be set up: {error}"));
                }
            }
            Msg::Failed(error) => self.fail(error),
            Msg::Web(json) => self.received(&json),
            Msg::Command(name) => self.post(json!({ "type": "command", "name": name })),
            Msg::New => {
                self.new_document();
            }
            Msg::Open => {
                if let Some(path) = file_dialog().add_filter("Markdown", &["md"]).pick_file() {
                    self.load(&path);
                }
            }
            Msg::Save => match self.path.clone() {
                Some(path) => self.request(Purpose::Save(path)),
                None => self.save_as(),
            },
            Msg::SaveAs => self.save_as(),
            Msg::Check => self.request(Purpose::Check),
            Msg::Bench => {
                self.status = "Typing benchmark running…".into();
                self.post(json!({ "type": "bench" }));
            }
            Msg::DevTools => {
                if let Some(core) = &self.core {
                    _ = core.OpenDevToolsWindow();
                }
            }
        }
    }

    fn view(&self, _: &Input, context: &mut ViewContext<Self>) -> View {
        let name = self
            .path
            .as_deref()
            .and_then(Path::file_name)
            .map_or("Untitled".into(), |name| name.to_string_lossy().into_owned());
        context.window_title(format!(
            "{}{name} – Tiptap prototype",
            if self.dirty { "• " } else { "" }
        ));
        context.window_visuals(WindowVisuals::new().client_size(1280.0, 860.0));
        let sender = context.sender();
        context.use_effect("webview", (), move || {
            sender.send(Msg::Init);
            None
        });

        let button = |label: &str, tip: &str, message: Msg| -> View {
            Button::new()
                .on_click(context.message(message))
                .content(TextBlock::new().text(label))
                .tooltip(tip)
        };
        // (label, tooltip, message); an empty label is a gap between groups.
        let tools_row: [(&str, &str, Option<Msg>); 23] = [
            ("New", "New (Ctrl+N)", Some(Msg::New)),
            ("Open…", "Open (Ctrl+O)", Some(Msg::Open)),
            ("Save", "Save (Ctrl+S)", Some(Msg::Save)),
            ("Save as…", "Save as (Ctrl+Shift+S)", Some(Msg::SaveAs)),
            ("", "", None),
            ("Normal", "Normal text (Ctrl+Shift+N)", Some(Msg::Command("paragraph"))),
            ("H1", "Heading 1 (Ctrl+Alt+1)", Some(Msg::Command("heading1"))),
            ("H2", "Heading 2 (Ctrl+Alt+2)", Some(Msg::Command("heading2"))),
            ("H3", "Heading 3 (Ctrl+Alt+3)", Some(Msg::Command("heading3"))),
            ("", "", None),
            ("B", "Bold (Ctrl+B)", Some(Msg::Command("bold"))),
            ("I", "Italic (Ctrl+I)", Some(Msg::Command("italic"))),
            ("S", "Strikethrough (Ctrl+Shift+X)", Some(Msg::Command("strike"))),
            ("Highlight", "Highlight (Ctrl+Shift+H)", Some(Msg::Command("highlight"))),
            ("Code", "Inline code (Ctrl+Alt+C)", Some(Msg::Command("code"))),
            ("Link", "Link (Ctrl+K)", Some(Msg::Command("link"))),
            ("", "", None),
            ("• List", "Bulleted list (Ctrl+Shift+L)", Some(Msg::Command("bulletList"))),
            ("1. List", "Numbered list", Some(Msg::Command("orderedList"))),
            ("☐ Tasks", "Task list", Some(Msg::Command("taskList"))),
            ("Quote", "Quote", Some(Msg::Command("blockquote"))),
            ("{ }", "Code block", Some(Msg::Command("codeBlock"))),
            ("Table", "Insert a table", Some(Msg::Command("table"))),
        ];
        let toolbar = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(4.0)
            .margin(Thickness::xy(8.0, 6.0))
            .keyed_children(
                tools_row
                    .into_iter()
                    .enumerate()
                    .map(|(index, (label, tip, message))| {
                        let view = match message {
                            Some(message) => button(label, tip, message),
                            None => Border::new().width(12.0).into(),
                        };
                        (index.to_string(), view)
                    })
                    .collect::<Vec<_>>(),
            );
        let tools = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(4.0)
            .margin(Thickness::new(8.0, 0.0, 8.0, 6.0))
            .children((
                button(
                    "Check round trip",
                    "Compare what the editor would save with what the current app saves",
                    Msg::Check,
                ),
                button(
                    "Typing benchmark",
                    "Type 300 characters at the end of the page and time each frame",
                    Msg::Bench,
                ),
                button("Dev tools", "Open the page's developer tools", Msg::DevTools),
            ));
        let status = [
            self.status.as_str(),
            self.selection.as_str(),
            self.typing.as_str(),
        ]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("   ·   ");
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR, GridLength::Auto])
            .children((
                Border::new().grid_row(0).content(
                    StackPanel::new()
                        .orientation(Orientation::Vertical)
                        .children((toolbar, tools)),
                ),
                WebView2::new().element_ref(&self.web).grid_row(1),
                Border::new()
                    .grid_row(2)
                    .padding(Thickness::xy(12.0, 6.0))
                    .content(TextBlock::new().text(status).font_size(12.0)),
            ))
    }
}

impl Prototype {
    /// Asks Reactor for the WebView2's core once the control is on screen.
    fn init(&mut self, context: &ComponentContext<Self>) {
        if self.requested || self.core.is_some() {
            return;
        }
        let sender = context.sender();
        self.requested = self.web.request_core_web_view2(move |result| {
            sender.send(match result {
                Ok(object) => match object.cast::<CoreWebView2>() {
                    Ok(core) => Msg::Core(core),
                    Err(error) => Msg::Failed(format!("Not a CoreWebView2: {error}")),
                },
                Err(error) => Msg::Failed(format!("WebView2 failed to start: {error:?}")),
            });
        });
        if !self.requested {
            self.fail("The WebView2 control was not on screen when the editor started.".into());
        }
    }

    fn attach(
        &mut self,
        core: CoreWebView2,
        context: &ComponentContext<Self>,
    ) -> windows_core::Result<()> {
        let settings = core.Settings()?;
        settings.SetAreDevToolsEnabled(true)?;
        // Kept: the browser's menu has the spelling suggestions.
        settings.SetAreDefaultContextMenusEnabled(true)?;
        // Shows a link's address at the bottom while the pointer is on it.
        settings.SetIsStatusBarEnabled(true)?;
        let folder = web_folder();
        log(&format!("serving {}", folder.display()));
        core.cast::<ICoreWebView2_3>()?.SetVirtualHostNameToFolderMapping(
            HOST,
            &folder.to_string_lossy(),
            CoreWebView2HostResourceAccessKind::Allow,
        )?;
        let sender = context.sender();
        self._message_handler = Some(core.WebMessageReceived(move |_, args| {
            if let Some(args) = args.as_ref()
                && let Ok(json) = args.WebMessageAsJson()
            {
                sender.send(Msg::Web(json));
            }
        })?);
        core.Navigate(&format!("https://{HOST}/index.html"))?;
        self.core = Some(core);
        Ok(())
    }

    fn received(&mut self, json: &str) {
        let Ok(message) = serde_json::from_str::<Value>(json) else {
            log(&format!("unreadable message: {json}"));
            return;
        };
        match message["type"].as_str().unwrap_or_default() {
            "ready" => {
                log("page ready");
                self.ready = true;
                self.status = "Ready".into();
                for message in std::mem::take(&mut self.outbox) {
                    self.post(message);
                }
                if self.self_test {
                    self.request(Purpose::Check);
                }
            }
            "changed" => self.dirty = true,
            "state" => self.selection = message["text"].as_str().unwrap_or_default().into(),
            "typing" => self.typing = message["text"].as_str().unwrap_or_default().into(),
            "doc" => {
                let id = message["id"].as_u64().unwrap_or_default();
                match self.requests.remove(&id) {
                    Some(Purpose::Save(path)) => self.save(&path, &message["doc"]),
                    Some(Purpose::Check) => {
                        self.check(&message["doc"]);
                        if self.self_test {
                            self.post(json!({ "type": "bench" }));
                        }
                    }
                    None => {}
                }
            }
            "bench" => {
                let text = message["text"].as_str().unwrap_or_default().to_string();
                log(&format!("benchmark: {text}"));
                self.status = text;
                if self.self_test {
                    log("self-test done");
                    std::process::exit(0);
                }
            }
            "shortcut" => match message["name"].as_str().unwrap_or_default() {
                "save" => match self.path.clone() {
                    Some(path) => self.request(Purpose::Save(path)),
                    None => self.save_as(),
                },
                "saveAs" => self.save_as(),
                "open" => {
                    if let Some(path) = file_dialog().add_filter("Markdown", &["md"]).pick_file() {
                        self.load(&path);
                    }
                }
                "new" => self.new_document(),
                _ => {}
            },
            "log" => log(&format!("page: {}", message["text"].as_str().unwrap_or_default())),
            other => log(&format!("unknown message {other}")),
        }
    }

    fn fail(&mut self, error: String) {
        log(&error);
        self.status = error;
        if self.self_test {
            std::process::exit(1);
        }
    }

    fn post(&mut self, message: Value) {
        match (&self.core, self.ready) {
            (Some(core), true) => {
                if let Err(error) = core.PostWebMessageAsJson(&message.to_string()) {
                    log(&format!("posting to the page failed: {error}"));
                }
            }
            _ => self.outbox.push(message),
        }
    }

    fn request(&mut self, purpose: Purpose) {
        let id = self.next_request;
        self.next_request += 1;
        self.requests.insert(id, purpose);
        self.post(json!({ "type": "get", "id": id }));
    }

    fn new_document(&mut self) {
        self.path = None;
        self.properties = markdown::Properties::None;
        self.original.clear();
        self.baseline.clear();
        self.dirty = false;
        let doc = bridge::to_pm(&markdown::Document::default());
        self.post(json!({ "type": "load", "doc": doc }));
    }

    fn load(&mut self, path: &Path) {
        let text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(error) => {
                self.fail(format!("Could not open {}: {error}", path.display()));
                return;
            }
        };
        let started = std::time::Instant::now();
        let document = markdown::parse(&text);
        self.baseline = markdown::serialize(&document);
        self.properties = document.properties.clone();
        let doc = bridge::to_pm(&document);
        log(&format!(
            "opened {} ({} bytes) in {:.1} ms",
            path.display(),
            text.len(),
            started.elapsed().as_secs_f64() * 1000.0
        ));
        self.original = text;
        self.path = Some(path.to_path_buf());
        self.dirty = false;
        self.status = format!("Opened {}", path.display());
        self.post(json!({ "type": "load", "doc": doc }));
    }

    fn markdown(&self, doc: &Value) -> String {
        markdown::serialize(&markdown::Document {
            properties: self.properties.clone(),
            blocks: bridge::from_pm(doc),
        })
    }

    fn save_as(&mut self) {
        if let Some(path) = file_dialog().add_filter("Markdown", &["md"]).save_file() {
            self.request(Purpose::Save(path));
        }
    }

    fn save(&mut self, path: &Path, doc: &Value) {
        let text = self.markdown(doc);
        match std::fs::write(path, &text) {
            Ok(()) => {
                self.path = Some(path.to_path_buf());
                self.dirty = false;
                self.status = format!("Saved {}", path.display());
                self.post(json!({ "type": "saved" }));
            }
            Err(error) => self.fail(format!("Could not save {}: {error}", path.display())),
        }
    }

    /// Compares what the editor would save with what the current app saves for the file as
    /// opened. Run it before editing: after edits they differ by the edits.
    fn check(&mut self, doc: &Value) {
        let output = self.markdown(doc);
        let written = std::env::temp_dir().join("kb-webview-roundtrip.md");
        _ = std::fs::write(&written, &output);
        let against_app = first_difference(&self.baseline, &output);
        let against_file = first_difference(&self.original, &output);
        self.status = match against_app {
            None => format!(
                "Round trip: identical to the current app's output{}",
                match against_file {
                    None => " and to the file".to_string(),
                    Some(line) => format!(" (both differ from the file from line {line})"),
                }
            ),
            Some(line) => format!(
                "Round trip: differs from the current app's output at line {line} (editor output in {})",
                written.display()
            ),
        };
        log(&self.status);
    }
}

/// The first line (from 1) where two texts differ.
fn first_difference(left: &str, right: &str) -> Option<usize> {
    if left == right {
        return None;
    }
    let mut left_lines = left.lines();
    let mut right_lines = right.lines();
    let mut line = 1;
    loop {
        match (left_lines.next(), right_lines.next()) {
            (Some(a), Some(b)) if a == b => line += 1,
            (None, None) => return Some(line),
            _ => return Some(line),
        }
    }
}

/// `web/` next to the executable, or in the source tree when run with `cargo run`.
fn web_folder() -> PathBuf {
    let beside = std::env::current_exe()
        .ok()
        .and_then(|exe| Some(exe.parent()?.join("web")));
    match beside {
        Some(folder) if folder.join("index.html").exists() => folder,
        _ => PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/web")),
    }
}

/// Appends a line to `kb-webview-prototype.log` in the temp folder (or `KB_PROTO_LOG`).
fn log(line: &str) {
    use std::io::Write;
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let elapsed = START.get_or_init(std::time::Instant::now).elapsed().as_millis();
    let path = std::env::var_os("KB_PROTO_LOG")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join("kb-webview-prototype.log"));
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        _ = writeln!(file, "{elapsed:>7} {line}");
    }
}

/// A file dialog owned by the window, so it is modal.
fn file_dialog() -> rfd::FileDialog {
    let dialog = rfd::FileDialog::new();
    match Owner::active() {
        Some(owner) => dialog.set_parent(&owner),
        None => dialog,
    }
}

struct Owner(std::num::NonZeroIsize);

impl Owner {
    fn active() -> Option<Self> {
        #[link(name = "user32")]
        unsafe extern "system" {
            fn GetActiveWindow() -> isize;
        }
        // SAFETY: GetActiveWindow has no preconditions; it returns 0 when there is none.
        std::num::NonZeroIsize::new(unsafe { GetActiveWindow() }).map(Self)
    }
}

impl raw_window_handle::HasWindowHandle for Owner {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let handle = raw_window_handle::Win32WindowHandle::new(self.0);
        // SAFETY: the window is the app's own and outlives the modal dialog that borrows it.
        Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle.into()) })
    }
}

impl raw_window_handle::HasDisplayHandle for Owner {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        Ok(raw_window_handle::DisplayHandle::windows())
    }
}
