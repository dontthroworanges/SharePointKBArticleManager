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
    Home,
    Insert,
    View,
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
    /// File › About: the version, what the app is, and where it lives.
    About,
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
    About,
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
    Pasted(Id),
    Shortcut(Id, Shortcut),
    /// A window-wide shortcut, from wherever focus is.
    AppShortcut(Shortcut),
    Command(Command),
    FileMenu(String),
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
    /// A preset picked in the View tab's zoom box.
    ZoomPreset(Option<usize>),
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
}

/// The File menu: label and command. Menu clicks are reported by label.
pub const FILE_MENU: [(&str, Command); 5] = [
    ("New page", Command::New),
    ("Open…", Command::Open),
    ("Save", Command::Save),
    ("Save as…", Command::SaveAs),
    ("About", Command::About),
];

/// The app's name, in the title bar and the About box.
pub const APP_NAME: &str = "SharePoint Article Manager";
/// The version, from Cargo.toml.
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Where the project lives, linked from the About box.
pub const PROJECT_URL: &str = "https://github.com/dontthroworanges/SharePointKBArticleManager";

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
    /// Content for text editors that are not attached yet.
    pending_content: HashMap<Id, Content>,
    generation: u64,
    next_id: Id,
    active: Option<Id>,
    selection: SelectionState,
    mode: Mode,
    source_id: Option<Id>,
    tab: Tab,
    dirty: bool,
    show_properties: bool,
    dialog: Option<Dialog>,
    status: String,
    window: WindowRef,
    close_guard: Rc<CloseGuard>,
    window_width: f64,
    show_navigation: bool,
    /// The Document Editor's width, set by dragging its right edge.
    navigation_width: f64,
    /// While the edge is dragged: where the drag started (window x) and the width then.
    resizing: Option<(f64, f64)>,
    /// The drag handle, to give it the resize cursor.
    pub(super) splitter: ElementRef<Grid>,
    /// Headings of each attached text flow, for the Navigation pane.
    headings: HashMap<Id, Vec<rich::HeadingInfo>>,
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
}

pub const MIN_ZOOM: f64 = 0.1;
pub const MAX_ZOOM: f64 = 5.0;
/// The presets in the View tab's zoom box, as percentages.
pub const ZOOM_PRESETS: [u16; 8] = [50, 75, 100, 125, 150, 200, 300, 500];

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
            pending_content: HashMap::new(),
            generation: 0,
            next_id: 1,
            active: None,
            selection: SelectionState::default(),
            mode: Mode::Visual,
            source_id: None,
            tab: Tab::Home,
            dirty: false,
            show_properties: false,
            dialog: None,
            status: "Ready".into(),
            window: context.window(),
            close_guard,
            window_width: 1280.0,
            show_navigation: false,
            navigation_width: saved.explorer_width.unwrap_or(340.0),
            resizing: None,
            splitter: ElementRef::new(),
            headings: HashMap::new(),
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
        };
        match path {
            Some(path) => editor.open_path(path.clone()),
            None => editor.load(Document::default()),
        }
        editor
    }

    fn update(&mut self, message: Msg, context: &ComponentContext<Self>) {
        crate::log::log!("{message:?} active={:?}", self.active);
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
                self.hosts.insert(id, host);
                self.refresh_headings(id);
                if focus || self.active == Some(id) {
                    self.active = Some(id);
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
            Msg::Changed(id) | Msg::Edited(id) => {
                self.dirty = true;
                // Undo and redo change formatting without moving the caret, so the ribbon's
                // Style box and toggles are read again here.
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
            Msg::FileMenu(label) => {
                if let Some(&(_, command)) = FILE_MENU.iter().find(|(name, _)| *name == label) {
                    self.run(command);
                }
            }
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
                    "Insert" => Tab::Insert,
                    "View" => Tab::View,
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
            Msg::WindowSize(size) => self.window_width = size.width,
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
                }
            }
            Msg::ZoomPreset(Some(index)) => {
                if let Some(&percent) = ZOOM_PRESETS.get(index) {
                    self.run(Command::Zoom(Zoom::To(percent)));
                }
            }
            Msg::ZoomPreset(None) => {}
            Msg::Navigate(id, position) => self.navigate(id, (position, position), 0.0, context),
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
        self.close_guard.dirty.set(self.dirty);
        self.remember_choices();
    }

    fn view(&self, _: &Option<PathBuf>, context: &mut ViewContext<Self>) -> View {
        self.render(context)
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
        for id in self.segments.iter().map(Segment::id).collect::<Vec<_>>() {
            self.forget_editor(id);
        }
        if let Some(id) = self.source_id.take() {
            self.forget_editor(id);
        }
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
            self.root_hooked = self.root.request_native_object(move |result| {
                let hooked = result
                    .map_err(|error| format!("{error:?}"))
                    .and_then(|object| {
                        host::hook_root(object, sender.clone()).map_err(|e| e.to_string())
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
            Command::About => {
                self.dialog = Some(Dialog {
                    kind: DialogKind::About,
                    fields: Vec::new(),
                });
            }
            Command::SaveAs => self.save(true),
            Command::Mode(mode) => self.set_mode(mode),
            Command::Theme(choice) => self.theme = choice,
            Command::SwitchBackground => {
                self.dark_page = !self.dark_page;
                rich::set_dark_page(self.dark_page);
                // Rebuild the page so its text takes the new colors, as switching back from
                // the Markdown source does. The page is unchanged, so it stays saved or not.
                if self.mode == Mode::Visual {
                    let markdown = self.markdown();
                    let dirty = self.dirty;
                    self.load(parse(&markdown));
                    self.dirty = dirty;
                } else if let Some(old) = self.source_id {
                    // A fresh source editor, with the text as it is, takes the new colors.
                    let text = self.markdown();
                    self.forget_editor(old);
                    let id = self.new_id();
                    self.track_editor(id, Content::Plain(text));
                    self.source_id = Some(id);
                    self.active = None;
                }
            }
            Command::ToggleProperties => self.show_properties = !self.show_properties,
            Command::Zoom(change) => {
                // Word steps to the next multiple of 10%.
                let percent = (self.zoom * 100.0).round();
                let target = match change {
                    Zoom::In => ((percent / 10.0).floor() + 1.0) * 10.0,
                    Zoom::Out => ((percent / 10.0).ceil() - 1.0) * 10.0,
                    Zoom::To(percent) => f64::from(percent),
                };
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
                if let Some(id) = self.active.filter(|_| self.show_navigation) {
                    self.refresh_headings(id);
                }
                self.focus_active();
            }
        }
    }

    fn set_mode(&mut self, mode: Mode) {
        if mode == self.mode {
            return;
        }
        match mode {
            Mode::Source => {
                let markdown = serialize(&self.collect());
                let id = self.new_id();
                self.track_editor(id, Content::Plain(markdown));
                self.source_id = Some(id);
                self.mode = Mode::Source;
                // The visual editors leave the tree; switching back rebuilds them from the source.
                for id in self.segments.iter().map(Segment::id).collect::<Vec<_>>() {
                    self.hosts.remove(&id);
                }
            }
            Mode::Visual => {
                let markdown = self.markdown();
                let dirty = self.dirty;
                self.load(parse(&markdown));
                self.dirty = dirty;
            }
        }
        self.active = None;
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
        // Always runs, even if the caret is unknown: it closes the paste's undo group.
        if host.rewrite_after_paste(&paragraphs).is_ok()
            && let Ok(selection) = selection
        {
            _ = rich::set_caret(document, selection);
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
            DialogKind::Discard(_) | DialogKind::About => Vec::new(),
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
            DialogKind::About => self.focus_active(),
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

impl Editor {
    /// Re-reads the headings of one text flow for the Navigation pane.
    fn refresh_headings(&mut self, id: Id) {
        let Some(host) = self
            .hosts
            .get(&id)
            .filter(|host| host.kind == HostKind::Flow)
        else {
            return;
        };
        match rich::headings(&host.document) {
            Ok(headings) => {
                self.headings.insert(id, headings);
            }
            Err(error) => crate::log::log!("reading headings failed: {error:?}"),
        }
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
        self.active = Some(id);
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

    /// The document's headings in page order, for the Navigation pane.
    pub fn outline(&self) -> Vec<OutlineEntry> {
        let caret = self.caret.and_then(|(id, position)| {
            let index = self.segments.iter().position(|s| s.id() == id)?;
            Some((index, position))
        });
        let mut occurrences: HashMap<(u8, String), usize> = HashMap::new();
        let mut entries: Vec<OutlineEntry> = Vec::new();
        let mut current = None;
        for (index, segment) in self.segments.iter().enumerate() {
            let Segment::Flow { id, .. } = segment else {
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
