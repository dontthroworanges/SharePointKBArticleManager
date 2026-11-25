//! The window layout: a Word-style ribbon, the page, the Properties pane, and the status bar.

use super::*;

pub(super) mod icons;
mod ribbon;
mod settings_page;

// The window around the page follows the light or dark theme through WinUI's theme brushes.
// Mica (the window backdrop) shows through the ribbon, panes and status bar; the area around
// the page is a layer over it, as WinUI apps lay content on Mica.
const CANVAS: ThemeBrush = ThemeBrush::Layer;
/// Where the app's name starts: in line with the File tab's label below it.
const TITLE_INSET: f64 = 20.0;
/// Where the app's name starts while the title bar shows its back button.
const BACK_BUTTON_INSET: f64 = 52.0;
/// Room either side of the centered document name: enough for the app's name and version on
/// the left, and more than the window buttons on the right.
const CENTERED_TITLE_MARGIN: f64 = 240.0;
const DIVIDER: ThemeBrush = ThemeBrush::Divider;
const MUTED: ThemeBrush = ThemeBrush::SecondaryText;
/// The Document Editor's width limits; the user sets the width by dragging its edge.
const NAVIGATION_MIN: f64 = 200.0;
const NAVIGATION_MAX: f64 = 640.0;
/// Room the page keeps beside a wide Document Editor.
const PAGE_MIN_ROOM: f64 = 420.0;
const PROPERTIES_WIDTH: f64 = 320.0;

impl Editor {
    pub(super) fn render(&self, context: &mut ViewContext<Self>) -> View {
        theme::set_current(self.palette());
        context.window_title(self.title());
        let visuals = WindowVisuals::new()
            .theme(match self.theme {
                theme::ThemeChoice::System => WindowTheme::System,
                theme::ThemeChoice::Light => WindowTheme::Light,
                theme::ThemeChoice::Dark => WindowTheme::Dark,
            })
            .backdrop(WindowBackdrop::Mica)
            .client_size(1280.0, 860.0);
        // Without the file (no APPDATA, or it cannot be written) the window keeps WinUI's icon.
        context.window_visuals(match window_icon_path() {
            Some(path) => visuals.icon(path),
            None => visuals,
        });
        context.on_window_size(context.callback(Msg::WindowSize));
        context.on_color_scheme(context.callback(Msg::ColorScheme));
        // The page starts once its WebView2 is on screen.
        let sender = context.sender();
        context.use_effect("page", (), move || {
            sender.send(Msg::PageInit);
            None
        });
        // The Styles box is (re)built when the ribbon is: style its list as soon as it exists,
        // not at the first edit.
        let (styles_box, hooked) = (self.styles_box.clone(), Rc::clone(&self.styles_box_hooked));
        context.use_effect(
            "styles-box",
            (
                self.tab,
                self.ribbon_layout,
                self.mode,
                theme::current().dark,
                self.window_width.round() as i64,
            ),
            move || {
                hook_styles_box(&styles_box, &hooked);
                None
            },
        );
        // Navigation pane | page | Properties pane, as in Word.
        let body = Grid::new()
            .columns([
                GridLength::Pixel(if self.show_navigation {
                    self.explorer_width()
                } else {
                    0.0
                }),
                GridLength::STAR,
                GridLength::Pixel(if self.show_properties {
                    PROPERTIES_WIDTH
                } else {
                    0.0
                }),
            ])
            .children((
                if self.show_navigation {
                    self.resizable_navigation(context).in_column(0)
                } else {
                    View::empty()
                },
                self.page().in_column(1),
                if self.show_properties {
                    self.properties_pane(context).in_column(2)
                } else {
                    View::empty()
                },
            ));

        let (title, tabs) = self.title_band(context);
        // Settings covers everything below the title bar, as in Notepad. The page stays under
        // it, so its editors keep their content and undo history.
        let settings = match self.settings {
            Some(category) => Border::new()
                .grid_row(1)
                .grid_row_span(5)
                .background(ThemeBrush::SolidBackground)
                .content(self.settings_page(category, context)),
            None => View::empty(),
        };
        Grid::new()
            .element_ref(&self.root)
            .rows([
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::Auto,
                GridLength::STAR,
                GridLength::Auto,
            ])
            .children((
                title.in_row(0),
                tabs.in_row(1),
                self.ribbon(context).in_row(2),
                // Problems show here, above the page, until closed. Always there, opened and
                // closed, so the layout below does not change shape.
                InfoBar::new()
                    .severity(InfoBarSeverity::Warning)
                    .message(self.problem.clone().unwrap_or_default())
                    .is_open(self.problem.is_some())
                    .is_closable(true)
                    .on_closed(context.message(Msg::DismissProblem))
                    .grid_row(3),
                body.in_row(4),
                self.status_bar(context).in_row(5),
                settings,
                self.dialog(context),
            ))
    }

    /// The window title, with the File menu and the ribbon tabs below it, on the Mica backdrop
    /// like the rest of the window. The selected tab is underlined in the accent color.
    fn title_band(&self, context: &mut ViewContext<Self>) -> (View, View) {
        let palette = theme::current();
        // The app's name and version sit at the left, lined up with the File tab below; the
        // document's name is centered. Both are laid over the whole width, not put in the
        // title bar's content area, which stops at the window buttons: centered there, the
        // name would sit left of center. Equal margins keep the centered name clear of the
        // app's name on the left and the window buttons on the right.
        let app = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(6.0)
            // Right of the back button while Settings shows.
            .margin(Thickness::new(
                if self.settings.is_some() {
                    BACK_BUTTON_INSET
                } else {
                    TITLE_INSET
                },
                0.0,
                0.0,
                0.0,
            ))
            .horizontal_alignment(HorizontalAlignment::Left)
            .vertical_alignment(VerticalAlignment::Center)
            .children((
                Image::new()
                    .source_data(EncodedImage::from_static(APP_ICON_SMALL))
                    .width(16.0)
                    .height(16.0)
                    .margin(Thickness::new(0.0, 0.0, 4.0, 0.0)),
                TextBlock::new()
                    .text(APP_NAME)
                    .foreground(palette.on_band)
                    .font_size(12.0),
                TextBlock::new()
                    .text(APP_VERSION)
                    .foreground(MUTED)
                    .font_size(12.0),
            ));
        let title = Grid::new().background(palette.band_fill).children((
            // Settings has a back button at the top left, as in Notepad.
            TitleBar::new()
                .is_back_button_visible(self.settings.is_some())
                .on_back_requested(context.message(Msg::CloseSettings)),
            app,
            TextBlock::new()
                .text(self.document_title())
                .foreground(palette.on_band)
                .font_size(12.0)
                .text_trimming(TextTrimming::CharacterEllipsis)
                .max_lines(1)
                .margin(Thickness::xy(CENTERED_TITLE_MARGIN, 0.0))
                .horizontal_alignment(HorizontalAlignment::Center)
                .vertical_alignment(VerticalAlignment::Center),
        ));
        let tabs = [
            ("File", Tab::File),
            ("Home", Tab::Home),
            ("Insert", Tab::Insert),
            ("View", Tab::View),
            ("Help", Tab::Help),
        ]
        .into_iter()
        .map(|(label, tab)| {
            let selected = self.tab == tab;
            let button = Button::new()
                .on_click(context.message(Msg::Tab(Some(label.to_string()))))
                .resource_overrides(if selected {
                    band_resources(Some(palette.selected_tab), false)
                } else {
                    band_resources(None, true)
                })
                .content(tab_label(label, selected));
            // Keyed by selection too: a tab that becomes selected gets a fresh button, since a
            // button keeps the pressed look it had when its colors change under the pointer.
            (
                format!("{label}-{selected}-{}", theme::current().dark),
                button,
                selected,
            )
        })
        .collect::<Vec<_>>();
        // Each cell paints the band's fill (none at present, so Mica shows), except the
        // selected tab's, which is always left clear.
        let cell = |column: usize, content: View, clear: bool| -> View {
            let cell = Border::new()
                .grid_column(column as i32)
                // Even on both sides, so a selected tab's label sits in the middle of the tab.
                .padding(Thickness::new(1.0, 2.0, 1.0, 0.0));
            let cell = if clear {
                cell
            } else {
                cell.background(palette.band_fill)
            };
            cell.content(content)
        };
        let mut cells = vec![
            ("start".to_string(), cell(0, View::empty(), false)),
        ];
        for (index, (key, button, selected)) in tabs.into_iter().enumerate() {
            cells.push((key, cell(index + 1, button, selected)));
        }
        let rest = cells.len();
        cells.push(("rest".to_string(), cell(rest, View::empty(), false)));
        // The Settings gear, held at the right edge of the window above the ribbon's layout
        // chevron: the same width, and the same margin from the edge as the ribbon's padding.
        let gear = Button::new()
            .resource_overrides(band_resources(None, true))
            .on_click(context.message(Msg::OpenSettings))
            .width(40.0)
            .min_width(0.0)
            .margin(Thickness::new(0.0, 0.0, 8.0, 0.0))
            .horizontal_alignment(HorizontalAlignment::Right)
            .vertical_alignment(VerticalAlignment::Stretch)
            .content(path_icon(icons::SETTINGS, 18.0))
            .tooltip("Settings");
        cells.push((
            format!("gear-{}", theme::current().dark),
            cell(rest + 1, gear.into(), false),
        ));
        let row = Grid::new()
            .columns(
                std::iter::once(GridLength::Pixel(8.0))
                    .chain(std::iter::repeat_n(GridLength::Auto, rest - 1))
                    .chain([GridLength::STAR, GridLength::Auto]),
            )
            .keyed_children(cells);
        (title.into(), row.into())
    }

    fn ribbon(&self, context: &mut ViewContext<Self>) -> View {
        let items = self.ribbon_items(context);
        // Commands that do not fit the window move to a "More" menu at the end of the row.
        let (shown, hidden) = fit(items, self.window_width - 16.0 - ribbon::CHEVRON_WIDTH);
        // The button is always there, shrunk to nothing when every command fits: Reactor cannot
        // add or remove a view that carries a menu.
        let overflowing = !hidden.is_empty();
        let (entries, commands) = overflow_menu(hidden);
        // The menu reports a click by label; the update looks the command up here.
        *self.overflow_commands.borrow_mut() = commands;
        let more = Button::new()
            .style(ButtonStyle::Subtle)
            .is_enabled(overflowing)
            .min_width(0.0)
            .width(if overflowing { 44.0 } else { 0.0 })
            .height(34.0)
            .margin(Thickness::new(
                if overflowing { 8.0 } else { 0.0 },
                0.0,
                0.0,
                0.0,
            ))
            // Reactor crashes on a tooltip around a view with a menu, so the icon carries it.
            .content(path_icon(icons::MORE, 20.0).tooltip("More commands"))
            .menu(Menu::new(entries, context.callback(Msg::MoreMenu)));
        let commands = Grid::new()
            .columns([GridLength::STAR, GridLength::Auto, GridLength::Auto])
            .children((
                row(shown).in_column(0),
                more.in_column(1),
                self.layout_chevron(context).in_column(2),
            ));
        Border::new()
            .border_brush(DIVIDER)
            .border_thickness(Thickness::new(0.0, 0.0, 0.0, 1.0))
            .padding(Thickness::new(8.0, 6.0, 8.0, 6.0))
            .content(commands)
    }

    /// The page: the editor in a WebView2 (see `page`), on the canvas around it. The page
    /// lays itself out, zoomed, and scrolls itself; its background is clear, so the canvas
    /// shows around it.
    fn page(&self) -> View {
        Border::new()
            .background(CANVAS)
            .content(WebView2::new().element_ref(&self.web))
    }

    fn properties_pane(&self, context: &mut ViewContext<Self>) -> View {
        let source_mode = self.mode == Mode::Source;
        let body: View = match &self.properties {
            _ if source_mode => caption(
                "In the Markdown source view, edit properties in the front matter between the --- lines at the top. Switch back to Visual to edit them here.",
            ),
            Properties::Raw(yaml) => TextBox::new()
                .text(yaml.clone())
                .accepts_return(true)
                .text_wrapping(TextWrapping::Wrap)
                .min_height(160.0)
                .on_text_changed(context.callback(Msg::RawProperties))
                .slot(TextBoxSlot::Header, "YAML"),
            Properties::None => caption("This page has no properties."),
            Properties::Entries(entries) => {
                StackPanel::new().spacing(6.0).keyed_children(
                    entries
                        .iter()
                        .enumerate()
                        .map(|(index, entry)| {
                            (
                                index as u64,
                                Grid::new()
                                    .columns([GridLength::STAR, GridLength::STAR, GridLength::Auto])
                                    .column_spacing(4.0)
                                    .children((
                                        TextBox::new()
                                            .text(entry.key.clone())
                                            .placeholder_text("Name")
                                            .on_text_changed(context.callback(move |text| {
                                                Msg::PropertyKey(index, text)
                                            }))
                                            .grid_column(0),
                                        TextBox::new()
                                            .text(entry.value.clone())
                                            .placeholder_text("Value")
                                            .on_text_changed(context.callback(move |text| {
                                                Msg::PropertyValue(index, text)
                                            }))
                                            .grid_column(1),
                                        Button::new()
                                            .style(ButtonStyle::Subtle)
                                            .on_click(context.message(Msg::RemoveProperty(index)))
                                            .content(symbol(Symbol::Delete))
                                            .tooltip("Remove property")
                                            .in_column(2),
                                    )),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
            }
        };
        let add = if source_mode || matches!(self.properties, Properties::Raw(_)) {
            View::empty()
        } else {
            Button::new()
                .on_click(context.message(Msg::AddProperty))
                .content("Add property")
        };
        Border::new()
            .border_brush(DIVIDER)
            .border_thickness(Thickness::new(1.0, 0.0, 0.0, 0.0))
            .padding(Thickness::uniform(16.0))
            .content(
                StackPanel::new().spacing(10.0).children((
                    // As the Document Editor's header: the title, and a close button.
                    Grid::new()
                        .columns([GridLength::STAR, GridLength::Auto])
                        .children((
                            TextBlock::new()
                                .text("Page properties")
                                .font_size(18.0)
                                .font_weight(FontWeight::SEMI_BOLD)
                                .vertical_alignment(VerticalAlignment::Center)
                                .grid_column(0),
                            Button::new()
                                .style(ButtonStyle::Subtle)
                                .on_click(context.message(Msg::Command(Command::ToggleProperties)))
                                .content(symbol(Symbol::Cancel))
                                .tooltip("Close page properties")
                                .in_column(1),
                        )),
                    caption("Saved as YAML front matter at the top of the file."),
                    body,
                    add,
                )),
            )
    }

    fn dialog(&self, context: &mut ViewContext<Self>) -> View {
        let Some(dialog) = &self.dialog else {
            return View::empty();
        };
        let field = |context: &mut ViewContext<Self>,
                     index: usize,
                     header: &str,
                     placeholder: &str|
         -> View {
            TextBox::new()
                .text(dialog.fields.get(index).cloned().unwrap_or_default())
                .placeholder_text(placeholder)
                .on_text_changed(context.callback(move |text| Msg::DialogField(index, text)))
                .slot(TextBoxSlot::Header, header)
        };
        let folder = self
            .path
            .as_ref()
            .and_then(|path| path.parent())
            .map(|folder| folder.display().to_string())
            .unwrap_or_default();
        let (title, primary, content): (&str, &str, View) = match &dialog.kind {
            DialogKind::Rename => (
                "Rename file",
                "Rename",
                StackPanel::new().spacing(12.0).width(420.0).children((
                    field(context, 0, "File name", "New name"),
                    TextBlock::new()
                        .text(format!(
                            "The file stays in {folder}. Without an extension, it keeps its own."
                        ))
                        .foreground(MUTED)
                        .font_size(12.0)
                        .text_wrapping(TextWrapping::Wrap),
                )),
            ),
            DialogKind::Link => (
                "Insert link",
                "Insert",
                StackPanel::new().spacing(12.0).width(420.0).children((
                    field(context, 0, "Address", "https://…"),
                    field(
                        context,
                        1,
                        "Text to display",
                        "Uses the selected text when empty",
                    ),
                )),
            ),
            DialogKind::PageLink => (
                "Link to a page",
                "Insert",
                StackPanel::new().spacing(12.0).width(420.0).children((
                    field(
                        context,
                        0,
                        "Page name",
                        "Exact name of the Knowledge Base page",
                    ),
                    field(context, 1, "Text to display (optional)", ""),
                )),
            ),
            DialogKind::Image | DialogKind::Video | DialogKind::EditMedia(_) => (
                match dialog.kind {
                    DialogKind::Image => "Insert picture",
                    DialogKind::Video => "Insert video",
                    _ => "Picture or video",
                },
                if matches!(dialog.kind, DialogKind::EditMedia(_)) {
                    "Save"
                } else {
                    "Insert"
                },
                StackPanel::new().spacing(12.0).width(420.0).children((
                    field(context, 0, "Address", "https://contoso.sharepoint.com/…"),
                    field(
                        context,
                        1,
                        "Alt text",
                        "Describe it for people who can't see it",
                    ),
                    caption(
                        "SharePoint does not copy local files – use an address readers can open.",
                    ),
                )),
            ),
            DialogKind::Discard(_) => (
                "Save changes?",
                "Save",
                TextBlock::new()
                    .text("This page has changes that are not saved.")
                    .into(),
            ),
        };
        let mut view = ContentDialog::new()
            .is_open(true)
            .title(title)
            .primary_button_text(primary)
            .close_button_text("Cancel")
            .on_closed(context.callback(Msg::DialogClosed));
        if matches!(dialog.kind, DialogKind::Discard(_)) {
            view = view.secondary_button_text("Don't save");
        }
        view.content(content)
    }

    fn status_bar(&self, context: &mut ViewContext<Self>) -> View {
        Border::new()
            .border_brush(DIVIDER)
            .border_thickness(Thickness::new(0.0, 1.0, 0.0, 0.0))
            .padding(Thickness::xy(12.0, 4.0))
            .content(
                Grid::new()
                    .columns([GridLength::STAR, GridLength::Auto, GridLength::Auto])
                    .column_spacing(24.0)
                    .vertical_alignment(VerticalAlignment::Center)
                    .children((
                        // The left of the bar stays empty: no status messages, as in the web
                        // editor.
                        View::empty().in_column(0),
                        self.view_switch(context).in_column(1),
                        self.zoom_slider(context).in_column(2),
                    )),
            )
    }

    /// Word's view buttons beside the zoom: the visual page or the Markdown source, with the
    /// one showing highlighted.
    fn view_switch(&self, context: &mut ViewContext<Self>) -> View {
        let button = |context: &mut ViewContext<Self>, mode: Mode, icon: View, tip: &str| {
            let selected = self.mode == mode;
            // Small, as in the web editor: just room for a 16-pixel icon inside the button's
            // own padding (11 pixels a side, 5 above and 6 below), which clips what is larger.
            let button = Button::new()
                .min_width(0.0)
                .min_height(0.0)
                .width(40.0)
                .height(30.0)
                .on_click(context.message(Msg::Command(Command::Mode(mode))));
            let button = if selected {
                button.resource_overrides(active_resources())
            } else {
                button.style(ButtonStyle::Subtle)
            };
            let button = button.content(icon.tooltip(tip));
            // Keyed by selection and theme: a live button keeps its first colors.
            (
                format!("{mode:?}-{selected}-{}", theme::current().dark),
                button,
            )
        };
        let visual = button(context, Mode::Visual, path_icon(icons::VISUAL, 16.0), "Visual");
        let source = button(context, Mode::Source, path_icon(icons::SOURCE, 16.0), "Markdown");
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(2.0)
            .vertical_alignment(VerticalAlignment::Center)
            .keyed_children(vec![visual, source])
    }

    /// Word's status bar zoom: −, a slider with 100% in the middle, +, and the percentage
    /// (click it to return to 100%).
    fn zoom_slider(&self, context: &mut ViewContext<Self>) -> View {
        let small = |context: &mut ViewContext<Self>, text: &str, tip: &str, command: Command| {
            Button::new()
                .style(ButtonStyle::Subtle)
                .on_click(context.message(Msg::Command(command)))
                .content(TextBlock::new().text(text).foreground(MUTED))
                .tooltip(tip)
        };
        let percent = (self.zoom * 100.0).round() as u16;
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(2.0)
            .vertical_alignment(VerticalAlignment::Center)
            .children((
                small(
                    context,
                    "\u{2212}",
                    "Zoom out (Ctrl+-)",
                    Command::Zoom(Zoom::Out),
                ),
                Slider::new()
                    .minimum(0.0)
                    .maximum(100.0)
                    .step_frequency(0.5)
                    .value(zoom_to_slider(self.zoom))
                    .on_value_changed(context.callback(Msg::ZoomSlider))
                    .width(140.0)
                    .vertical_alignment(VerticalAlignment::Center),
                small(context, "+", "Zoom in (Ctrl++)", Command::Zoom(Zoom::In)),
                small(
                    context,
                    &format!("{percent}%"),
                    "Zoom to 100% (Ctrl+0)",
                    Command::Zoom(Zoom::To(100)),
                ),
            ))
    }
}

/// A ribbon command with the width it takes, so the ribbon can tell what fits, and how it
/// reads as an entry in the "More" menu when it does not.
pub(super) struct Item {
    view: View,
    width: f64,
    /// A group separator: hidden at the edges of a row, and a separator in the overflow menu.
    separator: bool,
    menu: Vec<Entry>,
}

/// One line of the "More" menu.
enum Entry {
    Command {
        label: String,
        command: Command,
        enabled: bool,
        active: bool,
    },
    Submenu {
        label: String,
        entries: Vec<Entry>,
    },
}

impl Entry {
    fn command(label: &str, command: Command, enabled: bool, active: bool) -> Self {
        Self::Command {
            label: label.to_string(),
            command,
            enabled,
            active,
        }
    }
}

impl Item {
    fn new(view: impl Into<View>, width: f64, menu: Vec<Entry>) -> Self {
        Self {
            view: view.into(),
            width,
            separator: false,
            menu,
        }
    }
}

/// Space between ribbon items.
const ITEM_SPACING: f64 = 2.0;
/// The ribbon's "More" button, with its margin.
const MORE_WIDTH: f64 = 52.0;

/// Splits a tab's commands into those that fit in `available` pixels and those that go in the
/// overflow menu, the way Word's simplified ribbon does as the window narrows.
pub(super) fn fit(items: Vec<Item>, available: f64) -> (Vec<Item>, Vec<Item>) {
    let total: f64 = items.iter().map(|item| item.width + ITEM_SPACING).sum();
    if total <= available {
        return (items, Vec::new());
    }
    let room = available - MORE_WIDTH;
    let mut used = 0.0;
    let mut shown = Vec::new();
    let mut rest = items.into_iter().peekable();
    while let Some(item) = rest.next_if(|item| used + item.width + ITEM_SPACING <= room) {
        used += item.width + ITEM_SPACING;
        shown.push(item);
    }
    while shown.last().is_some_and(|item| item.separator) {
        shown.pop();
    }
    let hidden = rest.skip_while(|item| item.separator).collect();
    (shown, hidden)
}

/// `width` for the Document Editor, within its limits and leaving the page room in a window
/// `window` wide.
pub(super) fn navigation_width(width: f64, window: f64) -> f64 {
    let max = (window - PAGE_MIN_ROOM).clamp(NAVIGATION_MIN, NAVIGATION_MAX);
    width.clamp(NAVIGATION_MIN, max)
}

/// Gives `element` the left-right resize cursor.
fn resize_cursor(element: &windows_core::IUnknown) -> windows_core::Result<()> {
    use crate::winui as native;
    use windows_core::Interface;
    let cursor = native::InputSystemCursor::Create(native::InputSystemCursorShape::SizeWestEast)?;
    element
        .cast::<native::IUIElementProtected>()?
        .SetProtectedCursor(&cursor)
}

/// A list key that changes with the theme. Buttons colored through resource overrides keep
/// their old colors when the overrides change with the theme, so they are rebuilt instead.
fn themed_key(index: usize) -> u64 {
    (index as u64) << 1 | u64::from(theme::current().dark)
}

fn row(items: Vec<Item>) -> View {
    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(ITEM_SPACING)
        .keyed_children(
            items
                .into_iter()
                .enumerate()
                .map(|(index, item)| (themed_key(index), item.view))
                .collect::<Vec<_>>(),
        )
}

/// The "More" menu for the commands that do not fit, and the command behind each label.
/// Active commands are ticked, as the ribbon highlights them.
fn overflow_menu(items: Vec<Item>) -> (Vec<MenuItem>, Vec<(String, Command)>) {
    fn convert(
        entries: Vec<Entry>,
        menu: &mut Vec<MenuItem>,
        commands: &mut Vec<(String, Command)>,
    ) {
        for entry in entries {
            match entry {
                Entry::Command {
                    label,
                    command,
                    enabled,
                    active,
                } => {
                    let label = if active {
                        format!("\u{2713}  {label}")
                    } else {
                        label
                    };
                    let key = format!("more-{}", commands.len());
                    menu.push(if enabled {
                        MenuItem::item(key, label.clone())
                    } else {
                        MenuItem::disabled(key, label.clone())
                    });
                    commands.push((label, command));
                }
                Entry::Submenu { label, entries } => {
                    let mut items = Vec::new();
                    convert(entries, &mut items, commands);
                    menu.push(MenuItem::submenu(format!("more-{label}"), label, items));
                }
            }
        }
    }
    let mut menu = Vec::new();
    let mut commands = Vec::new();
    for (index, item) in items.into_iter().enumerate() {
        if item.separator {
            menu.push(MenuItem::separator(format!("more-separator-{index}")));
        } else {
            convert(item.menu, &mut menu, &mut commands);
        }
    }
    (menu, commands)
}

fn separator() -> Item {
    Item {
        view: Border::new()
            .background(DIVIDER)
            .width(1.0)
            .height(24.0)
            .margin(Thickness::xy(6.0, 0.0))
            .vertical_alignment(VerticalAlignment::Center)
            .into(),
        width: 13.0,
        separator: true,
        menu: Vec::new(),
    }
}

/// An approximate rendered width of `text` in Segoe UI at `size` pixels.
fn text_width(text: &str, size: f64) -> f64 {
    text.chars().count() as f64 * size * 0.55
}

fn symbol(symbol: Symbol) -> View {
    SymbolIcon::new().symbol(symbol).into()
}

fn glyph(glyph: &str) -> View {
    FontIcon::new().glyph(glyph).into()
}

fn caption(text: &str) -> View {
    TextBlock::new()
        .text(text)
        .foreground(MUTED)
        .text_wrapping(TextWrapping::Wrap)
        .vertical_alignment(VerticalAlignment::Center)
        .into()
}

fn active_resources() -> ResourceOverrides {
    let palette = theme::current();
    ResourceOverrides::new()
        .set("ButtonBackground", palette.active)
        .set("ButtonBackgroundPointerOver", palette.active)
        .set("ButtonBorderBrush", palette.active_border)
        .set("ButtonBorderBrushPointerOver", palette.active_border)
}

/// A button that sits on the title band: transparent, with the band's text color, and a lighter
/// or darker band color under the pointer. `foreground` overrides the text color.
fn band_resources(foreground: Option<Color>, hover: bool) -> ResourceOverrides {
    let palette = theme::current();
    let text = foreground.unwrap_or(palette.on_band);
    let (over, pressed) = if hover {
        (palette.band_hover, palette.band_pressed)
    } else {
        (Color::transparent(), Color::transparent())
    };
    ResourceOverrides::new()
        .set("ButtonBackground", Color::transparent())
        .set("ButtonBackgroundPointerOver", over)
        .set("ButtonBackgroundPressed", pressed)
        .set("ButtonBorderBrush", Color::transparent())
        .set("ButtonBorderBrushPointerOver", Color::transparent())
        .set("ButtonBorderBrushPressed", Color::transparent())
        .set("ButtonForeground", text)
        .set("ButtonForegroundPointerOver", text)
        .set("ButtonForegroundPressed", text)
}

/// Grid placement for finished views, which Reactor only offers on concrete controls.
trait Place {
    fn in_column(self, column: i32) -> View;
    fn in_row(self, row: i32) -> View;
}

impl Place for View {
    fn in_column(self, column: i32) -> View {
        Border::new().grid_column(column).content(self)
    }

    fn in_row(self, row: i32) -> View {
        Border::new().grid_row(row).content(self)
    }
}

/// A Fluent icon from its path markup, at a fixed size so it centers over a wider label
/// instead of stretching to it and drawing from the left.
fn path_icon(data: &str, size: f64) -> View {
    // The path is drawn for 20 pixels; PathIcon crops rather than scales it, so a Viewbox
    // scales it to other sizes.
    let icon = PathIcon::new().data(data).width(20.0).height(20.0);
    if size == 20.0 {
        return icon.into();
    }
    Viewbox::new()
        .width(size)
        .height(size)
        .slot(ViewboxSlot::Child, icon)
}

impl Editor {
    /// The Document Editor's width, kept within its limits for the window's width.
    fn explorer_width(&self) -> f64 {
        navigation_width(self.navigation_width, self.window_width)
    }

    /// The Document Editor with a drag handle on its right edge, as in Word, to set its
    /// width.
    fn resizable_navigation(&self, context: &mut ViewContext<Self>) -> View {
        let dragging = self.resizing.is_some();
        let handle = Border::new()
            // A drawn background, even a clear one, is what makes the handle take the pointer.
            .background(if dragging {
                theme::current().band
            } else {
                Color::transparent()
            })
            .capture_pointer_on_press(true)
            .on_pointer_pressed(context.callback(Msg::SplitterPressed))
            .on_pointer_moved(context.callback(Msg::SplitterMoved))
            .on_pointer_released(context.callback(|_| Msg::SplitterReleased))
            .on_pointer_capture_lost(context.message(Msg::SplitterReleased));
        let handle = Grid::new()
            .element_ref(&self.splitter)
            .width(6.0)
            .horizontal_alignment(HorizontalAlignment::Right)
            .children((handle,));
        // The left-right arrows show over the handle. Reactor has no cursor property, so the
        // native element's (protected) cursor is set.
        let splitter = self.splitter.clone();
        context.use_effect("splitter-cursor", self.show_navigation, move || {
            _ = splitter.request_native_object(|result| {
                if let Err(error) = result
                    .map_err(|error| format!("{error:?}"))
                    .and_then(|object| resize_cursor(&object).map_err(|e| e.to_string()))
                {
                    crate::log::log!("could not set the resize cursor: {error}");
                }
            });
            None
        });
        Grid::new().children((self.navigation_pane(context), handle))
    }

    /// The Document Editor: Word's Navigation pane, with Headings, Find and Replace tabs.
    fn navigation_pane(&self, context: &mut ViewContext<Self>) -> View {
        let header = Grid::new()
            .columns([GridLength::STAR, GridLength::Auto])
            .children((
                TextBlock::new()
                    .text("Document Editor")
                    .font_size(18.0)
                    .font_weight(FontWeight::SEMI_BOLD)
                    .vertical_alignment(VerticalAlignment::Center)
                    .grid_column(0),
                Button::new()
                    .style(ButtonStyle::Subtle)
                    .on_click(context.message(Msg::Command(Command::ToggleNavigation)))
                    .content(symbol(Symbol::Cancel))
                    .tooltip("Close the Document Editor")
                    .in_column(1),
            ));
        // The tools as text tabs. The selected one is underlined across its whole label (the
        // SelectorBar's indicator is a short fixed-width pill). Tabs that do not fit the pane
        // move into a "More" menu at the end of the row, as the ribbon's commands do.
        const TABS: [(&str, ExplorerTab); 4] = [
            ("Navigation", ExplorerTab::Headings),
            ("Find", ExplorerTab::Find),
            ("Replace", ExplorerTab::Replace),
            ("Spelling", ExplorerTab::Spelling),
        ];
        let shown = tabs_that_fit(
            &TABS.map(|(label, _)| label),
            self.explorer_width() - 2.0 * PANE_PADDING,
        );
        let underline = |selected: bool| {
            Border::new()
                .height(3.0)
                .corner_radius(CornerRadius::uniform(1.5))
                .background(if selected {
                    Brush::Theme(ThemeBrush::Accent)
                } else {
                    Brush::from(Color::transparent())
                })
        };
        let row = StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(TAB_SPACING)
            .keyed_children(
                TABS[..shown]
                    .iter()
                    .map(|&(label, tab)| {
                        let selected = self.explorer_tab == tab;
                        let button = Button::new()
                            .style(ButtonStyle::Subtle)
                            .on_click(
                                context.message(Msg::ExplorerTab(Some(label.to_string()))),
                            )
                            .content(StackPanel::new().spacing(5.0).children((
                                TextBlock::new().text(label).font_size(14.0).foreground(
                                    if selected { ThemeBrush::PrimaryText } else { MUTED },
                                ),
                                underline(selected),
                            )));
                        (label, button)
                    })
                    .collect::<Vec<_>>(),
            );
        let hidden = &TABS[shown..];
        let overflowing = !hidden.is_empty();
        let selected_hidden = hidden.iter().any(|&(_, tab)| tab == self.explorer_tab);
        // Always there, shrunk to nothing when every tab fits: Reactor cannot add or remove a
        // view that carries a menu.
        let more = Button::new()
            .style(ButtonStyle::Subtle)
            .is_enabled(overflowing)
            .min_width(0.0)
            .width(if overflowing { MORE_TABS_WIDTH } else { 0.0 })
            .margin(Thickness::new(
                if overflowing { TAB_SPACING } else { 0.0 },
                0.0,
                0.0,
                0.0,
            ))
            // Reactor crashes on a tooltip around a view with a menu, so the content carries it.
            .content(
                StackPanel::new()
                    .spacing(5.0)
                    .children((path_icon(icons::MORE, 20.0), underline(selected_hidden)))
                    .tooltip("More tools"),
            )
            .menu(Menu::new(
                hidden.iter().map(|&(label, tab)| {
                    MenuItem::item(
                        label,
                        if tab == self.explorer_tab {
                            format!("\u{2713}  {label}")
                        } else {
                            label.to_string()
                        },
                    )
                }),
                context.callback(|label: String| {
                    Msg::ExplorerTab(Some(label.trim_start_matches(['\u{2713}', ' ']).to_string()))
                }),
            ));
        let tabs = Grid::new()
            .columns([GridLength::Auto, GridLength::Auto])
            .children((row.in_column(0), more.in_column(1)));
        // The search box serves Navigation, Find and Replace; spelling has nothing to type.
        let search: View = if self.explorer_tab == ExplorerTab::Spelling {
            View::empty()
        } else {
            TextBox::new()
                .element_ref(&self.search_box)
                .text(self.query.clone())
                .placeholder_text("Search in document")
                .on_text_changed(context.callback(Msg::Query))
                .into()
        };

        let body = match self.explorer_tab {
            ExplorerTab::Headings => self.headings_tab(context),
            ExplorerTab::Find => self.results(context),
            ExplorerTab::Replace => self.replace_tab(context),
            ExplorerTab::Spelling => self.spelling_tab(context),
        };

        let search_box = self.search_box.clone();
        context.use_effect("focus-search", self.focus_search, move || {
            _ = search_box.request_focus();
            None
        });

        Border::new()
            .border_brush(DIVIDER)
            .border_thickness(Thickness::new(0.0, 0.0, 1.0, 0.0))
            .padding(Thickness::xy(PANE_PADDING, 12.0))
            .content(
                Grid::new()
                    .rows([
                        GridLength::Auto,
                        GridLength::Auto,
                        GridLength::Auto,
                        GridLength::STAR,
                    ])
                    .row_spacing(10.0)
                    .children((
                        header.in_row(0),
                        tabs.in_row(1),
                        search.in_row(2),
                        body.in_row(3),
                    )),
            )
    }

    /// The Spelling tool: every misspelling in the page, each with its suggestions and what to
    /// do about it, like the spelling part of Word's Editor pane.
    fn spelling_tab(&self, context: &mut ViewContext<Self>) -> View {
        if self.mode == Mode::Source {
            return notice(
                context,
                "Switch to the Visual view to check spelling.",
                None,
            );
        }
        let count = self.spelling.len();
        if count == 0 {
            return notice(context, "No spelling issues found on this page.", None);
        }
        let fixable = self.spelling.iter().any(|i| !i.suggestions.is_empty());
        let summary = Grid::new()
            .columns([GridLength::STAR, GridLength::Auto])
            .children((
                caption(&if count == 1 {
                    "1 spelling issue".to_string()
                } else {
                    format!("{count} spelling issues")
                })
                .in_column(0),
                Button::new()
                    .style(ButtonStyle::Accent)
                    .is_enabled(fixable)
                    .on_click(context.message(Msg::SpellApplyAll))
                    .content("Apply all")
                    .tooltip("Change every misspelling to its chosen suggestion")
                    .in_column(1),
            ));
        let list = ScrollViewer::new()
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
            .content(
                StackPanel::new().spacing(8.0).keyed_children(
                    self.spelling
                        .iter()
                        .enumerate()
                        .map(|(index, issue)| {
                            (themed_key(index), self.spelling_row(context, index, issue))
                        })
                        .collect::<Vec<_>>(),
                ),
            );
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .row_spacing(10.0)
            .children((summary.in_row(0), list.in_row(1)))
    }

    /// One misspelling: the word in context (click to go to it), its suggestions (click to
    /// pick one), and Change, Change all, Ignore and Ignore all.
    fn spelling_row(
        &self,
        context: &mut ViewContext<Self>,
        index: usize,
        issue: &spelling::Issue,
    ) -> View {
        let snippet = RichText::single_paragraph([
            RichTextInline::Run(RichTextRun::plain(issue.before.clone())),
            RichTextInline::Run(RichTextRun {
                text: issue.word.clone(),
                is_bold: true,
                is_italic: false,
            }),
            RichTextInline::Run(RichTextRun::plain(issue.after.clone())),
        ]);
        let context_button = Button::new()
            .style(ButtonStyle::Subtle)
            .on_click(context.message(Msg::SpellGoTo(index)))
            .horizontal_alignment(HorizontalAlignment::Stretch)
            .horizontal_content_alignment(HorizontalAlignment::Left)
            .content(
                RichTextBlock::new()
                    .paragraphs(snippet)
                    // Selectable text would take the click from the button.
                    .is_text_selection_enabled(false)
                    .text_wrapping(TextWrapping::Wrap),
            )
            .tooltip("Go to this word");
        let chosen = self
            .spelling_choice
            .get(&issue.start)
            .copied()
            .unwrap_or(0);
        let suggestions: View = if issue.suggestions.is_empty() {
            caption("No suggestions")
        } else {
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(6.0)
                .keyed_children(
                    issue
                        .suggestions
                        .iter()
                        .enumerate()
                        .map(|(choice, suggestion)| {
                            let picked = choice == chosen;
                            let button = Button::new()
                                .on_click(context.message(Msg::SpellChoose(index, choice)));
                            let button = if picked {
                                button.resource_overrides(active_resources())
                            } else {
                                button
                            };
                            // Keyed by whether it is picked: a button keeps its old colors
                            // when its resource overrides change.
                            (
                                (choice as u64) << 1 | u64::from(picked),
                                button.content(suggestion.clone()),
                            )
                        })
                        .collect::<Vec<_>>(),
                )
        };
        let same = self
            .spelling
            .iter()
            .filter(|other| other.word == issue.word)
            .count();
        let fixable = !issue.suggestions.is_empty();
        let link = |context: &mut ViewContext<Self>, text: String, message: Msg, enabled: bool| {
            Button::new()
                .style(ButtonStyle::TextLink)
                .is_enabled(enabled)
                .on_click(context.message(message))
                .content(text)
        };
        let actions = StackPanel::new().spacing(2.0).children((
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(4.0)
                .children((
                    link(context, "Change".into(), Msg::SpellChange(index), fixable),
                    link(
                        context,
                        format!("Change all ({same})"),
                        Msg::SpellChangeAll(index),
                        fixable && same > 1,
                    ),
                )),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(4.0)
                .children((
                    link(context, "Ignore".into(), Msg::SpellIgnore(index), true),
                    link(
                        context,
                        "Ignore all".into(),
                        Msg::SpellIgnoreAll(index),
                        true,
                    ),
                )),
        ));
        Border::new()
            .border_brush(DIVIDER)
            .border_thickness(Thickness::uniform(1.0))
            .corner_radius(CornerRadius::uniform(6.0))
            .padding(Thickness::uniform(8.0))
            .content(StackPanel::new().spacing(8.0).children((
                context_button,
                suggestions,
                actions,
            )))
    }

    fn headings_tab(&self, context: &mut ViewContext<Self>) -> View {
        let entries = self.outline();
        if entries.is_empty() && self.mode == Mode::Source {
            return notice(
                context,
                "This page doesn't have any headings. To add one, start a line with # \
                 (## for Heading 2, and so on).",
                None,
            );
        }
        if entries.is_empty() {
            return notice(
                context,
                "This page doesn't have any headings. To add headings for creating an \
                 interactive outline, go to Home > Styles.",
                Some(("Show me how", Msg::ShowMeHow)),
            );
        }
        ScrollViewer::new()
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
            .content(
                StackPanel::new().spacing(1.0).keyed_children(
                    self.visible_entries(&entries)
                        .into_iter()
                        .map(|(index, entry)| {
                            (
                                themed_key(index),
                                outline_row(
                                    context,
                                    entry,
                                    self.collapsed.contains(&entry.key),
                                    self.options.heading_styles,
                                    self.explorer_width(),
                                ),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            )
    }

    fn replace_tab(&self, context: &mut ViewContext<Self>) -> View {
        let has_matches = !self.matches.is_empty();
        Grid::new()
            .rows([GridLength::Auto, GridLength::Auto, GridLength::STAR])
            .row_spacing(10.0)
            .children((
                TextBox::new()
                    .text(self.replacement.clone())
                    .placeholder_text("Replace with")
                    .on_text_changed(context.callback(Msg::Replacement))
                    .grid_row(0),
                StackPanel::new()
                    .orientation(Orientation::Horizontal)
                    .spacing(8.0)
                    .children((
                        Button::new()
                            .is_enabled(has_matches)
                            .on_click(context.message(Msg::ReplaceOne))
                            .content("Replace"),
                        Button::new()
                            .style(ButtonStyle::Accent)
                            .is_enabled(has_matches)
                            .on_click(context.message(Msg::ReplaceAll))
                            .content("Replace all"),
                    ))
                    .in_row(1),
                self.results(context).in_row(2),
            ))
    }

    /// Search options, the result count with previous/next, and the list of results.
    fn results(&self, context: &mut ViewContext<Self>) -> View {
        // Side by side when they fit the pane, one above the other when it is narrow.
        let side_by_side = self.explorer_width() - 2.0 * PANE_PADDING >= 290.0;
        let options = StackPanel::new()
            .orientation(if side_by_side {
                Orientation::Horizontal
            } else {
                Orientation::Vertical
            })
            .spacing(16.0)
            .children((
                CheckBox::new()
                    .is_checked(self.match_case)
                    .on_is_checked_changed(context.callback(Msg::MatchCase))
                    .content("Match case"),
                CheckBox::new()
                    .is_checked(self.whole_words)
                    .on_is_checked_changed(context.callback(Msg::WholeWords))
                    .content("Whole words"),
            ));
        let count = self.matches.len();
        let summary: View = if self.query.is_empty() {
            caption("Type in the box above to search this page.")
        } else if count == 0 {
            caption("No results")
        } else {
            let label = match self.current_match {
                Some(index) => format!("Result {} of {count}", index + 1),
                None if count == 1 => "1 result".to_string(),
                None => format!("{count} results"),
            };
            Grid::new()
                .columns([GridLength::STAR, GridLength::Auto, GridLength::Auto])
                .children((
                    caption(&label).in_column(0),
                    Button::new()
                        .style(ButtonStyle::Subtle)
                        .on_click(context.message(Msg::PreviousMatch))
                        .content(glyph("\u{E70E}"))
                        .tooltip("Previous result")
                        .in_column(1),
                    Button::new()
                        .style(ButtonStyle::Subtle)
                        .on_click(context.message(Msg::NextMatch))
                        .content(glyph("\u{E70D}"))
                        .tooltip("Next result")
                        .in_column(2),
                ))
        };
        let list = ScrollViewer::new()
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
            .content(
                StackPanel::new().spacing(2.0).keyed_children(
                    self.matches
                        .iter()
                        .enumerate()
                        .map(|(index, found)| {
                            (
                                themed_key(index),
                                result_row(
                                    context,
                                    index,
                                    found,
                                    self.current_match == Some(index),
                                ),
                            )
                        })
                        .collect::<Vec<_>>(),
                ),
            );
        Grid::new()
            .rows([GridLength::Auto, GridLength::Auto, GridLength::STAR])
            .row_spacing(8.0)
            .children((options.in_row(0), summary.in_row(1), list.in_row(2)))
    }

    /// The entries not hidden under a collapsed heading, with their index in the outline.
    fn visible_entries<'a>(&self, entries: &'a [OutlineEntry]) -> Vec<(usize, &'a OutlineEntry)> {
        let mut visible = Vec::new();
        let mut hidden_below: Option<u8> = None;
        for (index, entry) in entries.iter().enumerate() {
            if let Some(level) = hidden_below {
                if entry.heading.level > level {
                    continue;
                }
                hidden_below = None;
            }
            if entry.has_children && self.collapsed.contains(&entry.key) {
                hidden_below = Some(entry.heading.level);
            }
            visible.push((index, entry));
        }
        visible
    }
}

/// One heading in the Navigation pane: an expand/collapse chevron when it has subheadings, and
/// the heading text, indented by level.
fn outline_row(
    context: &mut ViewContext<Editor>,
    entry: &OutlineEntry,
    collapsed: bool,
    show_style: bool,
    pane_width: f64,
) -> View {
    const INDENT: f64 = 14.0;
    let indent = f64::from(entry.heading.level.saturating_sub(1)) * INDENT;
    let chevron: View = if entry.has_children {
        // A button's padding is wider than this column, so the triangle is a clickable Border.
        let key = entry.key.clone();
        Border::new()
            .background(Color::transparent())
            .on_pointer_pressed(context.callback(move |_| Msg::ToggleOutline(key.clone())))
            .content(
                TextBlock::new()
                    .text(if collapsed { "\u{25B8}" } else { "\u{25BE}" })
                    .font_size(14.0)
                    .foreground(MUTED)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Center),
            )
            .tooltip(if collapsed { "Expand" } else { "Collapse" })
    } else {
        View::empty()
    };
    let text = if entry.heading.text.is_empty() {
        "(Empty heading)".to_string()
    } else {
        entry.heading.text.clone()
    };
    let label = TextBlock::new()
        .text(text.clone())
        .text_trimming(TextTrimming::CharacterEllipsis)
        .max_lines(1);
    let label = if entry.current {
        label.font_weight(FontWeight::SEMI_BOLD)
    } else {
        label
    };
    // Settings › Options: the heading's style after its name, such as (H2), in grey. The name
    // is capped so the style stays in view right after it when the name is cut short.
    let label: View = if show_style {
        let style = format!("({})", heading_abbreviation(entry.heading.level));
        // The pane's padding, the indent and chevron columns, and the button's own padding.
        let room = pane_width - 2.0 * PANE_PADDING - indent - 24.0 - 24.0;
        let name_room = (room - text_width(&style, 14.0) - 6.0).max(24.0);
        StackPanel::new()
            .orientation(Orientation::Horizontal)
            .spacing(6.0)
            .children((
                label.max_width(name_room),
                TextBlock::new().text(style).foreground(MUTED).max_lines(1),
            ))
    } else {
        label.into()
    };
    let button = Button::new()
        .on_click(context.message(Msg::Navigate(entry.heading.position)))
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .horizontal_content_alignment(HorizontalAlignment::Left);
    let button = if entry.current {
        button.resource_overrides(active_resources())
    } else {
        button.style(ButtonStyle::Subtle)
    };
    Grid::new()
        .columns([
            GridLength::Pixel(indent),
            GridLength::Pixel(24.0),
            GridLength::STAR,
        ])
        .children((
            chevron.in_column(1),
            button.content(label).tooltip(&text).in_column(2),
        ))
}

/// A search result: the match in bold with the text around it, like Word's results list.
fn result_row(
    context: &mut ViewContext<Editor>,
    index: usize,
    found: &SearchMatch,
    current: bool,
) -> View {
    let snippet = RichText::single_paragraph([
        RichTextInline::Run(RichTextRun::plain(found.before.clone())),
        RichTextInline::Run(RichTextRun {
            text: found.text.clone(),
            is_bold: true,
            is_italic: false,
        }),
        RichTextInline::Run(RichTextRun::plain(found.after.clone())),
    ]);
    let button = Button::new()
        .on_click(context.message(Msg::GoToMatch(index)))
        .horizontal_alignment(HorizontalAlignment::Stretch)
        .horizontal_content_alignment(HorizontalAlignment::Left);
    let button = if current {
        button.resource_overrides(active_resources())
    } else {
        button.style(ButtonStyle::Subtle)
    };
    button.content(
        RichTextBlock::new()
            .paragraphs(snippet)
            // Selectable text would take the click from the result button.
            .is_text_selection_enabled(false)
            .text_wrapping(TextWrapping::Wrap),
    )
}

/// A bordered message card, like Word's "This document doesn't have any headings" card.
fn notice(context: &mut ViewContext<Editor>, text: &str, action: Option<(&str, Msg)>) -> View {
    let link = match action {
        Some((label, message)) => Button::new()
            .style(ButtonStyle::TextLink)
            .on_click(context.message(message))
            .content(label.to_string()),
        None => View::empty(),
    };
    Border::new()
        .border_brush(DIVIDER)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(6.0))
        .padding(Thickness::uniform(12.0))
        .vertical_alignment(VerticalAlignment::Top)
        .content(
            StackPanel::new()
                .spacing(6.0)
                .children((caption(text), link)),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tool(width: f64) -> Item {
        Item::new(View::empty(), width, Vec::new())
    }

    #[test]
    fn commands_that_do_not_fit_move_to_the_more_menu() {
        let items = || vec![tool(100.0), separator(), tool(100.0), tool(100.0)];
        let (shown, hidden) = fit(items(), 1000.0);
        assert_eq!((shown.len(), hidden.len()), (4, 0));

        // Room for one command beside the More button: the separator after it is dropped,
        // and the overflow does not start with one either.
        let (shown, hidden) = fit(items(), 100.0 + ITEM_SPACING + MORE_WIDTH + 20.0);
        assert_eq!(shown.len(), 1);
        assert_eq!(hidden.len(), 2);
        assert!(hidden.iter().all(|item| !item.separator));
    }
}

#[cfg(test)]
mod navigation_tests {
    use super::*;

    #[test]
    fn the_document_explorer_stays_within_its_limits() {
        assert_eq!(navigation_width(100.0, 1600.0), NAVIGATION_MIN);
        assert_eq!(navigation_width(350.0, 1600.0), 350.0);
        assert_eq!(navigation_width(2000.0, 1600.0), NAVIGATION_MAX);
        // A narrow window keeps room for the page.
        assert_eq!(navigation_width(600.0, 800.0), 800.0 - PAGE_MIN_ROOM);
    }
}

/// Room around the Document Editor's contents, the same on both sides.
const PANE_PADDING: f64 = 16.0;
const TAB_SPACING: f64 = 2.0;
const MORE_TABS_WIDTH: f64 = 44.0;

/// How many of the Document Editor's tabs fit in `room` pixels, keeping room for the "More"
/// button when some do not. Widths are estimated from the labels (14px Segoe UI, plus the
/// button's padding), as the pane cannot measure them before laying them out.
fn tabs_that_fit(labels: &[&str], room: f64) -> usize {
    let width = |label: &str| label.chars().count() as f64 * 6.6 + 22.0 + TAB_SPACING;
    if labels.iter().map(|label| width(label)).sum::<f64>() <= room {
        return labels.len();
    }
    let mut used = MORE_TABS_WIDTH + TAB_SPACING;
    labels
        .iter()
        .take_while(|label| {
            used += width(label);
            used <= room
        })
        .count()
}

#[cfg(test)]
mod pane_tests {
    use super::*;

    #[test]
    fn tabs_move_into_the_more_menu_when_the_pane_narrows() {
        let labels = ["Navigation", "Find", "Replace", "Spelling"];
        assert_eq!(tabs_that_fit(&labels, 400.0), 4);
        assert_eq!(tabs_that_fit(&labels, 260.0), 3);
        assert_eq!(tabs_that_fit(&labels, 120.0), 0);
    }
}

/// A heading style's short name for the Navigation pane: Heading 2 is "H2".
fn heading_abbreviation(level: u8) -> String {
    format!("H{level}")
}

/// A title band tab's name, at the button's default (WinUI) size, with an accent underline
/// under the selected tab (the others keep a clear one, so labels line up).
fn tab_label(text: &str, selected: bool) -> View {
    StackPanel::new()
        .spacing(3.0)
        .children((
            TextBlock::new().text(text),
            Border::new()
                .height(3.0)
                .corner_radius(CornerRadius::uniform(1.5))
                .background(if selected {
                    Brush::Theme(ThemeBrush::Accent)
                } else {
                    Brush::from(Color::transparent())
                }),
        ))
}
