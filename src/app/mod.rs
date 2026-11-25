//! The editor window: document state, commands, and file handling.

mod context_menu;
mod host;
mod spelling;
mod styles_box;
mod settings;
mod theme;
mod view;

use crate::markdown::*;
use crate::rich::{self, InlineCommand, SelectionState};
use host::{Content, Host, HostKind};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;
use windows_reactor::*;

pub type Id = u64;

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

/// Keyboard shortcuts handled by the app rather than the RichEditBox.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shortcut {
    Save,
    SaveAs,
    Open,
    New,
    Normal,
    Link,
    List(ListKind),
    Highlight,
    Strikethrough,
    Code,
    Heading(u8),
    Indent,
    Outdent,
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
            Self::Normal => Command::Kind(ParagraphKind::Normal),
            Self::Link => Command::Link,
            Self::List(kind) => Command::List(kind),
            Self::Highlight => Command::Inline(InlineCommand::Highlight),
            Self::Strikethrough => Command::Inline(InlineCommand::Strikethrough),
            Self::Code => Command::Inline(InlineCommand::Code),
            Self::Heading(level) => Command::Kind(ParagraphKind::Heading(level)),
            Self::Indent => Command::Indent,
            Self::Outdent => Command::Outdent,
            // Without a search to step through, F3 opens Find, as in Word.
            Self::Find | Self::FindNext | Self::FindPrevious => Command::Search(ExplorerTab::Find),
            Self::Replace => Command::Search(ExplorerTab::Replace),
            Self::ZoomIn => Command::Zoom(Zoom::In),
            Self::ZoomOut => Command::Zoom(Zoom::Out),
            Self::ZoomReset => Command::Zoom(Zoom::To(100)),
        }
    }
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TableEdit {
    AddRow,
    AddColumn,
    RemoveRow,
    RemoveColumn,
}

/// One piece of the page: a run of text flow, or a block object between flows.
#[derive(Clone, Debug)]
pub enum Segment {
    /// Paragraphs edited in one RichEditBox. `initial` is only used until it is attached.
    Flow {
        id: Id,
        initial: Vec<Paragraph>,
    },
    Code {
        id: Id,
        language: String,
        initial: String,
    },
    Math {
        id: Id,
        initial: String,
    },
    Raw {
        id: Id,
        initial: String,
    },
    Table {
        id: Id,
        table: TableState,
    },
    Media {
        id: Id,
        media: Media,
    },
    Rule {
        id: Id,
    },
}

impl Segment {
    pub fn id(&self) -> Id {
        match self {
            Self::Flow { id, .. }
            | Self::Code { id, .. }
            | Self::Math { id, .. }
            | Self::Raw { id, .. }
            | Self::Table { id, .. }
            | Self::Media { id, .. }
            | Self::Rule { id } => *id,
        }
    }

    /// The editor content for segments shown in a RichEditBox.
    fn content(&self) -> Option<Content> {
        match self {
            Self::Flow { initial, .. } => Some(Content::Flow(initial.clone())),
            Self::Code { initial, .. } | Self::Math { initial, .. } | Self::Raw { initial, .. } => {
                Some(Content::Plain(initial.clone()))
            }
            _ => None,
        }
    }
}

/// Table cells are edited as inline Markdown text; row 0 is the header.
#[derive(Clone, Debug, PartialEq)]
pub struct TableState {
    pub alignments: Vec<Alignment>,
    pub cells: Vec<Vec<String>>,
}

impl TableState {
    fn from_table(table: &Table) -> Self {
        let cells = std::iter::once(&table.header)
            .chain(&table.rows)
            .map(|row| {
                row.iter()
                    .map(|cell| inlines_to_markdown(cell, true))
                    .collect()
            })
            .collect();
        Self {
            alignments: table.alignments.clone(),
            cells,
        }
    }

    fn to_table(&self) -> Table {
        let mut rows = self.cells.iter().map(|row| {
            row.iter()
                .map(|cell| parse_inlines(cell))
                .collect::<Vec<_>>()
        });
        Table {
            alignments: self.alignments.clone(),
            header: rows.next().unwrap_or_default(),
            rows: rows.collect(),
        }
    }

    fn columns(&self) -> usize {
        self.cells.first().map_or(0, Vec::len)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DialogKind {
    Link,
    PageLink,
    Image,
    Video,
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
    Attach,
    Styled(Id),
    Attached(Id, Rc<Host>),
    Failed(String),
    Selection(Id),
    Changed(Id),
    /// A formatting edit made with a keyboard shortcut, which does not change the text.
    Edited(Id),
    /// A text flow's editor was laid out at a new size.
    Resized(Id),
    /// Typing has paused (see `host::after_typing`): time for the whole-document work
    /// deferred while keys were coming in.
    Idle,
    Pasted(Id),
    Shortcut(Id, Shortcut),
    /// A window-wide shortcut, from wherever focus is.
    AppShortcut(Shortcut),
    Command(Command),
    /// A click in the ribbon's "More" menu, by label.
    MoreMenu(String),
    StyleChosen(Option<usize>),
    Tab(Option<String>),
    CodeLanguage(Id, String),
    Cell(Id, usize, usize, String),
    Table(Id, TableEdit),
    MediaSource(Id, String),
    MediaAlt(Id, String),
    Delete(Id),
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
    /// A preset picked in the View tab's zoom box (after the percentages, Page Width).
    ZoomPreset(Option<usize>),
    /// The mouse wheel turned over the page with Ctrl held (120 a notch; up is positive).
    WheelZoom(i32),
    /// Jump to a heading: the editor and the position in it.
    Navigate(Id, i32),
    /// A command chosen in an editor's right-click menu: a style, or a Font or Paragraph
    /// command.
    ContextCommand(Id, Command),
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
    /// Scroll an editor position to the top of the page.
    /// Scroll an editor position into view, at this fraction of the window height from the top.
    ScrollTo(Id, i32, f64),
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

/// Quotes have their own button, as in the web editor, and are in the right-click menu.
pub const QUOTE_STYLE: (&str, ParagraphKind) = ("Quote", ParagraphKind::Quote);

pub struct Editor {
    path: Option<PathBuf>,
    properties: Properties,
    segments: Vec<Segment>,
    hosts: HashMap<Id, Rc<Host>>,
    editors: HashMap<Id, ElementRef<RichEditBox>>,
    wrappers: HashMap<Id, ElementRef<Grid>>,
    styled: HashSet<Id>,
    in_flight: HashSet<Id>,
    /// Editors typed in since the last `Msg::Idle`, whose heading rules, headings, search
    /// results and spelling are brought up to date once typing pauses.
    typed_in: HashSet<Id>,
    /// Content for text editors that are not attached yet.
    pending_content: HashMap<Id, Content>,
    generation: u64,
    next_id: Id,
    active: Option<Id>,
    selection: SelectionState,
    mode: Mode,
    source_id: Option<Id>,
    /// The Markdown the source editor was last given from the visual page: back in the visual
    /// view, the page is rebuilt only if the Markdown no longer matches it.
    source_snapshot: Option<String>,
    /// The visual editor that had the caret when the Markdown view opened.
    visual_active: Option<Id>,
    tab: Tab,
    dirty: bool,
    show_properties: bool,
    dialog: Option<Dialog>,
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
    /// Headings of each attached text flow, for the Navigation pane.
    headings: HashMap<Id, Vec<rich::HeadingInfo>>,
    /// Each editor's length when its `headings` were last put right (see `follow_edit`).
    heading_lengths: HashMap<Id, i32>,
    /// Where each text flow's Heading 1 and 2 rules go (see `rich::heading_rules`).
    rules: HashMap<Id, Vec<f32>>,
    /// Headings collapsed in the Navigation pane.
    collapsed: HashSet<OutlineKey>,
    /// The caret: which editor and where, for highlighting the current heading.
    caret: Option<(Id, i32)>,
    explorer_tab: ExplorerTab,
    query: String,
    replacement: String,
    match_case: bool,
    whole_words: bool,
    matches: Vec<SearchMatch>,
    current_match: Option<usize>,
    /// The Windows spell checker, created when the Spelling pane first opens.
    spell_checker: Option<Rc<spelling::Checker>>,
    /// The misspellings shown in the Spelling pane, in page order.
    spelling: Vec<spelling::Issue>,
    /// The suggestion picked for a misspelling, by its editor and position.
    spelling_choice: HashMap<(Id, i32), usize>,
    /// Words the user chose to ignore everywhere, and single misspellings ignored where they
    /// are (by editor, word and which occurrence of that word it is).
    ignored_words: HashSet<String>,
    ignored_once: HashSet<(Id, String, usize)>,
    pub(super) search_box: ElementRef<TextBox>,
    /// Bumped to move focus to the search box once it is on screen.
    focus_search: u64,
    pub(super) root: ElementRef<Grid>,
    root_hooked: bool,
    /// The commands in the ribbon's "More" menu by label, as last rendered.
    pub(super) overflow_commands: std::cell::RefCell<Vec<(String, Command)>>,
    /// Page zoom, 1.0 for 100%.
    zoom: f64,
    /// View › Zoom › Page Width: the page's width follows the window's, with no upper limit
    /// (the zoom stays at 100%).
    page_fit: bool,
    /// Wheel movement with Ctrl held not yet turned into a zoom step (a touchpad sends a notch
    /// in pieces).
    wheel_zoom: i32,
    /// The page Grid that has the page handlers (its address), and the handlers (see
    /// `host::hook_page`).
    page_hooks: Rc<std::cell::RefCell<Option<(usize, Vec<windows_core::EventRevoker>)>>>,
    /// A Navigation pane heading was chosen and no key pressed since: focusing the editor
    /// must not scroll the heading away from the top (see `host::hook_page`).
    navigating: Rc<std::cell::Cell<bool>>,
    /// The text flows' hosts, for showing links' addresses on hover (see `host::hook_pointer`).
    flow_hosts: Rc<std::cell::RefCell<Vec<std::rc::Weak<Host>>>>,
    /// Whether the visual page is showing (not the Markdown view or Settings), for the same.
    visual_page: Rc<std::cell::Cell<bool>>,
    /// Blank page added at the end once a heading has been chosen in the Navigation pane, so
    /// even the last heading can scroll to the top of the window. Kept until another page
    /// loads, so the page does not shrink under the reader.
    reach_room: bool,
    /// The theme picked in View › Theme.
    theme: theme::ThemeChoice,
    /// Single-line or Classic ribbon.
    ribbon_layout: RibbonLayout,
    /// The theme the window shows (Windows' app mode when following the system).
    scheme: ColorScheme,
    /// The Windows accent color, for the title band.
    accent: Color,
    /// The page, kept in the light theme.
    pub(super) page: ElementRef<Grid>,
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
/// The zoom box's last choice, after the presets: see [`Zoom::PageWidth`].
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

/// One search result.
#[derive(Clone, Debug, PartialEq)]
pub struct SearchMatch {
    pub editor: Id,
    pub start: i32,
    pub end: i32,
    pub before: String,
    pub text: String,
    pub after: String,
}

/// One row of the Navigation pane.
#[derive(Clone, Debug)]
pub struct OutlineEntry {
    pub editor: Id,
    pub heading: rich::HeadingInfo,
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

impl Component for Editor {
    type Input = Option<PathBuf>;
    type Message = Msg;

    fn create(path: &Option<PathBuf>, context: &ComponentContext<Self>) -> Self {
        let saved = settings::Settings::load();
        rich::set_dark_page(saved.dark_page);
        let paragraph_size = saved
            .paragraph_size
            .filter(|size| PARAGRAPH_SIZES.iter().any(|&(_, s)| s == *size))
            .unwrap_or(rich::DEFAULT_BODY_SIZE);
        rich::set_body_size(paragraph_size);
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
            segments: Vec::new(),
            hosts: HashMap::new(),
            editors: HashMap::new(),
            wrappers: HashMap::new(),
            styled: HashSet::new(),
            in_flight: HashSet::new(),
            typed_in: HashSet::new(),
            pending_content: HashMap::new(),
            generation: 0,
            next_id: 1,
            active: None,
            selection: SelectionState::default(),
            mode: Mode::Visual,
            source_id: None,
            source_snapshot: None,
            visual_active: None,
            tab: Tab::Home,
            dirty: false,
            show_properties: false,
            dialog: None,
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
            headings: HashMap::new(),
            heading_lengths: HashMap::new(),
            rules: HashMap::new(),
            collapsed: HashSet::new(),
            caret: None,
            explorer_tab: ExplorerTab::Headings,
            query: String::new(),
            replacement: String::new(),
            match_case: false,
            whole_words: false,
            matches: Vec::new(),
            current_match: None,
            spell_checker: None,
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
            page_hooks: Rc::default(),
            navigating: Rc::default(),
            reach_room: false,
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
            page: ElementRef::new(),
            captions: Rc::default(),
            flow_hosts: Rc::default(),
            visual_page: Rc::default(),
        };
        match path {
            Some(path) => editor.open_path(path.clone()),
            None => editor.load(Document::default()),
        }
        // A link's address on hover, and the hand pointer (see `host::watch_pointer`).
        let guard = Rc::clone(&editor.close_guard);
        host::watch_pointer(
            Box::new(move || guard.window.get()),
            Rc::clone(&editor.flow_hosts),
            Rc::clone(&editor.visual_page),
        );
        editor
    }

    fn update(&mut self, message: Msg, context: &ComponentContext<Self>) {
        crate::log::log!("{message:?} active={:?}", self.active);
        let started = std::time::Instant::now();
        self.update_inner(message, context);
        crate::log::log!(
            "  update took {:.2} ms",
            started.elapsed().as_secs_f64() * 1000.0
        );
    }

    fn view(&self, _: &Option<PathBuf>, context: &mut ViewContext<Self>) -> View {
        let started = std::time::Instant::now();
        let view = self.render(context);
        crate::log::log!(
            "  view took {:.2} ms",
            started.elapsed().as_secs_f64() * 1000.0
        );
        view
    }
}

impl Editor {
    fn update_inner(&mut self, message: Msg, context: &ComponentContext<Self>) {
        let typed = matches!(message, Msg::Changed(_));
        self.color_caption_buttons();
        self.find_window();
        self.hook_styles_box();
        match message {
            Msg::Attach => self.attach_pending(context),
            Msg::Styled(id) => {
                self.in_flight.remove(&id);
                self.styled.insert(id);
                self.generation += 1;
            }
            Msg::Attached(id, host) => {
                self.in_flight.remove(&id);
                self.pending_content.remove(&id);
                let focus = self.active.is_none() && host.kind == HostKind::Flow;
                if host.kind == HostKind::Flow {
                    self.flow_hosts.borrow_mut().push(Rc::downgrade(&host));
                }
                self.hosts.insert(id, host);
                self.refresh_headings(id);
                self.refresh_rules(id);
                if focus || self.active == Some(id) {
                    self.active = Some(id);
                    if let Some(editor) = self.editors.get(&id) {
                        _ = editor.request_focus();
                    }
                } else if self.mode == Mode::Source && self.source_id == Some(id) {
                    // The caret moves into the Markdown, out of the hidden visual page.
                    if let Some(editor) = self.editors.get(&id) {
                        _ = editor.request_focus();
                    }
                }
            }
            Msg::Failed(error) => {
                crate::log::log!("failed: {error}");
                self.report(error);
            }
            Msg::Selection(id) => {
                // The first selection event of an editor comes from loading, not the user.
                let loading = self
                    .hosts
                    .get(&id)
                    .is_some_and(|host| host.refresh_after_load());
                if loading && let Some(host) = self.hosts.get(&id) {
                    host.attach_menus(id, &context.sender());
                }
                // Typing moves the caret before the text change is reported; the headings
                // after it have moved too.
                if !loading {
                    self.follow_edit(id);
                }
                // In the Markdown view, the caret picks out the current heading in Navigation.
                if !loading
                    && self.source_id == Some(id)
                    && let Some(host) = self.hosts.get(&id)
                {
                    self.caret = rich::caret(&host.document)
                        .ok()
                        .map(|(start, _)| (id, start));
                }
                if !loading
                    && self
                        .hosts
                        .get(&id)
                        .is_some_and(|host| host.kind == HostKind::Flow)
                {
                    self.active = Some(id);
                    self.refresh_selection();
                    if let Some(host) = self.hosts.get(&id) {
                        self.caret = rich::caret(&host.document)
                            .ok()
                            .map(|(start, _)| (id, start));
                    }
                }
            }
            Msg::Resized(id) => self.refresh_rules(id),
            // Typing: only what must keep up with each key happens now. The rest reads the
            // whole document, so it waits until typing pauses (`Msg::Idle`). A change in the
            // editor's height (a line added or removed) still moves the heading rules at
            // once, through `Msg::Resized`.
            Msg::Changed(id) => {
                self.dirty = true;
                // Typing in the editor with the caret keeps blank page below the line, per
                // Settings › Options.
                if typed && self.active == Some(id) {
                    self.keep_room_below_caret(id);
                }
                self.follow_edit(id);
                self.typed_in.insert(id);
                host::after_typing(&context.sender());
            }
            Msg::Edited(id) => {
                self.dirty = true;
                self.typed_in.remove(&id);
                self.settle(id);
            }
            Msg::Idle => {
                let ids: Vec<Id> = self.typed_in.drain().collect();
                for id in ids {
                    self.settle(id);
                }
            }
            Msg::Pasted(id) => self.normalize(id),
            Msg::Shortcut(id, shortcut) => {
                if self
                    .hosts
                    .get(&id)
                    .is_some_and(|host| host.kind == HostKind::Flow)
                {
                    self.active = Some(id);
                }
                self.run(shortcut.command());
            }
            Msg::AppShortcut(shortcut @ (Shortcut::FindNext | Shortcut::FindPrevious))
                if !self.matches.is_empty() =>
            {
                self.step_match(shortcut == Shortcut::FindNext, context);
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
            Msg::CodeLanguage(id, language) => {
                if let Some(Segment::Code {
                    language: current, ..
                }) = self.segment_mut(id)
                    && *current != language
                {
                    *current = language;
                    self.dirty = true;
                }
            }
            Msg::Cell(id, row, column, text) => {
                if let Some(Segment::Table { table, .. }) = self.segment_mut(id)
                    && let Some(cell) = table.cells.get_mut(row).and_then(|r| r.get_mut(column))
                    && *cell != text
                {
                    *cell = text;
                    self.dirty = true;
                }
            }
            Msg::Table(id, edit) => self.edit_table(id, edit),
            Msg::MediaSource(id, text) => {
                if let Some(Segment::Media { media, .. }) = self.segment_mut(id)
                    && media.source() != text
                {
                    media.set("source", &text);
                    self.dirty = true;
                }
            }
            Msg::MediaAlt(id, text) => {
                if let Some(Segment::Media { media, .. }) = self.segment_mut(id)
                    && media.alt_text() != text
                {
                    media.set("alt-text", &text);
                    self.dirty = true;
                }
            }
            Msg::Delete(id) => self.delete_segment(id),
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
                self.focus_active();
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
                if let Some(&(_, size)) = index.and_then(|index| PARAGRAPH_SIZES.get(index))
                    && size != self.options.paragraph_size
                {
                    self.options.paragraph_size = size;
                    rich::set_body_size(size);
                    self.restyle_page();
                }
            }
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
                } else if index == ZOOM_PRESETS.len() {
                    self.run(Command::Zoom(Zoom::PageWidth));
                }
            }
            Msg::WheelZoom(delta) => {
                // A step for each whole notch; a reversed direction drops what was gathered.
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
            Msg::ZoomPreset(None) => {}
            Msg::Navigate(id, position) => {
                // A heading goes to the top of the window, as in Word, whatever is below it.
                self.reach_room = true;
                self.navigating.set(true);
                self.navigate(id, (position, position), 0.0, context);
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
                    self.navigate(issue.editor, (issue.start, issue.end), 0.33, context);
                }
            }
            Msg::SpellChoose(index, choice) => {
                if let Some(issue) = self.spelling.get(index) {
                    self.spelling_choice
                        .insert((issue.editor, issue.start), choice);
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
                    self.check_spelling();
                }
            }
            Msg::SpellIgnoreAll(index) => {
                if let Some(issue) = self.spelling.get(index) {
                    self.ignored_words.insert(issue.word.clone());
                    self.check_spelling();
                }
            }
            Msg::SpellApplyAll => {
                let all = self.spelling.clone();
                self.correct(&all);
            }
            Msg::GoToMatch(index) => {
                self.current_match = Some(index);
                self.go_to_match(context);
            }
            Msg::NextMatch => self.step_match(true, context),
            Msg::PreviousMatch => self.step_match(false, context),
            Msg::ReplaceOne => self.replace_current(context),
            Msg::ReplaceAll => self.replace_all(),
            Msg::ShowMeHow => {
                self.tab = Tab::Home;
                self.status =
                    "Click in a line, then choose a heading in the Styles box on the Home tab."
                        .into();
            }
            Msg::ContextCommand(id, command) => {
                self.active = Some(id);
                self.refresh_selection();
                // Quote, as on the ribbon, turns a quote back into a paragraph.
                let command = match command {
                    Command::Kind(ParagraphKind::Quote)
                        if self.selection.kind == ParagraphKind::Quote =>
                    {
                        Command::Kind(ParagraphKind::Normal)
                    }
                    other => other,
                };
                self.run(command);
            }
            Msg::ScrollTo(id, position, ratio) => {
                if let Some(host) = self.hosts.get(&id)
                    && let Err(error) = host.scroll_to(position, ratio)
                {
                    crate::log::log!("scrolling to a heading failed: {error:?}");
                }
            }
            Msg::ToggleOutline(key) => {
                if !self.collapsed.remove(&key) {
                    self.collapsed.insert(key);
                }
            }
        }
        self.apply_zoom();
        self.close_guard.dirty.set(self.dirty);
        self.visual_page
            .set(self.mode == Mode::Visual && self.settings.is_none() && self.dialog.is_none());
        self.remember_choices();
    }
}

impl Editor {
    fn new_id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn segment_mut(&mut self, id: Id) -> Option<&mut Segment> {
        self.segments.iter_mut().find(|segment| segment.id() == id)
    }

    /// Registers a text editor that still has to be styled and attached.
    fn track_editor(&mut self, id: Id, content: Content) {
        self.editors.insert(id, ElementRef::new());
        self.wrappers.insert(id, ElementRef::new());
        self.pending_content.insert(id, content);
        self.generation += 1;
    }

    fn forget_editor(&mut self, id: Id) {
        self.editors.remove(&id);
        self.wrappers.remove(&id);
        self.hosts.remove(&id);
        self.styled.remove(&id);
        self.in_flight.remove(&id);
        self.pending_content.remove(&id);
        self.headings.remove(&id);
        self.heading_lengths.remove(&id);
        if self.active == Some(id) {
            self.active = None;
        }
    }

    fn make_segment(&mut self, block: Block) -> Segment {
        let id = self.new_id();
        match block {
            Block::Paragraph(paragraph) => Segment::Flow {
                id,
                initial: vec![paragraph],
            },
            Block::Code { language, code } => Segment::Code {
                id,
                language,
                initial: code,
            },
            Block::Math(tex) => Segment::Math { id, initial: tex },
            Block::Raw(text) => Segment::Raw { id, initial: text },
            Block::Table(table) => Segment::Table {
                id,
                table: TableState::from_table(&table),
            },
            Block::Media(media) => Segment::Media { id, media },
            Block::Rule => Segment::Rule { id },
        }
    }

    fn flow_segment(&mut self, paragraphs: Vec<Paragraph>) -> Segment {
        let id = self.new_id();
        Segment::Flow {
            id,
            initial: paragraphs,
        }
    }

    /// Replaces the page with `document`. Flows are kept between every object so there is
    /// always somewhere to type.
    fn load(&mut self, document: Document) {
        self.reach_room = false;
        self.navigating.set(false);
        for id in self.segments.iter().map(Segment::id).collect::<Vec<_>>() {
            self.forget_editor(id);
        }
        if let Some(id) = self.source_id.take() {
            self.forget_editor(id);
        }
        self.source_snapshot = None;
        self.visual_active = None;
        self.properties = document.properties;
        let mut segments = Vec::new();
        let mut flow = Vec::new();
        for block in document.blocks {
            match block {
                Block::Paragraph(paragraph) => flow.push(paragraph),
                object => {
                    let paragraphs = std::mem::take(&mut flow);
                    segments.push(self.flow_segment(paragraphs));
                    segments.push(self.make_segment(object));
                }
            }
        }
        segments.push(self.flow_segment(flow));
        for segment in &segments {
            if let Some(content) = segment.content() {
                self.track_editor(segment.id(), content);
            }
        }
        self.caret = segments.first().map(|segment| (segment.id(), 0));
        self.segments = segments;
        self.active = None;
        self.selection = SelectionState::default();
        self.mode = Mode::Visual;
        self.dirty = false;
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

    /// Styles and attaches every published editor that is not ready yet.
    fn attach_pending(&mut self, context: &ComponentContext<Self>) {
        if !self.root_hooked {
            let sender = context.sender();
            let navigating = Rc::clone(&self.navigating);
            self.root_hooked = self.root.request_native_object(move |result| {
                let hooked = result
                    .map_err(|error| format!("{error:?}"))
                    .and_then(|object| {
                        host::hook_root(object, sender.clone(), Rc::clone(&navigating))
                            .map_err(|e| e.to_string())
                    });
                if let Err(error) = hooked {
                    crate::log::log!("window shortcuts are unavailable: {error}");
                }
            });
        }
        let ids: Vec<Id> = match self.mode {
            Mode::Source => self.source_id.into_iter().collect(),
            Mode::Visual => self
                .segments
                .iter()
                .filter(|segment| segment.content().is_some())
                .map(Segment::id)
                .collect(),
        };
        for id in ids {
            if self.hosts.contains_key(&id) || self.in_flight.contains(&id) {
                continue;
            }
            if !self.styled.contains(&id) {
                let Some(wrapper) = self.wrappers.get(&id) else {
                    continue;
                };
                let sender = context.sender();
                let accepted =
                    wrapper.request_resource_overrides(view::borderless_editor(), move |result| {
                        match result {
                            Ok(()) => sender.send(Msg::Styled(id)),
                            Err(error) => sender
                                .send(Msg::Failed(format!("Could not style editor: {error:?}"))),
                        };
                    });
                if accepted {
                    self.in_flight.insert(id);
                }
                continue;
            }
            let (Some(editor), Some(content)) =
                (self.editors.get(&id), self.pending_content.get(&id))
            else {
                continue;
            };
            let sender = context.sender();
            let content = content.clone();
            let accepted = editor.request_native_object(move |result| {
                let message = match result {
                    Ok(object) => match host::attach(object, id, &content, sender.clone()) {
                        Ok(host) => Msg::Attached(id, host),
                        Err(error) => Msg::Failed(format!("Could not load the editor: {error}")),
                    },
                    Err(error) => Msg::Failed(format!("Could not reach the editor: {error:?}")),
                };
                sender.send(message);
            });
            if accepted {
                self.in_flight.insert(id);
            }
        }
    }

    fn active_flow(&self) -> Option<Rc<Host>> {
        self.active
            .and_then(|id| self.hosts.get(&id))
            .filter(|host| host.kind == HostKind::Flow)
            .cloned()
    }

    fn refresh_selection(&mut self) {
        if let Some(host) = self.active_flow()
            && let Ok(state) = rich::selection_state(&host.document)
        {
            self.selection = state;
        }
    }

    fn focus_active(&self) {
        if let Some(editor) = self.active.and_then(|id| self.editors.get(&id)) {
            _ = editor.request_focus();
        }
    }

    /// Reads the page back into a document.
    pub fn collect(&self) -> Document {
        let mut blocks = Vec::new();
        for segment in &self.segments {
            let host = self.hosts.get(&segment.id());
            match segment {
                Segment::Flow { initial, .. } => {
                    let paragraphs = host.map_or_else(|| initial.clone(), |host| host.paragraphs());
                    blocks.extend(paragraphs.into_iter().map(Block::Paragraph));
                }
                Segment::Code {
                    language, initial, ..
                } => blocks.push(Block::Code {
                    language: language.trim().to_string(),
                    code: host.map_or_else(|| initial.clone(), |host| host.plain_text()),
                }),
                Segment::Math { initial, .. } => blocks.push(Block::Math(
                    host.map_or_else(|| initial.clone(), |host| host.plain_text()),
                )),
                Segment::Raw { initial, .. } => blocks.push(Block::Raw(
                    host.map_or_else(|| initial.clone(), |host| host.plain_text()),
                )),
                Segment::Table { table, .. } => blocks.push(Block::Table(table.to_table())),
                Segment::Media { media, .. } => blocks.push(Block::Media(media.clone())),
                Segment::Rule { .. } => blocks.push(Block::Rule),
            }
        }
        Document {
            properties: self.properties.clone(),
            blocks,
        }
    }

    /// The Markdown for the current state of whichever view is showing.
    fn markdown(&self) -> String {
        match self.mode {
            Mode::Visual => serialize(&self.collect()),
            Mode::Source => self
                .source_id
                .and_then(|id| self.hosts.get(&id))
                .map(|host| host.plain_text())
                .or_else(|| {
                    self.source_id
                        .and_then(|id| match self.pending_content.get(&id) {
                            Some(Content::Plain(text)) => Some(text.clone()),
                            _ => None,
                        })
                })
                .unwrap_or_default(),
        }
    }

    fn run(&mut self, command: Command) {
        match command {
            Command::New => self.confirm_discard(Pending::New),
            Command::Open => self.confirm_discard(Pending::Open),
            Command::Save => self.save(false),
            Command::About => self.settings = Some(SettingsCategory::About),
            Command::OpenUrl(url) => open_url(url),
            Command::SaveAs => self.save(true),
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
            Command::SwitchBackground => {
                self.dark_page = !self.dark_page;
                rich::set_dark_page(self.dark_page);
                self.restyle_page();
            }
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
                let selected = self
                    .active_flow()
                    .and_then(|host| rich::selected_text(&host.document).ok())
                    .filter(|text| !text.is_empty() && !text.contains(['\r', '\n']));
                if let Some(text) = selected {
                    self.query = text;
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
            Command::ToggleNavigation => {
                self.show_navigation = !self.show_navigation;
                if self.show_navigation {
                    let ids: Vec<Id> = self.hosts.keys().copied().collect();
                    for id in ids {
                        self.refresh_headings(id);
                    }
                }
            }
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
            command => {
                if self.mode != Mode::Visual {
                    return;
                }
                let Some(host) = self.active_flow() else {
                    self.report("Click in the text where the change should go.".into());
                    return;
                };
                let document = &host.document;
                let result = match command {
                    Command::Undo => document.Undo(),
                    Command::Redo => document.Redo(),
                    Command::Inline(inline) => rich::toggle_inline(document, inline),
                    Command::Kind(kind) => rich::set_paragraph_kind(document, kind),
                    Command::List(kind) => rich::toggle_list(document, kind),
                    Command::ToggleDone => rich::toggle_task_done(document),
                    Command::Indent => rich::change_level(document, 1),
                    Command::Outdent => rich::change_level(document, -1),
                    _ => Ok(()),
                };
                if let Err(error) = result {
                    crate::log::log!("command {command:?} failed: {error:?}");
                    self.report(format!("That change could not be applied: {error}"));
                }
                self.dirty = true;
                self.refresh_selection();
                if let Some(id) = self.active {
                    // A style change can make or unmake a heading without changing the
                    // editor's height.
                    self.refresh_rules(id);
                    if self.show_navigation {
                        self.refresh_headings(id);
                    }
                }
                self.focus_active();
            }
        }
    }

    /// Rebuilds the page so its text takes new page colors or a new paragraph size, as
    /// switching back from the Markdown source does. The page is unchanged, so it stays saved
    /// or not.
    fn restyle_page(&mut self) {
        if self.mode == Mode::Visual {
            let markdown = self.markdown();
            let dirty = self.dirty;
            self.load(parse(&markdown));
            self.dirty = dirty;
        } else if let Some(old) = self.source_id {
            // A fresh source editor, with the text as it is, takes the new colors. The hidden
            // visual page has the old styling, so it is rebuilt on the way back.
            self.source_snapshot = None;
            let text = self.markdown();
            self.forget_editor(old);
            let id = self.new_id();
            self.track_editor(id, Content::Plain(text));
            self.source_id = Some(id);
            self.active = None;
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        if mode == self.mode {
            return;
        }
        // Both views stay on the page, the one not in use hidden (see `view::page`), so
        // switching does not rebuild the visual page's editors unless the Markdown changed.
        match mode {
            Mode::Source => {
                let markdown = serialize(&self.collect());
                self.visual_active = self.active;
                match self.source_id {
                    // The Markdown editor from last time: given the page's Markdown as it is now.
                    Some(id) if self.hosts.contains_key(&id) => {
                        let host = Rc::clone(&self.hosts[&id]);
                        if !same_markdown(&host.plain_text(), &markdown)
                            && let Err(error) = host.write_plain(&markdown)
                        {
                            crate::log::log!("updating the Markdown failed: {error:?}");
                        }
                        self.refresh_headings(id);
                        if let Some(editor) = self.editors.get(&id) {
                            _ = editor.request_focus();
                        }
                    }
                    // Not attached yet: it starts with this text instead.
                    Some(id) => {
                        self.pending_content
                            .insert(id, Content::Plain(markdown.clone()));
                    }
                    None => {
                        let id = self.new_id();
                        self.track_editor(id, Content::Plain(markdown.clone()));
                        self.source_id = Some(id);
                    }
                }
                self.source_snapshot = Some(markdown);
                self.mode = Mode::Source;
                self.active = None;
            }
            Mode::Visual => {
                let markdown = self.markdown();
                let unchanged = self
                    .source_snapshot
                    .as_deref()
                    .is_some_and(|snapshot| same_markdown(snapshot, &markdown));
                if unchanged {
                    // The visual page is still as it was: show it again, caret and all.
                    self.mode = Mode::Visual;
                    self.active = self.visual_active.filter(|id| self.hosts.contains_key(id));
                    self.focus_active();
                } else {
                    let dirty = self.dirty;
                    self.load(parse(&markdown));
                    self.dirty = dirty;
                    self.active = None;
                }
            }
        }
    }

    /// Re-reads a flow after a paste so only formatting Markdown can hold remains.
    fn normalize(&mut self, id: Id) {
        let Some(host) = self
            .hosts
            .get(&id)
            .filter(|host| host.kind == HostKind::Flow)
        else {
            return;
        };
        let document = &host.document;
        let selection = rich::caret(document);
        let paragraphs = host.paragraphs();
        // The rewrite keeps the caret where it is (see `rich::rewrite`), and the display is held
        // until the content is final, so the page does not scroll while the flow is rewritten.
        let batched = document.BatchDisplayUpdates().is_ok();
        let (start, end) = selection.as_ref().map_or((0, 0), |&selection| selection);
        // Always runs, even if the caret is unknown: it closes the paste's undo group.
        if let Ok(moved) = host.rewrite_after_paste(&paragraphs, start)
            && selection.is_ok()
        {
            _ = rich::set_caret(document, (moved, moved + (end - start)));
        }
        if batched {
            _ = document.ApplyDisplayUpdates();
        }
        self.dirty = true;
    }

    /// Splits the active flow at the caret and inserts `block` there.
    fn insert_block(&mut self, block: Block) {
        if self.mode != Mode::Visual {
            return;
        }
        let target = self.active_flow().and(self.active).or_else(|| {
            self.segments
                .iter()
                .rev()
                .find(|s| matches!(s, Segment::Flow { .. }))
                .map(Segment::id)
        });
        let Some(target) = target else { return };
        let Some(index) = self.segments.iter().position(|s| s.id() == target) else {
            return;
        };
        let (before, after) = match self.hosts.get(&target) {
            Some(host) => rich::split_at_selection(&host.document)
                .unwrap_or_else(|_| (host.paragraphs(), Vec::new())),
            None => match &self.segments[index] {
                Segment::Flow { initial, .. } => (initial.clone(), Vec::new()),
                _ => return,
            },
        };
        self.forget_editor(target);
        let first = self.flow_segment(before);
        let object = self.make_segment(block);
        let last = self.flow_segment(after);
        let replacement = vec![first, object, last];
        for segment in &replacement {
            if let Some(content) = segment.content() {
                self.track_editor(segment.id(), content);
            }
        }
        let object_id = replacement[1].id();
        self.segments.splice(index..=index, replacement);
        self.active = Some(object_id);
        self.dirty = true;
    }

    /// Removes an object and joins the flows around it.
    fn delete_segment(&mut self, id: Id) {
        let Some(index) = self.segments.iter().position(|s| s.id() == id) else {
            return;
        };
        self.forget_editor(id);
        self.segments.remove(index);
        if index > 0
            && index < self.segments.len()
            && let (Segment::Flow { id: a, .. }, Segment::Flow { id: b, .. }) =
                (&self.segments[index - 1], &self.segments[index])
        {
            let (a, b) = (*a, *b);
            let mut paragraphs = self.flow_paragraphs(a);
            paragraphs.extend(self.flow_paragraphs(b));
            self.forget_editor(a);
            self.forget_editor(b);
            let merged = self.flow_segment(paragraphs);
            if let Some(content) = merged.content() {
                self.track_editor(merged.id(), content);
            }
            self.segments.splice(index - 1..=index, [merged]);
        }
        self.dirty = true;
    }

    fn flow_paragraphs(&self, id: Id) -> Vec<Paragraph> {
        match self.hosts.get(&id) {
            Some(host) => host.paragraphs(),
            None => self
                .segments
                .iter()
                .find_map(|segment| match segment {
                    Segment::Flow { id: flow, initial } if *flow == id => Some(initial.clone()),
                    _ => None,
                })
                .unwrap_or_default(),
        }
    }

    fn edit_table(&mut self, id: Id, edit: TableEdit) {
        let Some(Segment::Table { table, .. }) = self.segment_mut(id) else {
            return;
        };
        let columns = table.columns();
        match edit {
            TableEdit::AddRow => table.cells.push(vec![String::new(); columns]),
            TableEdit::RemoveRow if table.cells.len() > 1 => {
                table.cells.pop();
            }
            TableEdit::AddColumn => {
                for (index, row) in table.cells.iter_mut().enumerate() {
                    row.push(if index == 0 {
                        format!("Column {}", columns + 1)
                    } else {
                        String::new()
                    });
                }
                table.alignments.push(Alignment::None);
            }
            TableEdit::RemoveColumn if columns > 1 => {
                for row in &mut table.cells {
                    row.pop();
                }
                table.alignments.pop();
            }
            _ => {}
        }
        self.dirty = true;
    }

    fn open_dialog(&mut self, kind: DialogKind) {
        if self.mode != Mode::Visual {
            return;
        }
        let selected = self
            .active_flow()
            .and_then(|host| rich::selected_text(&host.document).ok())
            .unwrap_or_default();
        let fields = match kind {
            DialogKind::Link => vec![String::new(), selected],
            DialogKind::PageLink => vec![selected.clone(), String::new()],
            DialogKind::Image | DialogKind::Video => vec![String::new(), String::new()],
            DialogKind::Discard(_) | DialogKind::Rename => Vec::new(),
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
                ContentDialogResult::Primary => {
                    self.save(false);
                    if !self.dirty {
                        self.continue_pending(pending);
                    }
                }
                ContentDialogResult::Secondary => self.continue_pending(pending),
                _ => {}
            },
            _ if result != ContentDialogResult::Primary => self.focus_active(),
            DialogKind::Link | DialogKind::PageLink => {
                let (link, label) = if dialog.kind == DialogKind::Link {
                    (Link::Url(field(0)), field(1))
                } else {
                    (Link::Page(field(0)), field(1))
                };
                let target_empty = match &link {
                    Link::Url(url) | Link::Page(url) => url.is_empty(),
                };
                if let (false, Some(host)) = (target_empty, self.active_flow()) {
                    if let Err(error) = rich::insert_link(&host.document, &link, &label) {
                        self.report(format!("The link could not be added: {error}"));
                    }
                    self.dirty = true;
                }
                self.focus_active();
            }
            DialogKind::Image | DialogKind::Video => {
                if !field(0).is_empty() {
                    self.insert_block(Block::Media(Media::new(&field(0), &field(1))));
                }
            }
            DialogKind::Rename => {
                self.rename(&field(0));
                self.focus_active();
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
        let same_file = target.to_string_lossy().to_lowercase() == path.to_string_lossy().to_lowercase();
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

    fn save(&mut self, choose_path: bool) {
        let path = match (&self.path, choose_path) {
            (Some(path), false) => path.clone(),
            _ => {
                let name = self.path.as_ref().and_then(|p| p.file_name()).map_or_else(
                    || suggested_name(&self.collect()),
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
        let markdown = self.markdown();
        match std::fs::write(&path, &markdown) {
            Ok(()) => {
                self.dirty = false;
                self.status = format!("Saved {} at {}", file_name(&path), now());
                self.problem = None;
                self.path = Some(path);
            }
            Err(error) => self.report(format!("Could not save {}: {error}", path.display())),
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
}

/// A file name from the page's first heading, as SharePoint page names usually match it.
fn suggested_name(document: &Document) -> String {
    let heading = document.blocks.iter().find_map(|block| match block {
        Block::Paragraph(paragraph) if matches!(paragraph.kind, ParagraphKind::Heading(_)) => {
            Some(paragraph.plain_text())
        }
        _ => None,
    });
    let name: String = heading
        .unwrap_or_else(|| "Untitled".into())
        .chars()
        .filter(|c| !r#"\/:*?"<>|"#.contains(*c))
        .collect();
    format!("{}.md", name.trim())
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
fn renamed_path(
    path: &std::path::Path,
    name: &str,
) -> Result<Option<PathBuf>, String> {
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

/// Whether two Markdown texts are the same page, ignoring how lines end (the editor reads its
/// text back with its own line breaks) and blank lines at the end.
/// An editor's length (its story length, in the positions headings and the caret use).
fn story_length(host: &Host) -> Option<i32> {
    host.document
        .GetRange(0, 0)
        .and_then(|range| range.StoryLength())
        .ok()
}

fn same_markdown(a: &str, b: &str) -> bool {
    let normal = |text: &str| text.replace("\r\n", "\n").replace('\r', "\n");
    normal(a).trim_end_matches('\n') == normal(b).trim_end_matches('\n')
}

/// The headings of the Markdown view's text, each at the editor position its line starts on.
fn source_headings(text: &str) -> Vec<rich::HeadingInfo> {
    // Where each line starts, in the editor's UTF-16 positions.
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
            Some(rich::HeadingInfo {
                level: heading.level,
                text: heading.text,
                position: *starts.get(heading.line)?,
            })
        })
        .collect()
}

/// Opens a web address in the default browser.
fn open_url(url: &str) {
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
    /// After any change of zoom (the slider, the zoom box, Ctrl+wheel, the shortcuts): rewrites
    /// every editor's text at the new zoom (see `rich::rescale`), which clears their undo
    /// history, and measures the heading rules again. Editors not attached yet are written at
    /// the new zoom when they are.
    fn apply_zoom(&mut self) {
        let zoom = self.zoom as f32;
        if (rich::zoom() - zoom).abs() < f32::EPSILON {
            return;
        }
        let started = std::time::Instant::now();
        let from = rich::zoom();
        let typing: Vec<Option<f32>> = self.hosts.values().map(|host| host.typing_size()).collect();
        let documents: Vec<&crate::winui::RichEditTextDocument> =
            self.hosts.values().map(|host| &host.document).collect();
        if let Err(error) = rich::rescale(&documents, zoom) {
            crate::log::log!("zooming the text failed: {error:?}");
        }
        for (host, typing) in self.hosts.values().zip(typing) {
            host.fit_zoom(typing, from, zoom);
        }
        crate::log::log!(
            "zoom {zoom}: text rescaled in {:.1} ms",
            started.elapsed().as_secs_f64() * 1000.0
        );
        let flows: Vec<Id> = self
            .hosts
            .iter()
            .filter(|(_, host)| host.kind == HostKind::Flow)
            .map(|(&id, _)| id)
            .collect();
        for id in flows {
            self.refresh_rules(id);
        }
    }

    /// Brings everything read from an editor's whole document up to date after it changed:
    /// the heading rules, the ribbon, and the Navigation pane's headings, search results and
    /// spelling.
    fn settle(&mut self, id: Id) {
        if !self.hosts.contains_key(&id) {
            return;
        }
        self.refresh_rules(id);
        // Undo and redo change formatting without moving the caret, so the ribbon's Style box
        // and toggles are read again here.
        if self.active == Some(id) {
            self.refresh_selection();
        }
        if self.show_navigation {
            self.refresh_headings(id);
            if !self.query.is_empty() {
                self.run_search();
            }
            if self.explorer_tab == ExplorerTab::Spelling {
                self.check_spelling();
            }
        }
    }

    /// Measures where one text flow's heading rules go.
    fn refresh_rules(&mut self, id: Id) {
        let Some(host) = self.hosts.get(&id).filter(|host| host.kind == HostKind::Flow) else {
            return;
        };
        match rich::heading_rules(&host.document) {
            Ok(rules) => {
                if self.rules.get(&id) != Some(&rules) {
                    self.rules.insert(id, rules);
                }
            }
            Err(error) => crate::log::log!("measuring heading rules failed: {error:?}"),
        }
    }

    /// Re-reads the headings of one text flow for the Navigation pane.
    fn refresh_headings(&mut self, id: Id) {
        let Some(host) = self.hosts.get(&id) else {
            return;
        };
        let length = story_length(host);
        // The Markdown view: its headings are read from the Markdown, at their lines.
        if self.source_id == Some(id) {
            let headings = source_headings(&host.plain_text());
            self.headings.insert(id, headings);
        } else if host.kind == HostKind::Flow {
            match rich::headings(&host.document) {
                Ok(headings) => {
                    self.headings.insert(id, headings);
                }
                Err(error) => {
                    crate::log::log!("reading headings failed: {error:?}");
                    return;
                }
            }
        } else {
            return;
        }
        match length {
            Some(length) => self.heading_lengths.insert(id, length),
            None => self.heading_lengths.remove(&id),
        };
    }

    /// Keeps the Navigation pane's headings in step with typing, between full re-reads (once
    /// typing pauses; see `settle`). An edit shifts every heading after it by the change in
    /// the editor's length, so the current heading stays highlighted instead of jumping to
    /// the next one as the caret passes where that one used to start. Cheap: one length.
    fn follow_edit(&mut self, id: Id) {
        let Some(host) = self.hosts.get(&id) else {
            return;
        };
        let (Some(&old), Some(new)) = (self.heading_lengths.get(&id), story_length(host)) else {
            return;
        };
        let change = new - old;
        if change == 0 {
            return;
        }
        // Where the edit was: the earlier of the caret before it and after it (before for
        // typing, after for deleting backwards). A heading that starts there is the one
        // being typed in, and does not move.
        let after = rich::caret(&host.document).ok().map(|(start, _)| start);
        let before = self
            .caret
            .filter(|(editor, _)| *editor == id)
            .map(|(_, position)| position);
        let Some(edit) = before.into_iter().chain(after).min() else {
            return;
        };
        for heading in self.headings.get_mut(&id).into_iter().flatten() {
            if heading.position > edit {
                heading.position += change;
            }
        }
        self.heading_lengths.insert(id, new);
    }

    /// Moves the caret to a heading and scrolls it to the top of the page, like clicking a
    /// heading in Word's Navigation pane.
    /// Selects `range` in an editor, focuses it, and scrolls it into view (`ratio` of the way
    /// down the window), as Word does for Navigation pane headings and search results.
    fn navigate(
        &mut self,
        id: Id,
        range: (i32, i32),
        ratio: f64,
        context: &ComponentContext<Self>,
    ) {
        let position = range.0;
        let Some(host) = self.hosts.get(&id).cloned() else {
            return;
        };
        if rich::set_caret(&host.document, range).is_err() {
            return;
        }
        // The ribbon follows the text flow with the caret; the Markdown editor is not one.
        if host.kind == HostKind::Flow {
            self.active = Some(id);
        }
        self.caret = Some((id, position));
        self.refresh_selection();
        // Focusing the editor scrolls the page to the editor itself, so the scroll to the
        // heading is sent as a message that runs after focus has settled.
        let sender = context.sender();
        let focused = self.editors.get(&id).is_some_and(|editor| {
            editor.request_focus_result(move |_| {
                sender.send(Msg::ScrollTo(id, position, ratio));
            })
        });
        if !focused {
            context.sender().send(Msg::ScrollTo(id, position, ratio));
        }
    }

    /// The blank page kept below the caret, per Settings › Options, in pixels: a share of the
    /// window, whatever the zoom (the page is laid out at its zoomed size; see `rich::zoom`).
    pub(super) fn cursor_room(&self) -> f64 {
        // The page's share of the window: less the title band, ribbon and status bar.
        self.viewport() * f64::from(self.options.cursor_space) / 100.0
    }

    /// The page's share of the window: less the title band, ribbon and status bar.
    fn viewport(&self) -> f64 {
        (self.window_height - 170.0).max(200.0)
    }

    /// The blank page at the end: the room below the cursor, or after a Navigation pane jump a
    /// whole window, so the last heading can still reach the top (see `reach_room`).
    pub(super) fn end_room(&self) -> f64 {
        if self.reach_room {
            self.cursor_room().max(self.viewport())
        } else {
            self.cursor_room()
        }
    }

    /// Scrolls the page, if needed, so the caret has the chosen room below it.
    fn keep_room_below_caret(&self, id: Id) {
        let room = self.cursor_room();
        // Diagnostic: KB_NO_ROOM=1 skips this scroll, to measure what it costs while typing.
        if room <= 0.0 || crate::log::switch("KB_NO_ROOM") {
            return;
        }
        if let Some(host) = self.hosts.get(&id)
            && let Err(error) = host.keep_room_below_caret(room)
        {
            crate::log::log!("keeping room below the caret failed: {error:?}");
        }
    }

    /// The document's headings in page order, for the Navigation pane.
    pub fn outline(&self) -> Vec<OutlineEntry> {
        // The editors whose headings make the outline, in page order: the text flows, or in
        // the Markdown view its one editor.
        let editors: Vec<Id> = match (self.mode, self.source_id) {
            (Mode::Source, Some(id)) => vec![id],
            _ => self
                .segments
                .iter()
                .filter_map(|segment| match segment {
                    Segment::Flow { id, .. } => Some(*id),
                    _ => None,
                })
                .collect(),
        };
        let order: Vec<Id> = match self.mode {
            Mode::Source => editors.clone(),
            Mode::Visual => self.segments.iter().map(Segment::id).collect(),
        };
        let caret = self.caret.and_then(|(id, position)| {
            let index = order.iter().position(|&editor| editor == id)?;
            Some((index, position))
        });
        let mut occurrences: HashMap<(u8, String), usize> = HashMap::new();
        let mut entries: Vec<OutlineEntry> = Vec::new();
        let mut current = None;
        for id in &editors {
            let Some(index) = order.iter().position(|editor| editor == id) else {
                continue;
            };
            for heading in self.headings.get(id).into_iter().flatten() {
                let count = occurrences
                    .entry((heading.level, heading.text.clone()))
                    .or_default();
                let key = OutlineKey {
                    level: heading.level,
                    text: heading.text.clone(),
                    occurrence: *count,
                };
                *count += 1;
                if caret.is_some_and(|caret| (index, heading.position) <= caret) {
                    current = Some(entries.len());
                }
                entries.push(OutlineEntry {
                    editor: *id,
                    heading: heading.clone(),
                    key,
                    has_children: false,
                    current: false,
                });
            }
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
    /// The editors searched, in page order: the visual page's text editors, or the source.
    /// Lists every misspelling in the page for the Spelling pane, leaving out what the user
    /// ignored. Only the visual view is checked: Markdown syntax is not prose.
    fn check_spelling(&mut self) {
        self.spelling.clear();
        if self.mode != Mode::Visual {
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
        let Some(checker) = self.spell_checker.clone() else {
            return;
        };
        for id in self.searchable_editors() {
            let Some(host) = self.hosts.get(&id).filter(|h| h.kind == HostKind::Flow) else {
                continue;
            };
            match spelling::check_editor(&checker, id, &host.document) {
                Ok(issues) => self.spelling.extend(issues),
                Err(error) => crate::log::log!("checking spelling failed: {error:?}"),
            }
        }
        let ignored_once = &self.ignored_once;
        let ignored_words = &self.ignored_words;
        let mut seen: HashMap<(Id, String), usize> = HashMap::new();
        self.spelling.retain(|issue| {
            let count = seen.entry((issue.editor, issue.word.clone())).or_default();
            let key = (issue.editor, issue.word.clone(), *count);
            *count += 1;
            !ignored_words.contains(&issue.word) && !ignored_once.contains(&key)
        });
        let issues = &self.spelling;
        self.spelling_choice
            .retain(|key, _| issues.iter().any(|i| (i.editor, i.start) == *key));
    }

    /// Identifies a misspelling across edits: its editor, word, and which occurrence of that
    /// word in the editor it is (counting ignored ones too).
    fn occurrence_key(&self, index: usize) -> Option<(Id, String, usize)> {
        let issue = self.spelling.get(index)?;
        let mut ordinal = 0;
        let host = self.hosts.get(&issue.editor)?;
        let checker = self.spell_checker.as_ref()?;
        for other in spelling::check_editor(checker, issue.editor, &host.document).ok()? {
            if other.start == issue.start {
                return Some((issue.editor, issue.word.clone(), ordinal));
            }
            if other.word == issue.word {
                ordinal += 1;
            }
        }
        None
    }

    /// The suggestion picked for a misspelling, or its first one.
    fn chosen_suggestion(&self, issue: &spelling::Issue) -> Option<String> {
        let choice = self
            .spelling_choice
            .get(&(issue.editor, issue.start))
            .copied()
            .unwrap_or(0);
        issue.suggestions.get(choice).cloned()
    }

    /// Replaces each misspelling with its chosen suggestion (one undo step per editor), then
    /// checks the page again. Misspellings without a suggestion are left alone.
    fn correct(&mut self, issues: &[spelling::Issue]) {
        let mut by_editor: HashMap<Id, Vec<(i32, i32, String)>> = HashMap::new();
        for issue in issues {
            if let Some(replacement) = self.chosen_suggestion(issue) {
                by_editor.entry(issue.editor).or_default().push((
                    issue.start,
                    issue.end,
                    replacement,
                ));
            }
        }
        let mut changed = 0;
        for (id, changes) in by_editor {
            if let Some(host) = self.hosts.get(&id) {
                match rich::replace_each(&host.document, &changes) {
                    Ok(count) => changed += count,
                    Err(error) => crate::log::log!("correcting spelling failed: {error:?}"),
                }
            }
        }
        if changed > 0 {
            self.dirty = true;
            self.status = if changed == 1 {
                "Corrected 1 misspelling.".into()
            } else {
                format!("Corrected {changed} misspellings.")
            };
        }
        self.check_spelling();
    }

    fn searchable_editors(&self) -> Vec<Id> {
        match self.mode {
            Mode::Source => self.source_id.into_iter().collect(),
            Mode::Visual => self
                .segments
                .iter()
                .filter(|segment| segment.content().is_some())
                .map(Segment::id)
                .collect(),
        }
    }

    /// Finds the query in every editor. Keeps the current result where it was when possible.
    fn run_search(&mut self) {
        let previous = self
            .current_match
            .and_then(|index| self.matches.get(index).cloned());
        self.matches.clear();
        if self.query.is_empty() {
            self.current_match = None;
            return;
        }
        for id in self.searchable_editors() {
            let Some(host) = self.hosts.get(&id) else {
                continue;
            };
            let found = rich::find_all(
                &host.document,
                &self.query,
                self.match_case,
                self.whole_words,
            );
            for (start, end) in found.unwrap_or_default() {
                let (before, text, after) =
                    rich::snippet(&host.document, start, end).unwrap_or_default();
                self.matches.push(SearchMatch {
                    editor: id,
                    start,
                    end,
                    before,
                    text,
                    after,
                });
            }
        }
        // After an edit or a replacement, stay on the first result at or after the old one.
        let order = |m: &SearchMatch| {
            let segment = self
                .segments
                .iter()
                .position(|s| s.id() == m.editor)
                .unwrap_or(0);
            (segment, m.start)
        };
        self.current_match = previous.and_then(|previous| {
            let previous = order(&previous);
            self.matches.iter().position(|m| order(m) >= previous)
        });
    }

    fn step_match(&mut self, forward: bool, context: &ComponentContext<Self>) {
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
        self.go_to_match(context);
    }

    /// Selects the current result in the page and scrolls it a third of the way down.
    fn go_to_match(&mut self, context: &ComponentContext<Self>) {
        let Some(found) = self
            .current_match
            .and_then(|index| self.matches.get(index).cloned())
        else {
            return;
        };
        self.navigate(found.editor, (found.start, found.end), 0.33, context);
    }

    /// Replaces the current result (or goes to the first one), then moves to the next, as
    /// Word's Replace button does.
    fn replace_current(&mut self, context: &ComponentContext<Self>) {
        let Some(found) = self
            .current_match
            .and_then(|index| self.matches.get(index).cloned())
        else {
            self.step_match(true, context);
            return;
        };
        let Some(host) = self.hosts.get(&found.editor) else {
            return;
        };
        if let Err(error) = rich::replace_ranges(
            &host.document,
            &[(found.start, found.end)],
            &self.replacement,
        ) {
            self.report(format!("The text could not be replaced: {error}"));
            return;
        }
        self.after_replace(found.editor);
        if !self.matches.is_empty() {
            self.go_to_match(context);
        }
    }

    fn replace_all(&mut self) {
        let mut replaced = 0;
        let editors: Vec<Id> = self
            .matches
            .iter()
            .map(|m| m.editor)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        for id in editors {
            let ranges: Vec<(i32, i32)> = self
                .matches
                .iter()
                .filter(|m| m.editor == id)
                .map(|m| (m.start, m.end))
                .collect();
            let Some(host) = self.hosts.get(&id) else {
                continue;
            };
            match rich::replace_ranges(&host.document, &ranges, &self.replacement) {
                Ok(count) => replaced += count,
                Err(error) => self.report(format!("Some text could not be replaced: {error}")),
            }
            self.after_replace(id);
        }
        self.current_match = None;
        self.run_search();
        self.status = match replaced {
            0 => "No results to replace.".into(),
            1 => "Replaced 1 result.".into(),
            count => format!("Replaced {count} results."),
        };
    }

    fn after_replace(&mut self, id: Id) {
        self.dirty = true;
        self.refresh_headings(id);
        self.run_search();
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
mod zoom_tests {
    use super::*;

    #[test]
    fn a_renamed_file_keeps_its_folder_and_extension() {
        let path = std::path::Path::new(r"C:\KB\old name.md");
        let renamed = |name| renamed_path(path, name);
        assert_eq!(renamed("New name"), Ok(Some(PathBuf::from(r"C:\KB\New name.md"))));
        assert_eq!(renamed("notes.txt"), Ok(Some(PathBuf::from(r"C:\KB\notes.txt"))));
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
}
