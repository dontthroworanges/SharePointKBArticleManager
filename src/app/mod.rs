//! The editor window: document state, commands, and file handling.
//!
//! The page itself (the text, and everything typed into it) lives in `web/` and runs in a
//! WebView2 (see `page`). The app keeps the document's front matter, files, the ribbon and the
//! panes, and talks to the page in messages: it sends commands and documents, and the page
//! reports edits, the selection's formatting, headings and search results.

mod keys;
mod page;
mod settings;
mod spelling;
mod styles_box;
mod theme;
mod view;

use crate::bridge;
use crate::markdown::*;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use windows_reactor::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    Visual,
    Source,
}

/// The ribbon layouts of Word for the web, chosen with the chevron at the end of the ribbon.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RibbonLayout {
    /// Groups of larger buttons, with group names: the expanded ribbon.
    Classic,
    /// One row of small buttons.
    #[default]
    SingleLine,
}

/// The names in the chevron menu.
pub const RIBBON_LAYOUTS: [(&str, RibbonLayout); 2] = [
    ("Classic Ribbon", RibbonLayout::Classic),
    ("Single Line Ribbon", RibbonLayout::SingleLine),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tab {
    /// New, open and save, as a ribbon tab.
    File,
    Home,
    Insert,
    View,
    /// Help: the About box and links to help pages.
    Help,
}

/// Keyboard shortcuts the app handles, from the page or from the app's own controls. The
/// page handles its editing keys itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shortcut {
    Save,
    SaveAs,
    Open,
    New,
    Link,
    Find,
    Replace,
    /// F3 and Shift+F3: the next or previous search result.
    FindNext,
    FindPrevious,
    ZoomIn,
    ZoomOut,
    ZoomReset,
}

impl Shortcut {
    fn command(self) -> Command {
        match self {
            Self::Save => Command::Save,
            Self::SaveAs => Command::SaveAs,
            Self::Open => Command::Open,
            Self::New => Command::New,
            Self::Link => Command::Link,
            // Without a search to step through, F3 opens Find, as in Word.
            Self::Find | Self::FindNext | Self::FindPrevious => Command::Search(ExplorerTab::Find),
            Self::Replace => Command::Search(ExplorerTab::Replace),
            Self::ZoomIn => Command::Zoom(Zoom::In),
            Self::ZoomOut => Command::Zoom(Zoom::Out),
            Self::ZoomReset => Command::Zoom(Zoom::To(100)),
        }
    }
}

/// Character formatting the ribbon toggles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InlineCommand {
    Bold,
    Italic,
    Strikethrough,
    Highlight,
    Code,
    Math,
}

impl InlineCommand {
    /// The page's name for it (see `run` in `web/editor.js`).
    fn mark(self) -> &'static str {
        match self {
            Self::Bold => "bold",
            Self::Italic => "italic",
            Self::Strikethrough => "strike",
            Self::Highlight => "highlight",
            Self::Code => "code",
            Self::Math => "math",
        }
    }
}

/// The formatting where the cursor is, as the page reports it, for the ribbon.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SelectionState {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub highlight: bool,
    pub code: bool,
    pub kind: ParagraphKind,
    pub list: Option<ListKind>,
    pub has_selection: bool,
}

/// A heading in the page: its level, text, and the position to put the cursor at.
#[derive(Clone, Debug, PartialEq)]
pub struct HeadingInfo {
    pub level: u8,
    pub text: String,
    pub position: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Command {
    New,
    Open,
    Save,
    SaveAs,
    /// File › Rename: rename the open file where it is.
    Rename,
    /// Help › About: Settings › About (the version, what the app is, where it lives).
    About,
    /// Open a web page in the default browser.
    OpenUrl(&'static str),
    Undo,
    Redo,
    Inline(InlineCommand),
    Kind(ParagraphKind),
    List(ListKind),
    ToggleDone,
    Indent,
    Outdent,
    Link,
    PageLink,
    Insert(Insert),
    ToggleProperties,
    ToggleNavigation,
    /// Open the Document Editor on its Find or Replace tab (Ctrl+F, Ctrl+H).
    Search(ExplorerTab),
    Zoom(Zoom),
    /// View › Theme.
    Theme(theme::ThemeChoice),
    /// View › Switch Background: a dark page, or white again.
    SwitchBackground,
    /// Open the Document Editor on one of its tools, or close it if that tool is showing.
    Explorer(ExplorerTab),
    Mode(Mode),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Insert {
    Table,
    Image,
    Video,
    Code,
    Mermaid,
    Equation,
    Divider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DialogKind {
    Link,
    PageLink,
    Image,
    Video,
    /// A picture or video's address and alt text, for the one at this page position.
    EditMedia(i32),
    Discard(Pending),
    /// File › Rename: the new file name.
    Rename,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pending {
    New,
    Open,
    Close,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dialog {
    pub kind: DialogKind,
    pub fields: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    /// The page's WebView2 is on screen: start it.
    PageInit,
    /// The WebView2 has its core.
    PageCore(crate::webview::CoreWebView2),
    /// A message from the page, as JSON.
    Page(String),
    Failed(String),
    /// A window-wide shortcut, from wherever focus is.
    AppShortcut(Shortcut),
    Command(Command),
    /// A click in the ribbon's "More" menu, by label.
    MoreMenu(String),
    StyleChosen(Option<usize>),
    Tab(Option<String>),
    PropertyKey(usize, String),
    PropertyValue(usize, String),
    AddProperty,
    RemoveProperty(usize),
    RawProperties(String),
    DialogField(usize, String),
    DialogClosed(ContentDialogResult),
    CloseRequested,
    /// The problem bar above the page was closed.
    DismissProblem,
    WindowSize(WindowSize),
    /// The window's theme changed (Windows' app mode, or View › Theme).
    ColorScheme(ColorScheme),
    /// Windows' colors changed (the accent, in Settings › Personalization › Colors).
    ColorsChanged,
    /// Dragging the Document Editor's edge: pressed, moved, released.
    SplitterPressed(PointerEventInfo),
    SplitterMoved(PointerEventInfo),
    SplitterReleased,
    /// A choice in the ribbon chevron menu, by label (it may carry a check mark).
    RibbonLayout(String),
    /// A choice in View › Theme.
    ThemeChosen(Option<usize>),
    /// The status bar zoom slider moved (its position, 0 to 100).
    ZoomSlider(f64),
    /// A percentage picked in the View tab's zoom box.
    ZoomPreset(Option<usize>),
    /// Jump to a heading, at this page position.
    Navigate(i32),
    ExplorerTab(Option<String>),
    Query(String),
    Replacement(String),
    MatchCase(bool),
    WholeWords(bool),
    GoToMatch(usize),
    /// The Spelling pane, by issue: go to it, pick a suggestion, change it, change every
    /// instance of the same misspelling, ignore it here, ignore the word everywhere.
    SpellGoTo(usize),
    SpellChoose(usize, usize),
    SpellChange(usize),
    SpellChangeAll(usize),
    SpellIgnore(usize),
    SpellIgnoreAll(usize),
    /// Apply the chosen (or first) suggestion to every misspelling that has one.
    SpellApplyAll,
    NextMatch,
    PreviousMatch,
    ReplaceOne,
    ReplaceAll,
    /// The empty Headings tab's "Show me how": point to the Styles box.
    ShowMeHow,
    ToggleOutline(OutlineKey),
    /// The gear at the right of the tabs: show the Settings page over the window.
    OpenSettings,
    /// The title bar's back button: leave Settings for the page again.
    CloseSettings,
    /// A category picked down the left of Settings, by tag.
    SettingsCategory(Option<String>),
    /// Settings › Options › Show heading styles in Navigation.
    HeadingStyles(bool),
    /// Settings › Options › Always show the Document Editor.
    DocumentEditor(bool),
    /// Settings › Options › Space below the cursor, by index in `CURSOR_SPACES`.
    CursorSpace(Option<usize>),
    /// Settings › Options › Paragraph font size, by index in `PARAGRAPH_SIZES`.
    ParagraphSize(Option<usize>),
}

/// What a copy of the page's content was asked for.
#[derive(Debug)]
enum Purpose {
    /// Write it to this file, then carry on with what was waiting for the save.
    Save {
        path: PathBuf,
        then: Option<Pending>,
    },
    /// Show it as Markdown.
    ToSource,
    /// Show the Markdown typed in the source view as the visual page.
    ToVisual,
    /// Check its prose for spelling.
    Spelling,
}

/// The app's name, in the title bar and Settings › About.
pub const APP_NAME: &str = "SharePoint Article Manager";
/// The version, from Cargo.toml.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
/// The app's icon: Fluent UI's color "Book Database" (MIT), rendered from assets/app-icon.svg.
/// The 16px design for the title bar, at up to 300% scale.
pub const APP_ICON_SMALL: &[u8] = include_bytes!("../../assets/app-icon-16.png");
/// The 32px design, large, for Settings › About.
pub const APP_ICON_LARGE: &[u8] = include_bytes!("../../assets/app-icon-32.png");
/// The same icon as build.rs embeds in the exe for Explorer.
const APP_ICON_ICO: &[u8] = include_bytes!("../../assets/app.ico");

/// The window's icon file, for the taskbar and Alt+Tab. WinUI's AppWindow takes only a path to
/// an .ico, so the copy built into the exe is written beside the settings (when it differs).
pub fn window_icon_path() -> Option<&'static str> {
    static PATH: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
    PATH.get_or_init(|| {
        let folder = PathBuf::from(std::env::var_os("APPDATA")?).join(APP_NAME);
        let path = folder.join("app.ico");
        if std::fs::read(&path).ok().as_deref() != Some(APP_ICON_ICO) {
            std::fs::create_dir_all(&folder).ok()?;
            std::fs::write(&path, APP_ICON_ICO).ok()?;
        }
        path.into_os_string().into_string().ok()
    })
    .as_deref()
}

/// Sets up what the page needs before the window opens (see `page::prepare_environment`).
pub fn prepare() {
    page::prepare_environment();
}

/// Where the project lives, linked from Settings › About.
pub const PROJECT_URL: &str = "https://github.com/dontthroworanges/SharePointKBArticleManager";
/// The categories down the left of Settings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SettingsCategory {
    Options,
    About,
}

/// Settings' categories: tag, name and category.
pub const SETTINGS_CATEGORIES: [(&str, &str, SettingsCategory); 2] = [
    ("options", "Options", SettingsCategory::Options),
    ("about", "About", SettingsCategory::About),
];

/// The choices made in Settings.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Options {
    /// Show each heading's style, such as (H2), after its name in the Navigation pane.
    pub heading_styles: bool,
    /// Open the Document Editor pane at every start.
    pub document_editor: bool,
    /// Blank page kept below the caret while typing, in percent of the window's height.
    pub cursor_space: u8,
    /// Normal text's size in the visual editor, in points.
    pub paragraph_size: f32,
}

/// Normal text's size in points when Settings › Options › Paragraph font size is not set:
/// 14px (10.5pt), as SharePoint shows it.
pub const DEFAULT_PARAGRAPH_SIZE: f32 = 10.5;

/// The choices for Settings › Options › Paragraph font size: label and size in points. Shown in
/// pixels only, as web pages measure text (a pixel is 3/4 of a point).
pub const PARAGRAPH_SIZES: [(&str, f32); 3] = [("14 px", 10.5), ("16 px", 12.0), ("18 px", 13.5)];

/// The choices for Settings › Options › Space below the cursor: label and percent of the window.
pub const CURSOR_SPACES: [(&str, u8); 4] = [
    ("None", 0),
    ("A quarter of the window", 25),
    ("A third of the window", 33),
    ("Half the window", 50),
];
pub const DEFAULT_CURSOR_SPACE: u8 = 33;

/// Microsoft's guide to creating and editing knowledge base articles, linked from Help.
pub const KB_HELP_URL: &str =
    "https://support.microsoft.com/en-us/sharepoint/knowledge-base/create-edit-knowledge-base-manually";

/// The paragraph styles offered in the Styles box, in the order and with the names of the
/// SharePoint web editor's.
pub const STYLES: [(&str, ParagraphKind); 7] = [
    ("Heading 1", ParagraphKind::Heading(1)),
    ("Heading 2", ParagraphKind::Heading(2)),
    ("Heading 3", ParagraphKind::Heading(3)),
    ("Heading 4", ParagraphKind::Heading(4)),
    ("Heading 5", ParagraphKind::Heading(5)),
    ("Heading 6", ParagraphKind::Heading(6)),
    ("Paragraph", ParagraphKind::Normal),
];

/// Quotes have their own button, as in the web editor.
pub const QUOTE_STYLE: (&str, ParagraphKind) = ("Quote", ParagraphKind::Quote);

pub struct Editor {
    path: Option<PathBuf>,
    properties: Properties,
    /// The WebView2 the page runs in, and the channel to it.
    pub(super) web: ElementRef<WebView2>,
    page: page::Page,
    /// Copies of the page's content asked for and not yet received, by request number.
    requests: HashMap<u64, Purpose>,
    next_request: u64,
    /// The view settings last sent to the page, so they go only when they change.
    sent_settings: Option<Value>,
    selection: SelectionState,
    /// The selected text, for Find and the link dialog.
    selected_text: String,
    mode: Mode,
    /// The Markdown the source view was given: back in the visual view, the page is rebuilt
    /// only if the Markdown no longer matches it.
    source_snapshot: Option<String>,
    tab: Tab,
    dirty: bool,
    show_properties: bool,
    dialog: Option<Dialog>,
    /// The picture or video being edited in the dialog, with all its attributes.
    pending_media: Option<Media>,
    status: String,
    window: WindowRef,
    close_guard: Rc<CloseGuard>,
    window_width: f64,
    window_height: f64,
    /// The choices made in Settings.
    pub(super) options: Options,
    /// The Settings page, when it is showing over the window, and its category.
    pub(super) settings: Option<SettingsCategory>,
    show_navigation: bool,
    /// The Document Editor's width, set by dragging its right edge.
    navigation_width: f64,
    /// While the edge is dragged: where the drag started (window x) and the width then.
    resizing: Option<(f64, f64)>,
    /// The drag handle, to give it the resize cursor.
    pub(super) splitter: ElementRef<Grid>,
    /// The page's headings, for the Navigation pane.
    headings: Vec<HeadingInfo>,
    /// Headings collapsed in the Navigation pane.
    collapsed: HashSet<OutlineKey>,
    /// Where the cursor is, for highlighting the current heading.
    caret: Option<i32>,
    explorer_tab: ExplorerTab,
    query: String,
    replacement: String,
    match_case: bool,
    whole_words: bool,
    matches: Vec<SearchMatch>,
    current_match: Option<usize>,
    /// A result was replaced: go to the next one when the new results come.
    advance_after_replace: bool,
    /// The Windows spell checker, created when the Spelling pane first opens.
    spell_checker: Option<Rc<spelling::Checker>>,
    /// Every misspelling in the page, and those shown in the Spelling pane (without what the
    /// user ignored), in page order.
    spelling_all: Vec<spelling::Issue>,
    spelling: Vec<spelling::Issue>,
    /// The suggestion picked for a misspelling, by its position.
    spelling_choice: HashMap<i32, usize>,
    /// Words the user chose to ignore everywhere, and single misspellings ignored where they
    /// are (by word and which occurrence of that word it is).
    ignored_words: HashSet<String>,
    ignored_once: HashSet<(String, usize)>,
    pub(super) search_box: ElementRef<TextBox>,
    /// Bumped to move focus to the search box once it is on screen.
    focus_search: u64,
    pub(super) root: ElementRef<Grid>,
    root_hooked: bool,
    /// The commands in the ribbon's "More" menu by label, as last rendered.
    pub(super) overflow_commands: std::cell::RefCell<Vec<(String, Command)>>,
    /// Page zoom, 1.0 for 100%.
    zoom: f64,
    /// View › Page Width: the page's width follows the window's, with no upper limit
    /// (the zoom stays at 100%).
    page_fit: bool,
    /// Wheel movement with Ctrl held not yet turned into a zoom step (a touchpad sends a notch
    /// in pieces).
    wheel_zoom: i32,
    /// The theme picked in View › Theme.
    theme: theme::ThemeChoice,
    /// Single-line or Classic ribbon.
    ribbon_layout: RibbonLayout,
    /// The theme the window shows (Windows' app mode when following the system).
    scheme: ColorScheme,
    /// The Windows accent color, for the title band.
    accent: Color,
    /// The palette the caption buttons were last colored for, once that has succeeded.
    captions: Rc<std::cell::Cell<Option<theme::Palette>>>,
    /// View › Switch Background: the page is dark.
    dark_page: bool,
    /// The Styles box, and the native box last given its styled list (by address), so each
    /// new box is hooked once.
    pub(super) styles_box: ElementRef<ComboBox>,
    styles_box_hooked: Rc<std::cell::Cell<usize>>,
    /// Something that went wrong, shown above the page until it is closed. Routine messages
    /// (saved, opened) are not shown, as in the web editor.
    problem: Option<String>,
    /// `KB_SELF_TEST`: open, round-trip through the Markdown view, save, and quit.
    self_test: Option<SelfTest>,
}

/// Identifies a heading in the Navigation pane across edits: its level, text, and which
/// occurrence of that heading it is.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct OutlineKey {
    pub level: u8,
    pub text: String,
    pub occurrence: usize,
}

/// A change of page zoom, in Word's steps of 10%.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Zoom {
    In,
    Out,
    /// A percentage.
    To(u16),
    /// Word's Page Width: the page as wide as the window leaves room for, at 100%.
    PageWidth,
}

/// The wheel movement of one notch of a mouse wheel; a touchpad sends smaller amounts.
const WHEEL_NOTCH: i32 = 120;

pub const MIN_ZOOM: f64 = 0.1;
pub const MAX_ZOOM: f64 = 5.0;
/// The presets in the View tab's zoom box, as percentages.
pub const ZOOM_PRESETS: [u16; 8] = [50, 75, 100, 125, 150, 200, 300, 500];
/// The View tab's Page Width button, beside the zoom box: see [`Zoom::PageWidth`].
pub const PAGE_WIDTH: &str = "Page Width";

/// The tabs of the Document Editor pane, as in Word's Navigation pane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExplorerTab {
    Headings,
    Find,
    Replace,
    /// Every misspelling in the page, with suggestions.
    Spelling,
}

/// One search result, at page positions (in the Markdown view, offsets in its text).
#[derive(Clone, Debug, PartialEq)]
pub struct SearchMatch {
    pub start: i32,
    pub end: i32,
    pub before: String,
    pub text: String,
    pub after: String,
}

/// One row of the Navigation pane.
#[derive(Clone, Debug)]
pub struct OutlineEntry {
    pub heading: HeadingInfo,
    pub key: OutlineKey,
    pub has_children: bool,
    pub current: bool,
}

/// Shared with the window close guard, which runs outside the component.
#[derive(Default)]
struct CloseGuard {
    dirty: std::cell::Cell<bool>,
    allow: std::cell::Cell<bool>,
    /// The window (HWND), once found, so its place can be remembered when it closes.
    window: std::cell::Cell<isize>,
    settings: std::cell::RefCell<settings::Settings>,
}

impl CloseGuard {
    /// Saves where the window is, for the next session.
    fn remember_window(&self) {
        let window = self.window.get();
        if window == 0 {
            return;
        }
        if let Some(placement) = settings::placement(window) {
            let mut settings = self.settings.borrow_mut();
            settings.placement = Some(placement);
            settings.save();
        }
    }
}

/// The steps of `KB_SELF_TEST`.
#[derive(Clone, Debug)]
struct SelfTest {
    output: PathBuf,
    step: u8,
}

impl Component for Editor {
    type Input = Option<PathBuf>;
    type Message = Msg;

    fn create(path: &Option<PathBuf>, context: &ComponentContext<Self>) -> Self {
        let saved = settings::Settings::load();
        let paragraph_size = saved
            .paragraph_size
            .filter(|size| PARAGRAPH_SIZES.iter().any(|&(_, s)| s == *size))
            .unwrap_or(DEFAULT_PARAGRAPH_SIZE);
        let close_guard = Rc::new(CloseGuard {
            settings: std::cell::RefCell::new(saved.clone()),
            ..CloseGuard::default()
        });
        let guard = Rc::clone(&close_guard);
        let sender = context.sender();
        windows_reactor::set_window_close_guard(move |args| {
            if !guard.dirty.get() || guard.allow.get() {
                guard.remember_window();
                return false;
            }
            let Ok(args) = windows_core::Interface::cast::<crate::winui::IWindowEventArgs>(args)
            else {
                return false;
            };
            if args.SetHandled(true).is_err() {
                return false;
            }
            sender.send(Msg::CloseRequested);
            true
        });
        let mut editor = Self {
            path: None,
            properties: Properties::None,
            web: ElementRef::new(),
            page: page::Page::default(),
            requests: HashMap::new(),
            next_request: 1,
            sent_settings: None,
            selection: SelectionState::default(),
            selected_text: String::new(),
            mode: Mode::Visual,
            source_snapshot: None,
            tab: Tab::Home,
            dirty: false,
            show_properties: false,
            dialog: None,
            pending_media: None,
            status: "Ready".into(),
            window: context.window(),
            close_guard,
            window_width: 1280.0,
            window_height: 860.0,
            options: Options {
                heading_styles: saved.heading_styles,
                document_editor: saved.document_editor,
                cursor_space: saved
                    .cursor_space
                    .filter(|space| CURSOR_SPACES.iter().any(|&(_, s)| s == *space))
                    .unwrap_or(DEFAULT_CURSOR_SPACE),
                paragraph_size,
            },
            settings: None,
            // Settings › Options › Always show the Document Editor.
            show_navigation: saved.document_editor,
            navigation_width: saved.explorer_width.unwrap_or(340.0),
            resizing: None,
            splitter: ElementRef::new(),
            headings: Vec::new(),
            collapsed: HashSet::new(),
            caret: None,
            explorer_tab: ExplorerTab::Headings,
            query: String::new(),
            replacement: String::new(),
            match_case: false,
            whole_words: false,
            matches: Vec::new(),
            current_match: None,
            advance_after_replace: false,
            spell_checker: None,
            spelling_all: Vec::new(),
            spelling: Vec::new(),
            spelling_choice: HashMap::new(),
            ignored_words: HashSet::new(),
            ignored_once: HashSet::new(),
            search_box: ElementRef::new(),
            focus_search: 0,
            root: ElementRef::new(),
            root_hooked: false,
            overflow_commands: Default::default(),
            zoom: 1.0,
            page_fit: false,
            wheel_zoom: 0,
            theme: theme::THEMES
                .iter()
                .find(|(name, _)| saved.theme.as_deref() == Some(*name))
                .map_or_else(theme::ThemeChoice::default, |&(_, choice)| choice),
            ribbon_layout: RIBBON_LAYOUTS
                .iter()
                .find(|(name, _)| saved.ribbon.as_deref() == Some(*name))
                .map_or_else(RibbonLayout::default, |&(_, layout)| layout),
            dark_page: saved.dark_page,
            styles_box: ElementRef::new(),
            styles_box_hooked: Rc::default(),
            problem: None,
            scheme: ColorScheme::Light,
            accent: theme::system_accent(),
            captions: Rc::default(),
            self_test: std::env::var_os("KB_SELF_TEST").map(|output| SelfTest {
                output: PathBuf::from(output),
                step: 0,
            }),
        };
        match path {
            Some(path) => editor.open_path(path.clone()),
            None => editor.load(Document::default()),
        }
        // The accent can change while the app runs; the title band and highlights follow it.
        let sender = context.sender();
        if let Err(error) = theme::watch_colors(move || {
            sender.send(Msg::ColorsChanged);
        }) {
            crate::log::log!("cannot follow accent color changes: {error:?}");
        }
        editor.sync_page();
        editor
    }

    fn update(&mut self, message: Msg, context: &ComponentContext<Self>) {
        crate::log::log!("{message:?}");
        let started = std::time::Instant::now();
        self.update_inner(message, context);
        crate::log::log!(
            "  update took {:.2} ms",
            started.elapsed().as_secs_f64() * 1000.0
        );
    }

    fn view(&self, _: &Option<PathBuf>, context: &mut ViewContext<Self>) -> View {
        self.render(context)
    }
}

impl Editor {
    fn update_inner(&mut self, message: Msg, context: &ComponentContext<Self>) {
        self.color_caption_buttons();
        self.find_window();
        self.hook_styles_box();
        match message {
            Msg::PageInit => self.init_page(context),
            Msg::PageCore(core) => {
                let sender = context.sender();
                if let Err(error) = self
                    .page
                    .attach(core, move |json| {
                        sender.send(Msg::Page(json));
                    })
                {
                    self.report(format!("The page could not be set up: {error}"));
                }
            }
            Msg::Page(json) => self.from_page(&json, context),
            Msg::Failed(error) => {
                crate::log::log!("failed: {error}");
                self.report(error);
            }
            Msg::AppShortcut(shortcut @ (Shortcut::FindNext | Shortcut::FindPrevious))
                if !self.matches.is_empty() =>
            {
                self.step_match(shortcut == Shortcut::FindNext);
            }
            Msg::AppShortcut(shortcut) => self.run(shortcut.command()),
            Msg::Command(command) => self.run(command),
            Msg::MoreMenu(label) => {
                let command = self
                    .overflow_commands
                    .borrow()
                    .iter()
                    .find(|(name, _)| *name == label)
                    .map(|&(_, command)| command);
                if let Some(command) = command {
                    self.run(command);
                }
            }
            // Reactor reports a menu click by the item's label.
            Msg::StyleChosen(Some(index)) => {
                if let Some(&(_, kind)) = STYLES.get(index)
                    && kind != self.selection.kind
                {
                    self.run(Command::Kind(kind));
                }
            }
            Msg::StyleChosen(None) => {}
            Msg::Tab(Some(text)) => {
                self.tab = match text.as_str() {
                    "File" => Tab::File,
                    "Insert" => Tab::Insert,
                    "View" => Tab::View,
                    "Help" => Tab::Help,
                    _ => Tab::Home,
                };
            }
            Msg::Tab(None) => {}
            Msg::PropertyKey(index, key) => {
                if let Properties::Entries(entries) = &mut self.properties
                    && let Some(entry) = entries.get_mut(index).filter(|entry| entry.key != key)
                {
                    entry.key = key;
                    self.dirty = true;
                }
            }
            Msg::PropertyValue(index, value) => {
                if let Properties::Entries(entries) = &mut self.properties
                    && let Some(entry) = entries.get_mut(index).filter(|entry| entry.value != value)
                {
                    entry.value = value;
                    self.dirty = true;
                }
            }
            Msg::AddProperty => {
                match &mut self.properties {
                    Properties::Entries(entries) => entries.push(Property::default()),
                    Properties::None => {
                        self.properties = Properties::Entries(vec![Property::default()]);
                    }
                    Properties::Raw(_) => {}
                }
                self.show_properties = true;
            }
            Msg::RemoveProperty(index) => {
                if let Properties::Entries(entries) = &mut self.properties {
                    if index < entries.len() {
                        entries.remove(index);
                        self.dirty = true;
                    }
                    if entries.is_empty() {
                        self.properties = Properties::None;
                    }
                }
            }
            Msg::RawProperties(text) => {
                if self.properties != Properties::Raw(text.clone()) {
                    self.properties = Properties::Raw(text);
                    self.dirty = true;
                }
            }
            Msg::DialogField(index, text) => {
                if let Some(field) = self.dialog.as_mut().and_then(|d| d.fields.get_mut(index)) {
                    *field = text;
                }
            }
            Msg::DialogClosed(result) => self.close_dialog(result),
            Msg::CloseRequested => self.confirm_discard(Pending::Close),
            Msg::DismissProblem => self.problem = None,
            Msg::WindowSize(size) => {
                self.window_width = size.width;
                self.window_height = size.height;
            }
            Msg::OpenSettings => self.settings = Some(SettingsCategory::Options),
            Msg::CloseSettings => {
                self.settings = None;
                self.focus_page();
            }
            Msg::SettingsCategory(tag) => {
                if let Some(&(_, _, category)) = SETTINGS_CATEGORIES
                    .iter()
                    .find(|(name, ..)| Some(*name) == tag.as_deref())
                {
                    self.settings = Some(category);
                }
            }
            Msg::HeadingStyles(on) => self.options.heading_styles = on,
            Msg::DocumentEditor(on) => {
                self.options.document_editor = on;
                // Shown straight away, so the pane is there on returning to the page.
                if on && !self.show_navigation {
                    self.run(Command::ToggleNavigation);
                }
            }
            Msg::CursorSpace(index) => {
                if let Some(&(_, space)) = index.and_then(|index| CURSOR_SPACES.get(index)) {
                    self.options.cursor_space = space;
                }
            }
            Msg::ParagraphSize(index) => {
                if let Some(&(_, size)) = index.and_then(|index| PARAGRAPH_SIZES.get(index)) {
                    self.options.paragraph_size = size;
                }
            }
            Msg::ColorsChanged => self.accent = theme::system_accent(),
            Msg::ColorScheme(scheme) => {
                self.scheme = scheme;
                // The accent may have changed with the theme in Settings; read it again.
                self.accent = theme::system_accent();
            }
            Msg::ThemeChosen(Some(index)) => {
                if let Some(&(_, choice)) = theme::THEMES.get(index) {
                    self.run(Command::Theme(choice));
                }
            }
            Msg::ThemeChosen(None) => {}
            Msg::SplitterPressed(pointer) => {
                self.resizing = Some((pointer.window_x, self.navigation_width));
            }
            Msg::SplitterMoved(pointer) => {
                if let Some((start, width)) = self.resizing {
                    self.navigation_width =
                        view::navigation_width(width + pointer.window_x - start, self.window_width);
                }
            }
            Msg::SplitterReleased => self.resizing = None,
            Msg::RibbonLayout(label) => {
                if let Some(&(_, layout)) = RIBBON_LAYOUTS
                    .iter()
                    .find(|(name, _)| label.ends_with(name))
                {
                    self.ribbon_layout = layout;
                }
            }
            Msg::ZoomSlider(position) => {
                let zoom = slider_to_zoom(position);
                // Ignore the slider echoing a zoom set elsewhere.
                if (zoom - self.zoom).abs() > 0.005 {
                    self.zoom = zoom;
                    self.page_fit = false;
                }
            }
            Msg::ZoomPreset(Some(index)) => {
                if let Some(&percent) = ZOOM_PRESETS.get(index) {
                    self.run(Command::Zoom(Zoom::To(percent)));
                }
            }
            Msg::ZoomPreset(None) => {}
            Msg::Navigate(position) => {
                // A heading goes to the top of the window, as in Word, whatever is below it.
                self.select(position, position, 0.0);
            }
            Msg::ExplorerTab(Some(text)) => {
                self.explorer_tab = match text.as_str() {
                    "Find" => ExplorerTab::Find,
                    "Replace" => ExplorerTab::Replace,
                    "Spelling" => ExplorerTab::Spelling,
                    _ => ExplorerTab::Headings,
                };
                match self.explorer_tab {
                    ExplorerTab::Find | ExplorerTab::Replace => self.focus_search += 1,
                    ExplorerTab::Spelling => self.check_spelling(),
                    ExplorerTab::Headings => {}
                }
            }
            Msg::ExplorerTab(None) => {}
            Msg::Query(text) => {
                if text != self.query {
                    self.query = text;
                    if !self.query.is_empty() && self.explorer_tab == ExplorerTab::Headings {
                        self.explorer_tab = ExplorerTab::Find;
                    }
                    self.current_match = None;
                    self.run_search();
                }
            }
            Msg::Replacement(text) => self.replacement = text,
            Msg::MatchCase(on) => {
                self.match_case = on;
                self.run_search();
            }
            Msg::WholeWords(on) => {
                self.whole_words = on;
                self.run_search();
            }
            Msg::SpellGoTo(index) => {
                if let Some(issue) = self.spelling.get(index).cloned() {
                    self.select(issue.start, issue.end, 0.33);
                }
            }
            Msg::SpellChoose(index, choice) => {
                if let Some(issue) = self.spelling.get(index) {
                    self.spelling_choice.insert(issue.start, choice);
                }
            }
            Msg::SpellChange(index) => {
                let changes: Vec<_> = self.spelling.get(index).into_iter().cloned().collect();
                self.correct(&changes);
            }
            Msg::SpellChangeAll(index) => {
                // Every instance of this misspelling, and no other, takes the same correction.
                if let Some(issue) = self.spelling.get(index).cloned()
                    && let Some(replacement) = self.chosen_suggestion(&issue)
                {
                    let matching: Vec<_> = self
                        .spelling
                        .iter()
                        .filter(|other| other.word == issue.word)
                        .map(|other| spelling::Issue {
                            suggestions: vec![replacement.clone()],
                            ..other.clone()
                        })
                        .collect();
                    self.spelling_choice.clear();
                    self.correct(&matching);
                }
            }
            Msg::SpellIgnore(index) => {
                if let Some(key) = self.occurrence_key(index) {
                    self.ignored_once.insert(key);
                    self.filter_spelling();
                }
            }
            Msg::SpellIgnoreAll(index) => {
                if let Some(issue) = self.spelling.get(index) {
                    self.ignored_words.insert(issue.word.clone());
                    self.filter_spelling();
                }
            }
            Msg::SpellApplyAll => {
                let all = self.spelling.clone();
                self.correct(&all);
            }
            Msg::GoToMatch(index) => {
                self.current_match = Some(index);
                self.go_to_match();
            }
            Msg::NextMatch => self.step_match(true),
            Msg::PreviousMatch => self.step_match(false),
            Msg::ReplaceOne => self.replace_current(),
            Msg::ReplaceAll => self.replace_all(),
            Msg::ShowMeHow => {
                self.tab = Tab::Home;
                self.status =
                    "Click in a line, then choose a heading in the Styles box on the Home tab."
                        .into();
            }
            Msg::ToggleOutline(key) => {
                if !self.collapsed.remove(&key) {
                    self.collapsed.insert(key);
                }
            }
        }
        self.sync_page();
        self.close_guard.dirty.set(self.dirty);
        self.remember_choices();
    }

    /// Hooks the window's keys and starts the page, once its WebView2 is on screen.
    fn init_page(&mut self, context: &ComponentContext<Self>) {
        if !self.root_hooked {
            let sender = context.sender();
            self.root_hooked = self.root.request_native_object(move |result| {
                let hooked = result
                    .map_err(|error| format!("{error:?}"))
                    .and_then(|object| {
                        keys::hook_root(object, sender.clone()).map_err(|e| e.to_string())
                    });
                if let Err(error) = hooked {
                    crate::log::log!("window shortcuts are unavailable: {error}");
                }
            });
        }
        if self.page.requested {
            return;
        }
        // The control paints white behind the page unless told otherwise; clear, the canvas
        // behind it (which follows the theme) shows around the page.
        _ = self.web.request_native_object(|result| {
            let cleared = result
                .map_err(|error| format!("{error:?}"))
                .and_then(|object| {
                    windows_core::Interface::cast::<crate::winui::IWebView2>(&object)
                        .and_then(|web| {
                            web.SetDefaultBackgroundColor(crate::winui::Color {
                                a: 0,
                                r: 0,
                                g: 0,
                                b: 0,
                            })
                        })
                        .map_err(|error| error.to_string())
                });
            if let Err(error) = cleared {
                crate::log::log!("could not clear the page's background: {error}");
            }
        });
        let sender = context.sender();
        self.page.requested = self.web.request_core_web_view2(move |result| {
            sender.send(match result {
                Ok(object) => match windows_core::Interface::cast(&object) {
                    Ok(core) => Msg::PageCore(core),
                    Err(error) => Msg::Failed(format!("The page could not start: {error}")),
                },
                Err(error) => Msg::Failed(format!(
                    "The page could not start. Is the Microsoft Edge WebView2 Runtime installed? ({error:?})"
                )),
            });
        });
    }

    /// Sends the page the view settings when they change: zoom, page width, colors, text size
    /// and the room kept below the cursor.
    fn sync_page(&mut self) {
        let settings = json!({
            "type": "settings",
            "zoom": self.zoom,
            "fit": self.page_fit,
            "darkPage": self.dark_page,
            // Points to pixels.
            "bodyPx": f64::from(self.options.paragraph_size) * 4.0 / 3.0,
            "cursorSpace": self.options.cursor_space,
            // The app's theme, which may differ from Windows': the page's scroll bars and
            // controls follow it.
            "windowDark": self.scheme == ColorScheme::Dark,
            // Windows' accent, for outlines on the page.
            "accent": format!("#{:02x}{:02x}{:02x}", self.accent.r, self.accent.g, self.accent.b),
        });
        if self.sent_settings.as_ref() != Some(&settings) {
            self.sent_settings = Some(settings.clone());
            self.page.post(settings);
        }
    }

    /// Gives the page keyboard focus, and the cursor in the page focus there.
    fn focus_page(&mut self) {
        _ = self.web.request_native_object(|result| {
            if let Ok(object) = result
                && let Ok(element) = windows_core::Interface::cast::<crate::winui::IUIElement>(&object)
            {
                _ = element.Focus(crate::winui::FocusState::Programmatic);
            }
        });
        self.page.post(json!({ "type": "focus" }));
    }

    /// Asks the page for a copy of its content: its document (`doc`), the Markdown typed in
    /// the source view (`markdown`) or its prose for spelling (`blocks`).
    fn request(&mut self, purpose: Purpose, what: &str) {
        let id = self.next_request;
        self.next_request += 1;
        self.requests.insert(id, purpose);
        self.page
            .post(json!({ "type": "get", "id": id, "what": what }));
    }

    /// Sends the page an editing command (see `run` in `web/editor.js`).
    fn edit(&mut self, mut command: Value) {
        command["type"] = json!("command");
        self.page.post(command);
        self.focus_page();
    }

    /// Selects a range in the page and scrolls it `ratio` of the way down the window.
    fn select(&mut self, from: i32, to: i32, ratio: f64) {
        self.page
            .post(json!({ "type": "select", "from": from, "to": to, "ratio": ratio }));
        self.focus_page();
    }

    /// Handles a message from the page.
    fn from_page(&mut self, json: &str, context: &ComponentContext<Self>) {
        let Ok(message) = serde_json::from_str::<Value>(json) else {
            crate::log::log!("unreadable message from the page: {json}");
            return;
        };
        match message["type"].as_str().unwrap_or_default() {
            "ready" => {
                self.page.set_ready();
                if self.self_test.is_some() {
                    self.advance_self_test();
                }
            }
            "changed" => self.dirty = true,
            "state" => self.read_state(&message),
            "idle" => {
                if let Some(outline) = message["outline"].as_array() {
                    self.headings = outline
                        .iter()
                        .filter_map(|heading| {
                            Some(HeadingInfo {
                                level: heading["level"].as_u64()? as u8,
                                text: heading["text"].as_str()?.to_string(),
                                // The cursor goes at the start of the heading's text.
                                position: heading["pos"].as_i64()? as i32 + 1,
                            })
                        })
                        .collect();
                } else if let Some(text) = message["source"].as_str() {
                    self.headings = source_headings(text);
                }
                if self.show_navigation
                    && self.explorer_tab == ExplorerTab::Spelling
                    && self.mode == Mode::Visual
                {
                    self.check_spelling();
                }
            }
            "matches" => self.read_matches(&message),
            "reply" => self.reply(&message),
            "shortcut" => {
                if let Some(shortcut) = message["name"].as_str().and_then(keys::page_shortcut) {
                    self.update_inner(Msg::AppShortcut(shortcut), context);
                }
            }
            "wheel" => self.wheel(message["delta"].as_f64().unwrap_or_default() as i32),
            "openUrl" => {
                if let Some(href) = message["href"].as_str() {
                    open_url(href);
                }
            }
            "editMedia" => {
                let attributes = Media {
                    attributes: message["attributes"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .filter_map(|pair| {
                            Some((pair[0].as_str()?.to_string(), pair[1].as_str()?.to_string()))
                        })
                        .collect(),
                };
                if let Some(pos) = message["pos"].as_i64() {
                    self.dialog = Some(Dialog {
                        kind: DialogKind::EditMedia(pos as i32),
                        fields: vec![
                            attributes.source().to_string(),
                            attributes.alt_text().to_string(),
                        ],
                    });
                    self.pending_media = Some(attributes);
                }
            }
            "log" => crate::log::log!("page: {}", message["text"].as_str().unwrap_or_default()),
            other => crate::log::log!("unknown message from the page: {other}"),
        }
    }

    /// The formatting where the cursor is, for the ribbon.
    fn read_state(&mut self, message: &Value) {
        let flag = |name: &str| message[name].as_bool().unwrap_or(false);
        let kind = match message["kind"].as_str() {
            Some("heading") => {
                ParagraphKind::Heading(message["level"].as_u64().unwrap_or(1).clamp(1, 6) as u8)
            }
            Some("quote") => ParagraphKind::Quote,
            _ => ParagraphKind::Normal,
        };
        let list = match message["list"].as_str() {
            Some("bullet") => Some(ListKind::Bullet),
            Some("numbered") => Some(ListKind::Numbered),
            Some("task") => Some(ListKind::Task {
                checked: flag("checked"),
            }),
            _ => None,
        };
        self.selection = SelectionState {
            bold: flag("bold"),
            italic: flag("italic"),
            strikethrough: flag("strike"),
            highlight: flag("highlight"),
            code: flag("code"),
            kind,
            list,
            has_selection: flag("hasSelection"),
        };
        self.selected_text = message["selectedText"].as_str().unwrap_or_default().to_string();
        self.caret = message["caret"].as_i64().map(|caret| caret as i32);
    }

    /// A copy of the page's content, asked for with `request`.
    fn reply(&mut self, message: &Value) {
        let Some(purpose) = message["id"].as_u64().and_then(|id| self.requests.remove(&id))
        else {
            return;
        };
        match purpose {
            Purpose::Save { path, then } => {
                let markdown = match message["markdown"].as_str() {
                    Some(markdown) => markdown.to_string(),
                    None => self.markdown_of(&message["doc"]),
                };
                if self.write(&path, &markdown)
                    && let Some(pending) = then
                {
                    self.continue_pending(pending);
                }
            }
            Purpose::ToSource => {
                let markdown = self.markdown_of(&message["doc"]);
                self.headings = source_headings(&markdown);
                self.source_snapshot = Some(markdown.clone());
                self.mode = Mode::Source;
                self.selection = SelectionState::default();
                self.page
                    .post(json!({ "type": "source", "markdown": markdown }));
                self.focus_page();
                self.advance_self_test();
            }
            Purpose::ToVisual => {
                let markdown = message["markdown"].as_str().unwrap_or_default().to_string();
                let unchanged = self
                    .source_snapshot
                    .as_deref()
                    .is_some_and(|snapshot| same_markdown(snapshot, &markdown));
                self.mode = Mode::Visual;
                self.source_snapshot = None;
                if unchanged {
                    // The visual page is still as it was: show it again, cursor, undo and all.
                    self.page.post(json!({ "type": "visual" }));
                } else {
                    let document = parse(&markdown);
                    self.properties = document.properties.clone();
                    self.page.post(json!({
                        "type": "load",
                        "doc": bridge::to_pm(&document),
                    }));
                }
                self.focus_page();
                self.run_search();
                self.advance_self_test();
            }
            Purpose::Spelling => {
                let Some(checker) = self.spell_checker.clone() else {
                    return;
                };
                match spelling::check_blocks(&checker, &message["blocks"]) {
                    Ok(issues) => self.spelling_all = issues,
                    Err(error) => {
                        crate::log::log!("checking spelling failed: {error:?}");
                        self.spelling_all.clear();
                    }
                }
                self.filter_spelling();
                if self.self_test.as_ref().is_some_and(|test| test.step == 4) {
                    self.advance_self_test();
                }
            }
        }
    }

    /// The Markdown for a document from the page, with the page's properties.
    fn markdown_of(&self, doc: &Value) -> String {
        serialize(&Document {
            properties: self.properties.clone(),
            blocks: bridge::from_pm(doc),
        })
    }

    /// The colors for the current theme and accent.
    pub(super) fn palette(&self) -> theme::Palette {
        theme::Palette::new(self.scheme == ColorScheme::Dark, self.accent)
    }

    /// Saves the View tab's choices when they change, for the next session.
    fn remember_choices(&self) {
        // Not on every step of dragging the Document Editor's edge.
        if self.resizing.is_some() {
            return;
        }
        let theme = theme::THEMES
            .iter()
            .find(|(_, choice)| *choice == self.theme)
            .map(|(name, _)| name.to_string());
        let ribbon = RIBBON_LAYOUTS
            .iter()
            .find(|(_, layout)| *layout == self.ribbon_layout)
            .map(|(name, _)| name.to_string());
        let mut settings = self.close_guard.settings.borrow_mut();
        let next = settings::Settings {
            placement: settings.placement,
            theme,
            ribbon,
            dark_page: self.dark_page,
            explorer_width: Some(self.navigation_width),
            heading_styles: self.options.heading_styles,
            document_editor: self.options.document_editor,
            cursor_space: Some(self.options.cursor_space),
            paragraph_size: Some(self.options.paragraph_size),
        };
        if *settings != next {
            *settings = next;
            settings.save();
        }
    }

    /// Gives the Styles box its styled list (see the free function of the same name).
    fn hook_styles_box(&self) {
        hook_styles_box(&self.styles_box, &self.styles_box_hooked);
    }

    /// Finds the window once it is on screen, and moves it back to where it was last session.
    fn find_window(&self) {
        if self.close_guard.window.get() != 0 {
            return;
        }
        let guard = Rc::clone(&self.close_guard);
        _ = self.root.request_native_object(move |result| {
            if guard.window.get() != 0 {
                return;
            }
            if let Ok(object) = result
                && let Ok(window) = theme::window_handle(&object)
                && window != 0
            {
                guard.window.set(window);
                let saved = guard.settings.borrow().placement;
                if let Some(placement) = saved {
                    settings::restore(window, placement);
                }
            }
        });
    }

    /// Colors the system's minimize, maximize and close buttons for the title band. This runs on
    /// every message until it works (the window has no XAML root at first), and again when the
    /// theme or accent changes.
    fn color_caption_buttons(&self) {
        let palette = self.palette();
        if self.captions.get() == Some(palette) {
            return;
        }
        let done = Rc::clone(&self.captions);
        _ = self.root.request_native_object(move |result| {
            if let Ok(object) = result
                && theme::color_caption_buttons(&object, palette).is_ok()
            {
                done.set(Some(palette));
            }
        });
    }

    /// Replaces the page with `document`.
    fn load(&mut self, document: Document) {
        self.properties = document.properties.clone();
        self.mode = Mode::Visual;
        self.source_snapshot = None;
        self.selection = SelectionState::default();
        self.selected_text.clear();
        self.caret = None;
        self.headings.clear();
        self.matches.clear();
        self.current_match = None;
        self.spelling_all.clear();
        self.spelling.clear();
        self.spelling_choice.clear();
        self.ignored_once.clear();
        self.dirty = false;
        self.page.post(json!({
            "type": "load",
            "doc": bridge::to_pm(&document),
        }));
        self.run_search();
    }

    fn run(&mut self, command: Command) {
        match command {
            Command::New => self.confirm_discard(Pending::New),
            Command::Open => self.confirm_discard(Pending::Open),
            Command::Save => self.save(false, None),
            Command::About => self.settings = Some(SettingsCategory::About),
            Command::OpenUrl(url) => open_url(url),
            Command::SaveAs => self.save(true, None),
            Command::Rename => {
                // Only a page saved to a file has a name to change.
                if let Some(path) = &self.path {
                    let name = path
                        .file_stem()
                        .map(|stem| stem.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    self.dialog = Some(Dialog {
                        kind: DialogKind::Rename,
                        fields: vec![name],
                    });
                }
            }
            Command::Mode(mode) => self.set_mode(mode),
            Command::Theme(choice) => self.theme = choice,
            Command::SwitchBackground => self.dark_page = !self.dark_page,
            Command::ToggleProperties => self.show_properties = !self.show_properties,
            Command::Zoom(change) => {
                // Word steps to the next multiple of 10%.
                let percent = (self.zoom * 100.0).round();
                let target = match change {
                    Zoom::In => ((percent / 10.0).floor() + 1.0) * 10.0,
                    Zoom::Out => ((percent / 10.0).ceil() - 1.0) * 10.0,
                    Zoom::To(percent) => f64::from(percent),
                    Zoom::PageWidth => 100.0,
                };
                // Any other zoom leaves Page Width, and the page goes back to its usual width.
                self.page_fit = change == Zoom::PageWidth;
                self.zoom = (target / 100.0).clamp(MIN_ZOOM, MAX_ZOOM);
            }
            Command::Search(tab) => {
                if !self.show_navigation {
                    self.run(Command::ToggleNavigation);
                }
                self.explorer_tab = tab;
                // Like Word, search for the selected text.
                let selected = self.selected_text.clone();
                if !selected.is_empty() && !selected.contains(['\r', '\n']) {
                    self.query = selected;
                    self.current_match = None;
                }
                self.run_search();
                self.focus_search += 1;
            }
            Command::Explorer(tab) => {
                if self.show_navigation && self.explorer_tab == tab {
                    self.show_navigation = false;
                } else {
                    if !self.show_navigation {
                        self.run(Command::ToggleNavigation);
                    }
                    self.explorer_tab = tab;
                    match tab {
                        ExplorerTab::Find | ExplorerTab::Replace => self.focus_search += 1,
                        ExplorerTab::Spelling => self.check_spelling(),
                        ExplorerTab::Headings => {}
                    }
                }
            }
            Command::ToggleNavigation => self.show_navigation = !self.show_navigation,
            Command::Link => self.open_dialog(DialogKind::Link),
            Command::PageLink => self.open_dialog(DialogKind::PageLink),
            Command::Insert(Insert::Image) => self.open_dialog(DialogKind::Image),
            Command::Insert(Insert::Video) => self.open_dialog(DialogKind::Video),
            Command::Insert(insert) => {
                let block = match insert {
                    Insert::Table => Block::Table(Table::new(3, 2)),
                    Insert::Code => Block::Code {
                        language: String::new(),
                        code: String::new(),
                    },
                    Insert::Mermaid => Block::Code {
                        language: "mermaid".into(),
                        code: "graph TD\n  A[Start] --> B[Finish]".into(),
                    },
                    Insert::Equation => Block::Math("x^2 + y^2 = z^2".into()),
                    _ => Block::Rule,
                };
                self.insert_block(block);
            }
            // Editing commands, for the visual page.
            command => {
                if self.mode != Mode::Visual {
                    return;
                }
                let message = match command {
                    Command::Undo => json!({ "name": "undo" }),
                    Command::Redo => json!({ "name": "redo" }),
                    Command::Inline(inline) => json!({ "name": "inline", "mark": inline.mark() }),
                    Command::Kind(kind) => match kind {
                        ParagraphKind::Normal => json!({ "name": "kind", "kind": "normal" }),
                        ParagraphKind::Heading(level) => {
                            json!({ "name": "kind", "kind": "heading", "level": level })
                        }
                        ParagraphKind::Quote => json!({ "name": "kind", "kind": "quote" }),
                    },
                    Command::List(kind) => json!({
                        "name": "list",
                        "list": match kind {
                            ListKind::Bullet => "bullet",
                            ListKind::Numbered => "numbered",
                            ListKind::Task { .. } => "task",
                        },
                    }),
                    Command::ToggleDone => json!({ "name": "toggleDone" }),
                    Command::Indent => json!({ "name": "indent" }),
                    Command::Outdent => json!({ "name": "outdent" }),
                    _ => return,
                };
                self.edit(message);
            }
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        if mode == self.mode {
            return;
        }
        match mode {
            Mode::Source => self.request(Purpose::ToSource, "doc"),
            Mode::Visual => self.request(Purpose::ToVisual, "markdown"),
        }
    }

    /// Inserts `block` at the cursor in the visual page.
    fn insert_block(&mut self, block: Block) {
        if self.mode != Mode::Visual {
            return;
        }
        let doc = bridge::to_pm(&Document {
            properties: Properties::None,
            blocks: vec![block],
        });
        let node = doc["content"][0].clone();
        self.edit(json!({ "name": "insert", "node": node }));
    }

    fn open_dialog(&mut self, kind: DialogKind) {
        if self.mode != Mode::Visual {
            return;
        }
        let selected = self.selected_text.clone();
        let fields = match kind {
            DialogKind::Link => vec![String::new(), selected],
            DialogKind::PageLink => vec![selected.clone(), String::new()],
            DialogKind::Image | DialogKind::Video => vec![String::new(), String::new()],
            DialogKind::EditMedia(_) | DialogKind::Discard(_) | DialogKind::Rename => Vec::new(),
        };
        self.dialog = Some(Dialog { kind, fields });
    }

    fn close_dialog(&mut self, result: ContentDialogResult) {
        let Some(dialog) = self.dialog.take() else {
            return;
        };
        let field = |index: usize| {
            dialog
                .fields
                .get(index)
                .map(|f| f.trim().to_string())
                .unwrap_or_default()
        };
        match dialog.kind {
            DialogKind::Discard(pending) => match result {
                ContentDialogResult::Primary => self.save(false, Some(pending)),
                ContentDialogResult::Secondary => self.continue_pending(pending),
                _ => {}
            },
            _ if result != ContentDialogResult::Primary => self.focus_page(),
            DialogKind::Link => {
                if !field(0).is_empty() {
                    self.edit(json!({ "name": "link", "href": field(0), "text": field(1) }));
                } else {
                    self.focus_page();
                }
            }
            DialogKind::PageLink => {
                if !field(0).is_empty() {
                    self.edit(json!({ "name": "pageLink", "page": field(0), "text": field(1) }));
                } else {
                    self.focus_page();
                }
            }
            DialogKind::Image | DialogKind::Video => {
                if !field(0).is_empty() {
                    self.insert_block(Block::Media(Media::new(&field(0), &field(1))));
                }
            }
            DialogKind::EditMedia(pos) => {
                let mut media = self.pending_media.take().unwrap_or_default();
                media.set("source", &field(0));
                media.set("alt-text", &field(1));
                let attributes: Vec<Value> = media
                    .attributes
                    .iter()
                    .map(|(key, value)| json!([key, value]))
                    .collect();
                self.edit(json!({
                    "name": "setAttrs",
                    "pos": pos,
                    "attrs": { "attributes": attributes },
                }));
            }
            DialogKind::Rename => {
                self.rename(&field(0));
                self.focus_page();
            }
        }
    }

    /// Renames the open file in its folder. Without an extension, the name keeps the file's own.
    fn rename(&mut self, name: &str) {
        let Some(path) = self.path.clone() else {
            return;
        };
        let target = match renamed_path(&path, name) {
            Ok(Some(target)) => target,
            Ok(None) => return,
            Err(problem) => {
                self.report(problem);
                return;
            }
        };
        // Windows names ignore case, so a change of case alone is the same file.
        let same_file =
            target.to_string_lossy().to_lowercase() == path.to_string_lossy().to_lowercase();
        if target.exists() && !same_file {
            self.report(format!(
                "There is already a file called {} in this folder.",
                file_name(&target)
            ));
            return;
        }
        match std::fs::rename(&path, &target) {
            Ok(()) => {
                self.status = format!("Renamed to {}", file_name(&target));
                self.path = Some(target);
            }
            Err(error) => self.report(format!("The file could not be renamed: {error}")),
        }
    }

    fn confirm_discard(&mut self, pending: Pending) {
        if self.dirty {
            self.dialog = Some(Dialog {
                kind: DialogKind::Discard(pending),
                fields: Vec::new(),
            });
        } else {
            self.continue_pending(pending);
        }
    }

    fn continue_pending(&mut self, pending: Pending) {
        match pending {
            Pending::Close => {
                self.close_guard.allow.set(true);
                _ = self.window.request_close();
            }
            Pending::New => {
                self.path = None;
                self.load(Document::default());
                self.status = "New page".into();
            }
            Pending::Open => {
                let picked = file_dialog()
                    .set_title("Open Knowledge Base page")
                    .add_filter("Markdown", &["md", "markdown"])
                    .add_filter("All files", &["*"])
                    .pick_file();
                if let Some(path) = picked {
                    self.open_path(path);
                }
            }
        }
    }

    fn open_path(&mut self, path: PathBuf) {
        match std::fs::read_to_string(&path) {
            Ok(text) => {
                let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
                self.load(parse(text));
                self.status = format!("Opened {}", file_name(&path));
                self.path = Some(path);
            }
            Err(error) => self.report(format!("Could not open {}: {error}", path.display())),
        }
    }

    /// Saves the page, choosing a file first for Save As or a page never saved. The page's
    /// content is asked for and written when it comes (see `reply`); `then` runs after that.
    fn save(&mut self, choose_path: bool, then: Option<Pending>) {
        let path = match (&self.path, choose_path) {
            (Some(path), false) => path.clone(),
            _ => {
                let name = self.path.as_ref().and_then(|p| p.file_name()).map_or_else(
                    || suggested_name(&self.headings),
                    |n| n.to_string_lossy().into_owned(),
                );
                let picked = file_dialog()
                    .set_title("Save Knowledge Base page")
                    .set_file_name(name)
                    .add_filter("Markdown", &["md"])
                    .save_file();
                match picked {
                    Some(path) => path,
                    None => return,
                }
            }
        };
        let what = if self.mode == Mode::Source { "markdown" } else { "doc" };
        self.request(Purpose::Save { path, then }, what);
    }

    /// Writes the page's Markdown to `path`; whether that worked.
    fn write(&mut self, path: &std::path::Path, markdown: &str) -> bool {
        match std::fs::write(path, markdown) {
            Ok(()) => {
                self.dirty = false;
                self.status = format!("Saved {} at {}", file_name(path), now());
                self.problem = None;
                self.path = Some(path.to_path_buf());
                self.page.post(json!({ "type": "saved" }));
                if self.self_test.is_some() {
                    crate::log::log!("self-test: saved {}", path.display());
                    std::process::exit(0);
                }
                true
            }
            Err(error) => {
                self.report(format!("Could not save {}: {error}", path.display()));
                false
            }
        }
    }

    /// `KB_SELF_TEST`: once the page is ready, switch to the Markdown view and back, search for
    /// `KB_SELF_TEST_QUERY` (if set) and make the first result bold, check spelling, then save
    /// to the test's file (which quits). Each step logs what it found.
    fn advance_self_test(&mut self) {
        let Some(test) = self.self_test.as_mut() else {
            return;
        };
        test.step += 1;
        let (step, output) = (test.step, test.output.clone());
        crate::log::log!("self-test: step {step}");
        let query = std::env::var("KB_SELF_TEST_QUERY").unwrap_or_default();
        match step {
            1 => self.set_mode(Mode::Source),
            2 => self.set_mode(Mode::Visual),
            // The results come as a `matches` message, which moves the test on.
            3 if !query.is_empty() => {
                self.query = query;
                self.run_search();
            }
            3 | 4 => {
                if step == 4
                    && let Some(found) = self.matches.first().cloned()
                {
                    crate::log::log!(
                        "self-test: {} results; first {:?} at {}–{}",
                        self.matches.len(),
                        found.text,
                        found.start,
                        found.end
                    );
                    self.select(found.start, found.end, 0.33);
                    self.edit(json!({ "name": "inline", "mark": "bold" }));
                }
                if let Some(test) = self.self_test.as_mut() {
                    test.step = 4;
                }
                // The issues come as a reply, which moves the test on.
                self.check_spelling();
            }
            _ => {
                crate::log::log!("self-test: {} spelling issues", self.spelling.len());
                for issue in self.spelling.iter().take(5) {
                    crate::log::log!(
                        "self-test: misspelled {:?} at {}–{}, suggestions {:?}",
                        issue.word,
                        issue.start,
                        issue.end,
                        issue.suggestions
                    );
                }
                // KB_SELF_TEST_FIX: apply every first suggestion before saving.
                if std::env::var_os("KB_SELF_TEST_FIX").is_some() {
                    let all = self.spelling.clone();
                    self.correct(&all);
                }
                let what = if self.mode == Mode::Source { "markdown" } else { "doc" };
                self.request(Purpose::Save { path: output, then: None }, what);
            }
        }
    }

    /// Shows `message` above the page, for something that went wrong or needs the user.
    fn report(&mut self, message: String) {
        self.status = message.clone();
        self.problem = Some(message);
    }

    /// The window's title, as the taskbar shows it: the document, then the app.
    pub fn title(&self) -> String {
        format!("{} - {APP_NAME}", self.document_title())
    }

    /// The open document's name, marked when it has unsaved changes.
    pub fn document_title(&self) -> String {
        let name = self.path.as_ref().and_then(|p| p.file_name()).map_or_else(
            || "Untitled".to_string(),
            |n| n.to_string_lossy().into_owned(),
        );
        format!("{name}{}", if self.dirty { " (Unsaved)" } else { "" })
    }

    /// Ctrl+wheel over the page: a zoom step for each whole notch.
    fn wheel(&mut self, delta: i32) {
        // A reversed direction drops what was gathered.
        if delta.signum() != self.wheel_zoom.signum() {
            self.wheel_zoom = 0;
        }
        self.wheel_zoom += delta;
        while self.wheel_zoom.abs() >= WHEEL_NOTCH {
            let up = self.wheel_zoom > 0;
            self.wheel_zoom -= WHEEL_NOTCH * self.wheel_zoom.signum();
            self.run(Command::Zoom(if up { Zoom::In } else { Zoom::Out }));
        }
    }
}

/// A file name from the page's first heading, as SharePoint page names usually match it.
fn suggested_name(headings: &[HeadingInfo]) -> String {
    let name: String = headings
        .first()
        .map_or("Untitled", |heading| heading.text.as_str())
        .chars()
        .filter(|c| !r#"\/:*?"<>|"#.contains(*c))
        .collect();
    let name = name.trim();
    format!("{}.md", if name.is_empty() { "Untitled" } else { name })
}

fn file_name(path: &std::path::Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// The local time as HH:MM, for the status bar.
fn now() -> String {
    #[repr(C)]
    struct SystemTime {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        milliseconds: u16,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetLocalTime(time: *mut SystemTime);
    }
    let mut time = std::mem::MaybeUninit::<SystemTime>::uninit();
    // SAFETY: GetLocalTime fully initializes the structure it is given.
    let time = unsafe {
        GetLocalTime(time.as_mut_ptr());
        time.assume_init()
    };
    format!("{:02}:{:02}", time.hour, time.minute)
}

/// The path a file would have under the name typed in the Rename dialog, in the same folder.
/// None when the name is empty or unchanged; an error for a name Windows does not allow.
fn renamed_path(path: &std::path::Path, name: &str) -> Result<Option<PathBuf>, String> {
    let name = name.trim();
    if name.is_empty() {
        return Ok(None);
    }
    if let Some(bad) = name.chars().find(|c| r#"<>:"/\|?*"#.contains(*c) || c.is_control()) {
        return Err(format!("A file name can't contain {bad}."));
    }
    if name.ends_with('.') {
        return Err("A file name can't end with a full stop.".into());
    }
    // Without an extension, the file keeps its own (.md).
    let mut file = PathBuf::from(name);
    if file.extension().is_none()
        && let Some(extension) = path.extension()
    {
        file.set_extension(extension);
    }
    let target = path.with_file_name(file);
    Ok((target != path).then_some(target))
}

/// Whether two Markdown texts are the same page, ignoring how lines end and blank lines at the
/// end.
fn same_markdown(a: &str, b: &str) -> bool {
    let normal = |text: &str| text.replace("\r\n", "\n").replace('\r', "\n");
    normal(a).trim_end_matches('\n') == normal(b).trim_end_matches('\n')
}

/// The headings of the Markdown view's text, each at the offset (in UTF-16 units, as the page
/// counts) its line starts at.
fn source_headings(text: &str) -> Vec<HeadingInfo> {
    let mut starts = vec![0];
    let mut position = 0;
    for character in text.chars() {
        position += character.len_utf16() as i32;
        if character == '\n' {
            starts.push(position);
        }
    }
    crate::markdown::headings(text)
        .into_iter()
        .filter_map(|heading| {
            Some(HeadingInfo {
                level: heading.level,
                text: heading.text,
                position: *starts.get(heading.line)?,
            })
        })
        .collect()
}

/// Opens a web address in the default browser. Only web and mail addresses: a page could
/// otherwise run a program.
pub(crate) fn open_url(url: &str) {
    let lower = url.to_ascii_lowercase();
    if !["https://", "http://", "mailto:"]
        .iter()
        .any(|scheme| lower.starts_with(scheme))
    {
        crate::log::log!("not opening {url}: not a web address");
        return;
    }
    #[link(name = "shell32")]
    unsafe extern "system" {
        fn ShellExecuteW(
            hwnd: *mut core::ffi::c_void,
            operation: *const u16,
            file: *const u16,
            parameters: *const u16,
            directory: *const u16,
            show: i32,
        ) -> *mut core::ffi::c_void;
    }
    const SW_SHOWNORMAL: i32 = 1;
    let wide = |s: &str| s.encode_utf16().chain([0]).collect::<Vec<u16>>();
    let (operation, file) = (wide("open"), wide(url));
    // SAFETY: both strings are null-terminated and outlive the call; the rest may be null.
    let result = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            operation.as_ptr(),
            file.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };
    // ShellExecute returns a value above 32 on success.
    if result as usize <= 32 {
        crate::log::log!("could not open {url}: ShellExecute returned {}", result as usize);
    }
}

impl Editor {
    /// The document's headings in page order, for the Navigation pane.
    pub fn outline(&self) -> Vec<OutlineEntry> {
        let mut occurrences: HashMap<(u8, String), usize> = HashMap::new();
        let mut entries: Vec<OutlineEntry> = Vec::new();
        let mut current = None;
        for heading in &self.headings {
            let count = occurrences
                .entry((heading.level, heading.text.clone()))
                .or_default();
            let key = OutlineKey {
                level: heading.level,
                text: heading.text.clone(),
                occurrence: *count,
            };
            *count += 1;
            if self.caret.is_some_and(|caret| heading.position <= caret) {
                current = Some(entries.len());
            }
            entries.push(OutlineEntry {
                heading: heading.clone(),
                key,
                has_children: false,
                current: false,
            });
        }
        for index in 0..entries.len() {
            let level = entries[index].heading.level;
            entries[index].has_children = entries
                .get(index + 1)
                .is_some_and(|next| next.heading.level > level);
        }
        if let Some(current) = current {
            entries[current].current = true;
        }
        entries
    }
}

impl Editor {
    /// Asks the page for its prose and lists every misspelling in it for the Spelling pane,
    /// leaving out what the user ignored. Only the visual view is checked: Markdown syntax is
    /// not prose.
    fn check_spelling(&mut self) {
        if self.mode != Mode::Visual {
            self.spelling_all.clear();
            self.spelling.clear();
            return;
        }
        if self.spell_checker.is_none() {
            match spelling::Checker::new() {
                Ok(checker) => self.spell_checker = Some(Rc::new(checker)),
                Err(error) => {
                    crate::log::log!("the spell checker is unavailable: {error:?}");
                    self.report("Windows spell checking is not available.".into());
                    return;
                }
            }
        }
        self.request(Purpose::Spelling, "blocks");
    }

    /// The misspellings to show: all of them, less what the user ignored.
    fn filter_spelling(&mut self) {
        let mut seen: HashMap<String, usize> = HashMap::new();
        let ignored_once = &self.ignored_once;
        let ignored_words = &self.ignored_words;
        self.spelling = self
            .spelling_all
            .iter()
            .filter(|issue| {
                let count = seen.entry(issue.word.clone()).or_default();
                let key = (issue.word.clone(), *count);
                *count += 1;
                !ignored_words.contains(&issue.word) && !ignored_once.contains(&key)
            })
            .cloned()
            .collect();
        let issues = &self.spelling;
        self.spelling_choice
            .retain(|start, _| issues.iter().any(|issue| issue.start == *start));
    }

    /// Identifies a misspelling across edits: its word, and which occurrence of that word in
    /// the page it is (counting ignored ones too).
    fn occurrence_key(&self, index: usize) -> Option<(String, usize)> {
        let issue = self.spelling.get(index)?;
        let ordinal = self
            .spelling_all
            .iter()
            .take_while(|other| other.start != issue.start)
            .filter(|other| other.word == issue.word)
            .count();
        Some((issue.word.clone(), ordinal))
    }

    /// The suggestion picked for a misspelling, or its first one.
    fn chosen_suggestion(&self, issue: &spelling::Issue) -> Option<String> {
        let choice = self.spelling_choice.get(&issue.start).copied().unwrap_or(0);
        issue.suggestions.get(choice).cloned()
    }

    /// Replaces each misspelling with its chosen suggestion (one undo step), then checks the
    /// page again. Misspellings without a suggestion are left alone.
    fn correct(&mut self, issues: &[spelling::Issue]) {
        let changes: Vec<Value> = issues
            .iter()
            .filter_map(|issue| {
                let replacement = self.chosen_suggestion(issue)?;
                Some(json!([issue.start, issue.end, replacement]))
            })
            .collect();
        let changed = changes.len();
        if changed == 0 {
            return;
        }
        self.page
            .post(json!({ "type": "replace", "changes": changes }));
        self.status = if changed == 1 {
            "Corrected 1 misspelling.".into()
        } else {
            format!("Corrected {changed} misspellings.")
        };
        self.check_spelling();
    }

    /// Searches the page for the query; the results come back as a `matches` message.
    fn run_search(&mut self) {
        if self.query.is_empty() {
            self.matches.clear();
            self.current_match = None;
        }
        self.page.post(json!({
            "type": "find",
            "query": self.query,
            "matchCase": self.match_case,
            "wholeWords": self.whole_words,
        }));
    }

    /// New search results, after a search or an edit. Keeps the current result where it was
    /// when possible: the first result at or after it.
    fn read_matches(&mut self, message: &Value) {
        if message["query"].as_str() != Some(self.query.as_str()) {
            return;
        }
        let previous = self
            .current_match
            .and_then(|index| self.matches.get(index))
            .map(|found| found.start);
        self.matches = message["matches"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|found| {
                Some(SearchMatch {
                    start: found["from"].as_i64()? as i32,
                    end: found["to"].as_i64()? as i32,
                    before: found["before"].as_str()?.to_string(),
                    text: found["text"].as_str()?.to_string(),
                    after: found["after"].as_str()?.to_string(),
                })
            })
            .collect();
        self.current_match =
            previous.and_then(|start| self.matches.iter().position(|found| found.start >= start));
        if std::mem::take(&mut self.advance_after_replace) && !self.matches.is_empty() {
            self.current_match = Some(self.current_match.unwrap_or(0));
            self.go_to_match();
        }
        if !self.query.is_empty() && self.self_test.as_ref().is_some_and(|test| test.step == 3) {
            self.advance_self_test();
        }
    }

    fn step_match(&mut self, forward: bool) {
        let count = self.matches.len();
        if count == 0 {
            return;
        }
        self.current_match = Some(match self.current_match {
            None if forward => 0,
            None => count - 1,
            Some(index) if forward => (index + 1) % count,
            Some(index) => (index + count - 1) % count,
        });
        self.go_to_match();
    }

    /// Selects the current result in the page and scrolls it a third of the way down.
    fn go_to_match(&mut self) {
        let Some(found) = self
            .current_match
            .and_then(|index| self.matches.get(index).cloned())
        else {
            return;
        };
        self.select(found.start, found.end, 0.33);
    }

    /// Replaces the current result (or goes to the first one), then moves to the next, as
    /// Word's Replace button does.
    fn replace_current(&mut self) {
        let Some(found) = self
            .current_match
            .and_then(|index| self.matches.get(index).cloned())
        else {
            self.step_match(true);
            return;
        };
        self.page.post(json!({
            "type": "replace",
            "changes": [[found.start, found.end, self.replacement]],
        }));
        self.advance_after_replace = true;
    }

    fn replace_all(&mut self) {
        let changes: Vec<Value> = self
            .matches
            .iter()
            .map(|found| json!([found.start, found.end, self.replacement]))
            .collect();
        let replaced = changes.len();
        if replaced > 0 {
            self.page
                .post(json!({ "type": "replace", "changes": changes }));
        }
        self.current_match = None;
        self.status = match replaced {
            0 => "No results to replace.".into(),
            1 => "Replaced 1 result.".into(),
            count => format!("Replaced {count} results."),
        };
    }
}

/// A file dialog owned by the editor window, so it is modal like Word's Open and Save As.
fn file_dialog() -> rfd::FileDialog {
    let dialog = rfd::FileDialog::new();
    match Owner::active() {
        Some(owner) => dialog.set_parent(&owner),
        None => dialog,
    }
}

/// The app's active window, as a parent for file dialogs.
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

/// The zoom slider is not linear, as in Word: its middle is 100%, the left half covers
/// 10–100% and the right half 100–500%.
pub fn slider_to_zoom(position: f64) -> f64 {
    let position = position.clamp(0.0, 100.0);
    let zoom = if position <= 50.0 {
        MIN_ZOOM + (1.0 - MIN_ZOOM) * position / 50.0
    } else {
        1.0 + (MAX_ZOOM - 1.0) * (position - 50.0) / 50.0
    };
    (zoom * 100.0).round() / 100.0
}

pub fn zoom_to_slider(zoom: f64) -> f64 {
    let zoom = zoom.clamp(MIN_ZOOM, MAX_ZOOM);
    if zoom <= 1.0 {
        (zoom - MIN_ZOOM) / (1.0 - MIN_ZOOM) * 50.0
    } else {
        50.0 + (zoom - 1.0) / (MAX_ZOOM - 1.0) * 50.0
    }
}

/// Gives the Styles box its styled list, once per native box (the ribbon rebuilds the box when
/// its layout changes). Runs after each render and each message, as the box may only now exist.
fn hook_styles_box(styles_box: &ElementRef<ComboBox>, hooked: &Rc<std::cell::Cell<usize>>) {
    let hooked = Rc::clone(hooked);
    _ = styles_box.request_native_object(move |result| {
        let Ok(object) = result else { return };
        let address = windows_core::Interface::as_raw(&object) as usize;
        if hooked.get() != address && styles_box::hook(&object).is_ok() {
            hooked.set(address);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_renamed_file_keeps_its_folder_and_extension() {
        let path = std::path::Path::new(r"C:\KB\old name.md");
        let renamed = |name| renamed_path(path, name);
        assert_eq!(
            renamed("New name"),
            Ok(Some(PathBuf::from(r"C:\KB\New name.md")))
        );
        assert_eq!(
            renamed("notes.txt"),
            Ok(Some(PathBuf::from(r"C:\KB\notes.txt")))
        );
        assert_eq!(renamed("  "), Ok(None));
        assert_eq!(renamed("old name"), Ok(None));
        assert!(renamed("a/b").is_err());
        assert!(renamed("what?").is_err());
        assert!(renamed("trailing.").is_err());
    }

    #[test]
    fn markdown_read_back_from_the_editor_matches_what_it_was_given() {
        assert!(same_markdown("# A\n\nText\n", "# A\r\rText"));
        assert!(same_markdown("a\r\nb", "a\nb\n\n"));
        assert!(!same_markdown("# A\n\nText", "# A\n\nText!"));
    }

    #[test]
    fn slider_has_100_percent_in_the_middle() {
        assert_eq!(slider_to_zoom(0.0), MIN_ZOOM);
        assert_eq!(slider_to_zoom(50.0), 1.0);
        assert_eq!(slider_to_zoom(100.0), MAX_ZOOM);
        for percent in [10, 25, 50, 100, 120, 300, 500] {
            let zoom = f64::from(percent) / 100.0;
            assert!(
                (slider_to_zoom(zoom_to_slider(zoom)) - zoom).abs() < 0.011,
                "{percent}%"
            );
        }
    }

    #[test]
    fn source_headings_are_at_their_lines() {
        let headings = source_headings("# One\n\ntext\n\n## Two\n");
        assert_eq!(headings.len(), 2);
        assert_eq!((headings[0].level, headings[0].position), (1, 0));
        assert_eq!((headings[1].level, headings[1].position), (2, 13));
    }

    #[test]
    fn a_new_page_is_named_after_its_first_heading() {
        let heading = |text: &str| HeadingInfo {
            level: 1,
            text: text.into(),
            position: 1,
        };
        assert_eq!(suggested_name(&[heading("Reset: a password?")]), "Reset a password.md");
        assert_eq!(suggested_name(&[]), "Untitled.md");
        assert_eq!(suggested_name(&[heading("  ")]), "Untitled.md");
    }
}
