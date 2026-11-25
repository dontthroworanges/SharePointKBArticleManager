//! The ribbon's commands, laid out on one line or as the Classic ribbon.
//!
//! Each tab is described once, as named groups of commands, and drawn in either layout, as in
//! Word for the web: the single-line ribbon puts every command in one row of small buttons; the
//! Classic ribbon shows the groups side by side, with larger buttons and each group's name
//! underneath. In both, commands that do not fit the window move to the "More" (…) menu.

use super::*;

/// The height of a Classic ribbon group above its name: two rows of small buttons.
const CLASSIC_HEIGHT: f64 = 72.0;
/// The chevron that switches the layout, with its margin.
pub(super) const CHEVRON_WIDTH: f64 = 48.0;

/// Fluent UI System Icons' "Flowchart" (20 px, regular; MIT licence), as path markup from its
/// SVG, for Mermaid diagrams. Windows does not ship that icon font, so the outline is drawn.
/// `F1` keeps SVG's nonzero fill rule.
const FLOWCHART: &str = "F1 M4.5 3C3.67 3 3 3.67 3 4.5v2C3 7.33 3.67 8 4.5 8H5v3.84l-.2 .16L3 13.8a1 1 0 0 0 0 1.4L4.8 17a1 1 0 0 0 1.4 0L8 15.2l.16-.2H12v.5c0 .83 .67 1.5 1.5 1.5h2c.83 0 1.5-.67 1.5-1.5v-2c0-.83-.67-1.5-1.5-1.5h-2c-.83 0-1.5 .67-1.5 1.5v.5H8.16L8 13.8 6.2 12l-.2-.16V8h.5C7.33 8 8 7.33 8 6.5v-2C8 3.67 7.33 3 6.5 3zM4 4.5c0-.28 .22-.5 .5-.5h2c.28 0 .5 .22 .5 .5v2a.5 .5 0 0 1-.5 .5h-2a.5 .5 0 0 1-.5-.5zm-.3 10 1.8-1.8 1.8 1.8-1.8 1.8zm9.8-1.5h2c.28 0 .5 .22 .5 .5v2a.5 .5 0 0 1-.5 .5h-2a.5 .5 0 0 1-.5-.5v-2c0-.28 .22-.5 .5-.5";
/// Fluent UI System Icons' "Text Proofing Tools" (20 px, regular; MIT licence), for Spelling.
const PROOFING: &str = "F1 M4.94 3c.21 0 .4 .14 .47 .33l1.77 5a.5 .5 0 0 1-.94 .34l-.26-.73V8H3.73l-.26 .68a.5 .5 0 1 1-.94-.36l1.94-5A.5 .5 0 0 1 4.94 3m-.02 1.94L4.12 7h1.53zm12.49-.41c-.25-.69-.6-1.1-1.06-1.33-.4-.2-.83-.2-1.1-.2h-.03c-.55 0-1 .19-1.33 .49-.32 .29-.52 .65-.64 .99A4 4 0 0 0 13 5.77v.05c0 .39 0 1.09 .25 1.73 .13 .33 .33 .67 .66 .93q.48 .41 1.25 .5c.42 .05 1.67 .02 2.23-1.24a.5 .5 0 1 0-.9-.4c-.3 .64-.94 .68-1.22 .65a1.3 1.3 0 0 1-.73-.28 1.3 1.3 0 0 1-.36-.52A4 4 0 0 1 14 5.8V5.8l.02-.28q.04-.31 .16-.68 .14-.37 .38-.6a1 1 0 0 1 .66-.23c.3 0 .5 0 .7 .1 .15 .08 .36 .25 .55 .77a.5 .5 0 1 0 .94-.34m-5.94 1.3q.31-.4 .32-1.07 0-.57-.22-.96a1.4 1.4 0 0 0-.56-.57c-.4-.23-.84-.23-1-.23H8.5a.5 .5 0 0 0-.5 .5v5c0 .28 .22 .5 .5 .5h1.75c.22 0 .66-.03 1.07-.27 .45-.26 .8-.74 .8-1.49q0-.57-.25-.98a2 2 0 0 0-.4-.42m-.95-1.73q.1 .06 .18 .18a1 1 0 0 1 .1 .48c0 .4-.16 .54-.28 .62-.17 .1-.38 .12-.5 .12H9V4h1q.26-.02 .52 .1M9.36 6.5h.83c.18 0 .43 0 .62 .1q.14 .07 .2 .18 .1 .12 .1 .46c0 .38-.14 .54-.3 .62-.18 .11-.42 .14-.56 .14H9V6.5zm4.4 4.32a.5 .5 0 1 0-.78-.64l-3.74 4.58-1.89-1.89a.5 .5 0 0 0-.7 .71l2.27 2.27a.5 .5 0 0 0 .74-.03z";

/// Fluent's DarkTheme (20, regular): a half-filled circle, for Switch Background.
const DARK_THEME: &str = "F1 M10 3A7 7 0 1 1 10 17Z M10 2A8 8 0 1 0 10 18A8 8 0 0 0 10 2Z";

/// A command's icon: Fluent path markup sized for 20 pixels (see `icons`), kept as a
/// description so it can be drawn in either layout.
#[derive(Clone, Copy)]
enum Icon {
    Path(&'static str),
}

impl Icon {
    fn view(self) -> View {
        let Self::Path(data) = self;
        path_icon(data, 20.0)
    }
}

/// A ribbon button.
struct Tool {
    icon: Icon,
    /// The name under a large button, beside a stacked one, and in the "More" menu.
    label: &'static str,
    tip: &'static str,
    active: bool,
    enabled: bool,
    command: Command,
    /// Whether the single-line ribbon shows the label beside the icon.
    labeled: bool,
}

impl Tool {
    fn new(icon: Icon, label: &'static str, tip: &'static str, command: Command) -> Self {
        Self {
            icon,
            label,
            tip,
            active: false,
            enabled: true,
            command,
            labeled: false,
        }
    }

    fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    fn labeled(mut self) -> Self {
        self.labeled = true;
        self
    }

    fn entry(&self) -> Entry {
        Entry::command(self.label, self.command, self.enabled, self.active)
    }
}

enum Control {
    Tool(Tool),
    /// A control drawn the same way in both layouts, such as a drop-down box.
    Custom(Item),
}

/// How a group arranges its buttons in the Classic ribbon.
enum Shape {
    /// Large buttons, icon above name.
    Large,
    /// Small icon buttons in two rows.
    Rows,
    /// Small buttons with names, one under the other.
    Stack,
}

struct Group {
    name: &'static str,
    shape: Shape,
    controls: Vec<Control>,
}

impl Group {
    fn new(name: &'static str, shape: Shape, controls: Vec<Control>) -> Self {
        Self {
            name,
            shape,
            controls,
        }
    }
}

fn tools(tools: impl IntoIterator<Item = Tool>) -> Vec<Control> {
    tools.into_iter().map(Control::Tool).collect()
}

impl Editor {
    /// The selected tab's commands, laid out for the chosen ribbon layout.
    pub(super) fn ribbon_items(&self, context: &mut ViewContext<Self>) -> Vec<Item> {
        let groups = match self.tab {
            Tab::File => self.file_groups(),
            Tab::Home => self.home_groups(context),
            Tab::Insert => self.insert_groups(),
            Tab::View => self.view_groups(context),
            Tab::Help => help_groups(),
        };
        match self.ribbon_layout {
            RibbonLayout::SingleLine => single_line(context, groups),
            RibbonLayout::Classic => classic(context, groups),
        }
    }

    /// The chevron at the end of the ribbon and its menu: Classic or single-line ribbon.
    pub(super) fn layout_chevron(&self, context: &mut ViewContext<Self>) -> View {
        let check = |layout: RibbonLayout, name: &str| {
            if self.ribbon_layout == layout {
                format!("\u{2713}  {name}")
            } else {
                name.to_string()
            }
        };
        Button::new()
            .style(ButtonStyle::Subtle)
            // Room for the icon inside the button's own padding (11 pixels each side).
            .width(40.0)
            .height(32.0)
            .margin(Thickness::new(8.0, 0.0, 0.0, 0.0))
            .vertical_alignment(VerticalAlignment::Bottom)
            // Reactor crashes on a tooltip around a view with a menu, so the icon carries it.
            // A Viewbox draws the icon small; Reactor's FontIcon has no font size.
            .content(
                Viewbox::new()
                    .width(14.0)
                    .height(14.0)
                    // Up when the ribbon is expanded, as a hint that it can collapse.
                    .slot(
                        ViewboxSlot::Child,
                        glyph(match self.ribbon_layout {
                            RibbonLayout::Classic => "\u{E70E}",
                            RibbonLayout::SingleLine => "\u{E70D}",
                        }),
                    )
                    .tooltip("Ribbon layout"),
            )
            .menu(Menu::new(
                [
                    MenuItem::item("classic", check(RibbonLayout::Classic, RIBBON_LAYOUTS[0].0)),
                    MenuItem::item(
                        "single-line",
                        check(RibbonLayout::SingleLine, RIBBON_LAYOUTS[1].0),
                    ),
                ],
                context.callback(Msg::RibbonLayout),
            ))
    }

    /// The File tab: new, open and save, as Word for the web's File tab offers them.
    fn file_groups(&self) -> Vec<Group> {
        vec![
            Group::new(
                "File",
                Shape::Large,
                tools([
                    Tool::new(
                        Icon::Path(icons::NEW_PAGE),
                        "New page",
                        "Start a new, empty page (Ctrl+N)",
                        Command::New,
                    )
                    .labeled(),
                    Tool::new(
                        Icon::Path(icons::OPEN),
                        "Open…",
                        "Open a Markdown file (Ctrl+O)",
                        Command::Open,
                    )
                    .labeled(),
                    // Only a page saved to a file has a name to change.
                    Tool::new(
                        Icon::Path(icons::RENAME),
                        "Rename…",
                        "Rename the open file",
                        Command::Rename,
                    )
                    .enabled(self.path.is_some())
                    .labeled(),
                ]),
            ),
            Group::new(
                "Save",
                Shape::Large,
                tools([
                    // Greyed out while the file on disk already matches the page.
                    Tool::new(Icon::Path(icons::SAVE), "Save", "Save (Ctrl+S)", Command::Save)
                        .enabled(self.dirty)
                        .labeled(),
                    Tool::new(
                        Icon::Path(icons::SAVE_AS),
                        "Save as…",
                        "Save to a new file (Ctrl+Shift+S)",
                        Command::SaveAs,
                    )
                    .labeled(),
                ]),
            ),
        ]
    }

    fn home_groups(&self, context: &mut ViewContext<Self>) -> Vec<Group> {
        let state = self.selection;
        let on = self.mode == Mode::Visual;
        let style_index = STYLES.iter().position(|(_, kind)| *kind == state.kind);
        let task = matches!(state.list, Some(ListKind::Task { .. }));
        let done = matches!(state.list, Some(ListKind::Task { checked: true }));
        let styles = Item::new(
            ComboBox::new()
                .element_ref(&self.styles_box)
                .items_source(STYLES.iter().map(|(name, _)| *name))
                .selected_index(style_index)
                // A quote has no entry in the box; the Quote button shows it.
                .placeholder_text(if state.kind == ParagraphKind::Quote {
                    QUOTE_STYLE.0
                } else {
                    ""
                })
                .is_enabled(on)
                .on_selection_changed(context.callback(Msg::StyleChosen))
                .width(150.0)
                .vertical_alignment(VerticalAlignment::Center)
                .tooltip("Paragraph style (Ctrl+Alt+1–6 for headings, Ctrl+Shift+N for Paragraph)"),
            150.0,
            vec![Entry::Submenu {
                label: "Styles".to_string(),
                entries: STYLES
                    .iter()
                    .map(|&(name, kind)| {
                        Entry::command(name, Command::Kind(kind), on, kind == state.kind)
                    })
                    .collect(),
            }],
        );
        let inline = |icon, label, tip, active, command| {
            Tool::new(icon, label, tip, Command::Inline(command))
                .active(active)
                .enabled(on)
        };
        vec![
            Group::new(
                "Undo",
                Shape::Stack,
                tools([
                    Tool::new(
                        Icon::Path(icons::UNDO),
                        "Undo",
                        "Undo (Ctrl+Z)",
                        Command::Undo,
                    )
                    .enabled(on),
                    Tool::new(
                        Icon::Path(icons::REDO),
                        "Redo",
                        "Redo (Ctrl+Y)",
                        Command::Redo,
                    )
                    .enabled(on),
                ]),
            ),
            Group::new("Styles", Shape::Large, vec![Control::Custom(styles)]),
            Group::new(
                "Font",
                Shape::Rows,
                tools([
                    inline(
                        Icon::Path(icons::BOLD),
                        "Bold",
                        "Bold (Ctrl+B)",
                        state.bold,
                        InlineCommand::Bold,
                    ),
                    inline(
                        Icon::Path(icons::ITALIC),
                        "Italic",
                        "Italic (Ctrl+I)",
                        state.italic,
                        InlineCommand::Italic,
                    ),
                    inline(
                        Icon::Path(icons::STRIKETHROUGH),
                        "Strikethrough",
                        "Strikethrough (Ctrl+Shift+X)",
                        state.strikethrough,
                        InlineCommand::Strikethrough,
                    ),
                    inline(
                        Icon::Path(icons::HIGHLIGHT),
                        "Highlight",
                        "Highlight (Ctrl+Shift+H)",
                        state.highlight,
                        InlineCommand::Highlight,
                    ),
                    inline(
                        Icon::Path(icons::INLINE_CODE),
                        "Inline code",
                        "Inline code (Ctrl+Alt+C)",
                        state.code,
                        InlineCommand::Code,
                    ),
                    inline(
                        Icon::Path(icons::MATH),
                        "Inline equation",
                        "Inline equation",
                        false,
                        InlineCommand::Math,
                    ),
                ]),
            ),
            Group::new(
                "Paragraph",
                Shape::Rows,
                tools([
                    Tool::new(
                        Icon::Path(icons::BULLETS),
                        "Bulleted list",
                        "Bulleted list (Ctrl+Shift+L)",
                        Command::List(ListKind::Bullet),
                    )
                    .active(state.list == Some(ListKind::Bullet))
                    .enabled(on),
                    Tool::new(
                        Icon::Path(icons::NUMBERED),
                        "Numbered list",
                        "Numbered list",
                        Command::List(ListKind::Numbered),
                    )
                    .active(state.list == Some(ListKind::Numbered))
                    .enabled(on),
                    Tool::new(
                        Icon::Path(icons::TASKS),
                        "Task list",
                        "Task list",
                        Command::List(ListKind::Task { checked: false }),
                    )
                    .active(task)
                    .enabled(on),
                    Tool::new(
                        Icon::Path(icons::MARK_DONE),
                        "Mark task done",
                        "Mark task done / not done",
                        Command::ToggleDone,
                    )
                    .active(done)
                    .enabled(on && task),
                    Tool::new(
                        Icon::Path(icons::OUTDENT),
                        "Decrease list level",
                        "Decrease list level (Shift+Tab)",
                        Command::Outdent,
                    )
                    .enabled(on),
                    Tool::new(
                        Icon::Path(icons::INDENT),
                        "Increase list level",
                        "Increase list level (Tab)",
                        Command::Indent,
                    )
                    .enabled(on),
                    // A button of its own, as in the web editor: on in a quote, and back to a
                    // paragraph when pressed there.
                    Tool::new(
                        Icon::Path(icons::QUOTE),
                        "Quote",
                        "Quote",
                        Command::Kind(if state.kind == ParagraphKind::Quote {
                            ParagraphKind::Normal
                        } else {
                            ParagraphKind::Quote
                        }),
                    )
                    .active(state.kind == ParagraphKind::Quote)
                    .enabled(on),
                ]),
            ),
            Group::new(
                "Links",
                Shape::Large,
                tools([
                    Tool::new(
                        Icon::Path(icons::LINK),
                        "Link",
                        "Link (Ctrl+K)",
                        Command::Link,
                    )
                    .enabled(on),
                    Tool::new(
                        Icon::Path(icons::PAGE_LINK),
                        "Page link",
                        "Link to a page – [[Page name]]",
                        Command::PageLink,
                    )
                    .enabled(on),
                ]),
            ),
        ]
    }

    fn insert_groups(&self) -> Vec<Group> {
        let on = self.mode == Mode::Visual;
        let insert = |icon, label, tip, what| {
            Tool::new(icon, label, tip, Command::Insert(what))
                .enabled(on)
                .labeled()
        };
        vec![
            Group::new(
                "Tables",
                Shape::Large,
                tools([insert(
                    Icon::Path(icons::TABLE),
                    "Table",
                    "Insert a table",
                    Insert::Table,
                )]),
            ),
            Group::new(
                "Media",
                Shape::Large,
                tools([
                    insert(
                        Icon::Path(icons::PICTURE),
                        "Picture",
                        "Insert a picture",
                        Insert::Image,
                    ),
                    insert(
                        Icon::Path(icons::VIDEO),
                        "Video",
                        "Insert a video",
                        Insert::Video,
                    ),
                ]),
            ),
            Group::new(
                "Blocks",
                Shape::Large,
                tools([
                    insert(
                        Icon::Path(icons::CODE_BLOCK),
                        "Code block",
                        "Insert a code block",
                        Insert::Code,
                    ),
                    insert(
                        Icon::Path(FLOWCHART),
                        "Diagram",
                        "Insert a Mermaid diagram",
                        Insert::Mermaid,
                    ),
                    insert(
                        Icon::Path(icons::MATH),
                        "Equation",
                        "Insert an equation",
                        Insert::Equation,
                    ),
                    insert(
                        Icon::Path(icons::DIVIDER),
                        "Divider",
                        "Insert a divider",
                        Insert::Divider,
                    ),
                ]),
            ),
            Group::new(
                "Links",
                Shape::Large,
                tools([
                    Tool::new(
                        Icon::Path(icons::LINK),
                        "Link",
                        "Link (Ctrl+K)",
                        Command::Link,
                    )
                    .enabled(on)
                    .labeled(),
                    Tool::new(
                        Icon::Path(icons::PAGE_LINK),
                        "Page link",
                        "Link to a page – [[Page name]]",
                        Command::PageLink,
                    )
                    .enabled(on)
                    .labeled(),
                ]),
            ),
        ]
    }

    fn view_groups(&self, context: &mut ViewContext<Self>) -> Vec<Group> {
        // Each Document Editor tool has its own button, highlighted while it is showing.
        let explorer = |icon, label, tip, tab| {
            Tool::new(icon, label, tip, Command::Explorer(tab))
                .active(self.show_navigation && self.explorer_tab == tab)
                .labeled()
        };
        let percent = (self.zoom * 100.0).round() as u16;
        // Page Width comes after the percentages, as in Word's Zoom box.
        let preset = if self.page_fit {
            Some(ZOOM_PRESETS.len())
        } else {
            ZOOM_PRESETS.iter().position(|&p| p == percent)
        };
        let zoom_box = Item::new(
            ComboBox::new()
                .items_source(
                    ZOOM_PRESETS
                        .iter()
                        .map(|p| format!("{p}%"))
                        .chain(std::iter::once(PAGE_WIDTH.to_string())),
                )
                .selected_index(preset)
                // A zoom between presets shows as the box's placeholder.
                .placeholder_text(format!("{percent}%"))
                .on_selection_changed(context.callback(Msg::ZoomPreset))
                .width(120.0)
                .vertical_alignment(VerticalAlignment::Center)
                .tooltip("Zoom"),
            120.0,
            vec![Entry::Submenu {
                label: if self.page_fit {
                    format!("Zoom ({PAGE_WIDTH})")
                } else {
                    format!("Zoom ({percent}%)")
                },
                entries: ZOOM_PRESETS
                    .iter()
                    .map(|&p| {
                        Entry::command(
                            &format!("{p}%"),
                            Command::Zoom(Zoom::To(p)),
                            true,
                            !self.page_fit && p == percent,
                        )
                    })
                    .chain(std::iter::once(Entry::command(
                        PAGE_WIDTH,
                        Command::Zoom(Zoom::PageWidth),
                        true,
                        self.page_fit,
                    )))
                    .collect(),
            }],
        );
        let theme_index = theme::THEMES
            .iter()
            .position(|&(_, choice)| choice == self.theme);
        let theme_box = Item::new(
            ComboBox::new()
                .items_source(theme::THEMES.iter().map(|&(name, _)| name))
                .selected_index(theme_index)
                .on_selection_changed(context.callback(Msg::ThemeChosen))
                .width(170.0)
                .vertical_alignment(VerticalAlignment::Center)
                .tooltip("Theme"),
            170.0,
            vec![Entry::Submenu {
                label: "Theme".to_string(),
                entries: theme::THEMES
                    .iter()
                    .map(|&(name, choice)| {
                        Entry::command(name, Command::Theme(choice), true, choice == self.theme)
                    })
                    .collect(),
            }],
        );
        vec![
            Group::new(
                "Views",
                Shape::Large,
                tools([
                    Tool::new(
                        Icon::Path(icons::VISUAL),
                        "Visual",
                        "Edit the page as it will look",
                        Command::Mode(Mode::Visual),
                    )
                    .active(self.mode == Mode::Visual)
                    .labeled(),
                    Tool::new(
                        Icon::Path(icons::SOURCE),
                        "Markdown",
                        "Edit the Markdown text",
                        Command::Mode(Mode::Source),
                    )
                    .active(self.mode == Mode::Source)
                    .labeled(),
                ]),
            ),
            Group::new(
                "Document Editor",
                Shape::Large,
                tools([
                    explorer(
                        Icon::Path(icons::NAVIGATION),
                        "Navigation",
                        "Headings: click one to go to it",
                        ExplorerTab::Headings,
                    ),
                    explorer(
                        Icon::Path(icons::FIND),
                        "Find",
                        "Find text on the page (Ctrl+F)",
                        ExplorerTab::Find,
                    ),
                    explorer(
                        Icon::Path(icons::REPLACE),
                        "Replace",
                        "Find and replace text (Ctrl+H)",
                        ExplorerTab::Replace,
                    ),
                    explorer(
                        Icon::Path(PROOFING),
                        "Spelling",
                        "Every misspelling on the page, with suggestions",
                        ExplorerTab::Spelling,
                    ),
                ]),
            ),
            Group::new(
                "Show",
                Shape::Stack,
                tools([Tool::new(
                    Icon::Path(icons::PROPERTIES),
                    "Properties",
                    "The page's properties (front matter)",
                    Command::ToggleProperties,
                )
                .active(self.show_properties)
                .labeled()]),
            ),
            Group::new(
                "Zoom",
                Shape::Large,
                vec![
                    Control::Tool(Tool::new(
                        Icon::Path(icons::ZOOM_OUT),
                        "Zoom out",
                        "Zoom out (Ctrl+-)",
                        Command::Zoom(Zoom::Out),
                    )),
                    Control::Tool(Tool::new(
                        Icon::Path(icons::ZOOM_IN),
                        "Zoom in",
                        "Zoom in (Ctrl++)",
                        Command::Zoom(Zoom::In),
                    )),
                    Control::Custom(zoom_box),
                ],
            ),
            Group::new(
                "Theme",
                Shape::Large,
                vec![
                    // Word's Switch Background: a dark page, for writing in a dark room.
                    Control::Tool(
                        Tool::new(
                            Icon::Path(DARK_THEME),
                            "Switch Background",
                            "Show the page dark, or white again. The saved page is not changed",
                            Command::SwitchBackground,
                        )
                        .active(self.dark_page)
                        .labeled(),
                    ),
                    Control::Custom(theme_box),
                ],
            ),
        ]
    }
}

/// The Help tab: Settings › About, and Microsoft's guide to knowledge base articles.
fn help_groups() -> Vec<Group> {
    vec![
        Group::new(
            "About",
            Shape::Large,
            tools([Tool::new(
                Icon::Path(icons::INFO),
                "About",
                "The version, what the app is, and where the project lives (Settings › About)",
                Command::About,
            )
            .labeled()]),
        ),
        Group::new(
            "Help",
            Shape::Large,
            tools([Tool::new(
                Icon::Path(icons::HELP),
                "Microsoft Resources",
                "Microsoft Support: create and edit a knowledge base manually (opens in your browser)",
                Command::OpenUrl(KB_HELP_URL),
            )
            .labeled()]),
        ),
    ]
}

/// The single-line ribbon: every command in one row, groups split by separators.
fn single_line(context: &mut ViewContext<Editor>, groups: Vec<Group>) -> Vec<Item> {
    let mut items = Vec::new();
    for (index, group) in groups.into_iter().enumerate() {
        if index > 0 {
            items.push(separator());
        }
        for (index, control) in group.controls.into_iter().enumerate() {
            items.push(match control {
                Control::Tool(tool) if tool.labeled => inline_tool(context, tool),
                Control::Tool(tool) => icon_tool(context, tool, 34.0),
                Control::Custom(item) if index > 0 => after_button(item),
                Control::Custom(item) => item,
            });
        }
    }
    items
}

/// The Classic ribbon: each group is one item, so a group moves to the "More" menu whole.
fn classic(context: &mut ViewContext<Editor>, groups: Vec<Group>) -> Vec<Item> {
    let mut items = Vec::new();
    for (index, group) in groups.into_iter().enumerate() {
        if index > 0 {
            items.push(group_separator());
        }
        items.push(classic_group(context, group));
    }
    items
}

fn classic_group(context: &mut ViewContext<Editor>, group: Group) -> Item {
    let mut menu = Vec::new();
    let mut views = Vec::new();
    let mut width = 0.0;
    match group.shape {
        Shape::Large => {
            for (index, control) in group.controls.into_iter().enumerate() {
                let item = match control {
                    Control::Tool(tool) => large_tool(context, tool),
                    Control::Custom(item) if index > 0 => after_button(item),
                    Control::Custom(item) => item,
                };
                width += item.width + ITEM_SPACING;
                menu.extend(item.menu);
                views.push(item.view);
            }
            views = vec![row_of(views)];
        }
        Shape::Rows => {
            let controls: Vec<Item> = group
                .controls
                .into_iter()
                .map(|control| match control {
                    Control::Tool(tool) => icon_tool(context, tool, 32.0),
                    Control::Custom(item) => item,
                })
                .collect();
            let per_row = controls.len().div_ceil(2);
            let mut rows: Vec<Vec<View>> = vec![Vec::new(), Vec::new()];
            let mut row_widths = [0.0, 0.0];
            for (index, item) in controls.into_iter().enumerate() {
                let row = usize::from(index >= per_row);
                row_widths[row] += item.width + ITEM_SPACING;
                menu.extend(item.menu);
                rows[row].push(item.view);
            }
            width = f64::max(row_widths[0], row_widths[1]);
            views = rows.into_iter().map(row_of).collect();
        }
        Shape::Stack => {
            for control in group.controls {
                let item = match control {
                    Control::Tool(tool) => stacked_tool(context, tool),
                    Control::Custom(item) => item,
                };
                width = f64::max(width, item.width);
                menu.extend(item.menu);
                views.push(item.view);
            }
        }
    }
    let width = f64::max(width, text_width(group.name, 11.0)) + 8.0;
    let content = StackPanel::new()
        .spacing(2.0)
        .vertical_alignment(VerticalAlignment::Center)
        .keyed_children(
            views
                .into_iter()
                .enumerate()
                .map(|(index, view)| (index as u64, view))
                .collect::<Vec<_>>(),
        );
    let view = StackPanel::new()
        .spacing(2.0)
        .margin(Thickness::xy(4.0, 0.0))
        .children((
            Grid::new().height(CLASSIC_HEIGHT).children((content,)),
            TextBlock::new()
                .text(group.name)
                .font_size(11.0)
                .foreground(MUTED)
                .horizontal_alignment(HorizontalAlignment::Center),
        ));
    Item::new(view, width, menu)
}

fn row_of(views: Vec<View>) -> View {
    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(ITEM_SPACING)
        .keyed_children(
            views
                .into_iter()
                .enumerate()
                .map(|(index, view)| (index as u64, view))
                .collect::<Vec<_>>(),
        )
}

/// A ribbon button, highlighted while its formatting is on at the caret.
fn tool_button(context: &mut ViewContext<Editor>, tool: &Tool) -> Button {
    let button = Button::new()
        .is_enabled(tool.enabled)
        .on_click(context.message(Msg::Command(tool.command)));
    if tool.active {
        button.resource_overrides(active_resources())
    } else {
        button.style(ButtonStyle::Subtle)
    }
}

/// A small icon-only button.
fn icon_tool(context: &mut ViewContext<Editor>, tool: Tool, height: f64) -> Item {
    let view = tool_button(context, &tool)
        .min_width(36.0)
        .height(height)
        .content(tool.icon.view())
        .tooltip(tool.tip);
    Item::new(view, 44.0, vec![tool.entry()])
}

/// A small button with its icon and name side by side, for the single-line ribbon.
fn inline_tool(context: &mut ViewContext<Editor>, tool: Tool) -> Item {
    let width = 24.0 + 20.0 + 8.0 + text_width(tool.label, 14.0);
    let view = tool_button(context, &tool)
        .height(34.0)
        .content(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((tool.icon.view(), TextBlock::new().text(tool.label))),
        )
        .tooltip(tool.tip);
    Item::new(view, width, vec![tool.entry()])
}

/// A small button with icon and name, stacked under others in a Classic group.
fn stacked_tool(context: &mut ViewContext<Editor>, tool: Tool) -> Item {
    let width = 24.0 + 20.0 + 8.0 + text_width(tool.label, 13.0);
    let view = tool_button(context, &tool)
        .height(32.0)
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .horizontal_content_alignment(HorizontalAlignment::Left)
        .content(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(8.0)
                .children((
                    tool.icon.view(),
                    TextBlock::new().text(tool.label).font_size(13.0),
                )),
        )
        .tooltip(tool.tip);
    Item::new(view, width, vec![tool.entry()])
}

/// A large button, icon above name, as in the Classic ribbon. A long two-word name goes on two
/// lines, as Word does ("Markdown" over "source").
fn large_tool(context: &mut ViewContext<Editor>, tool: Tool) -> Item {
    let lines = large_label_lines(tool.label);
    let widest = lines
        .iter()
        .map(|line| text_width(line, 12.0))
        .fold(0.0, f64::max);
    let width = f64::max(56.0, 24.0 + widest);
    // Each line is its own centered TextBlock: Reactor's TextBlock has no text alignment, so
    // wrapped lines would sit to the left.
    let label = StackPanel::new()
        .horizontal_alignment(HorizontalAlignment::Center)
        .keyed_children(
            lines
                .into_iter()
                .enumerate()
                .map(|(index, line)| {
                    (
                        index as u64,
                        TextBlock::new()
                            .text(line)
                            .font_size(12.0)
                            .horizontal_alignment(HorizontalAlignment::Center),
                    )
                })
                .collect::<Vec<_>>(),
        );
    let view = tool_button(context, &tool)
        .min_width(56.0)
        .height(CLASSIC_HEIGHT)
        .content(
            StackPanel::new()
                .spacing(4.0)
                .horizontal_alignment(HorizontalAlignment::Center)
                .children((tool.icon.view(), label)),
        )
        .tooltip(tool.tip);
    Item::new(view, width, vec![tool.entry()])
}

/// A large button's name, split onto two lines when one line would make the button wide.
fn large_label_lines(label: &str) -> Vec<&str> {
    const ONE_LINE: f64 = 80.0;
    match label.rsplit_once(' ') {
        Some((first, last)) if text_width(label, 12.0) > ONE_LINE => vec![first, last],
        _ => vec![label],
    }
}

/// The divider between Classic ribbon groups, the height of the groups.
/// Room before a box that follows a button in a Classic group: the separator's margin plus
/// the group's own padding, so the gaps either side of a highlighted button match.
const CUSTOM_GAP: f64 = 8.0;

/// A box that follows a button in its group, with room before it, so a highlighted button
/// does not touch it: as much room as the group's edge leaves on the button's other side.
fn after_button(item: Item) -> Item {
    Item {
        view: Border::new()
            .padding(Thickness::new(CUSTOM_GAP, 0.0, 0.0, 0.0))
            .content(item.view),
        width: item.width + CUSTOM_GAP,
        ..item
    }
}

fn group_separator() -> Item {
    Item {
        view: Border::new()
            .background(DIVIDER)
            .width(1.0)
            .height(CLASSIC_HEIGHT + 12.0)
            .margin(Thickness::xy(4.0, 0.0))
            .vertical_alignment(VerticalAlignment::Center)
            .into(),
        width: 9.0,
        separator: true,
        menu: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_large_button_names_take_two_lines() {
        assert_eq!(large_label_lines("Markdown source"), ["Markdown", "source"]);
        assert_eq!(large_label_lines("Mermaid diagram"), ["Mermaid", "diagram"]);
        assert_eq!(large_label_lines("Page link"), ["Page link"]);
        assert_eq!(large_label_lines("Table"), ["Table"]);
    }
}
