//! Settings: shown over the whole window below the title bar (the gear at the right of the
//! tabs opens it; the title bar's back button closes it), like Notepad's settings. Categories
//! run down the left, as in Windows 11 Settings; each setting sits on its own card with its
//! name, a description and its control.

use super::*;

/// The width of the categories pane.
const PANE_LENGTH: f64 = 280.0;
/// The room either side of a category's page.
const PAGE_MARGIN: f64 = 40.0;
/// The widest a category's page grows; past this it is centered in the space beside the
/// categories, as in Windows 11 Settings.
const PAGE_MAX_WIDTH: f64 = 1000.0;

impl Editor {
    pub(in crate::app) fn settings_page(
        &self,
        category: SettingsCategory,
        context: &mut ViewContext<Self>,
    ) -> View {
        let items = SETTINGS_CATEGORIES.iter().map(|&(tag, name, item)| {
            let icon = match item {
                SettingsCategory::Options => icons::OPTIONS,
                SettingsCategory::About => icons::INFO,
            };
            (
                tag,
                NavigationViewItem::new()
                    .tag(tag)
                    .is_selected(item == category)
                    .slots([
                        SlotView::new(NavigationViewItemSlot::Icon, path_icon(icon, 20.0)),
                        SlotView::new(NavigationViewItemSlot::Content, TextBlock::new().text(name)),
                    ]),
            )
        });
        let page = match category {
            SettingsCategory::Options => self.options_page(context),
            SettingsCategory::About => about_page(),
        };
        NavigationView::new()
            .pane_display_mode(NavigationViewPaneDisplayMode::Left)
            .is_pane_open(true)
            .is_pane_toggle_button_visible(false)
            // The title bar has the back button.
            .is_back_button_visible(NavigationViewBackButtonVisible::Collapsed)
            .is_settings_visible(false)
            .open_pane_length(PANE_LENGTH)
            .on_selected_tag_changed(context.callback(Msg::SettingsCategory))
            .slots([
                SlotView::collection(NavigationViewSlot::MenuItems, items),
                SlotView::new(NavigationViewSlot::Content, page),
            ])
    }

    fn options_page(&self, context: &mut ViewContext<Self>) -> View {
        let space = CURSOR_SPACES
            .iter()
            .position(|&(_, space)| space == self.options.cursor_space);
        let paragraph_size = PARAGRAPH_SIZES
            .iter()
            .position(|&(_, size)| size == self.options.paragraph_size);
        category_page(
            "Options",
            vec![
                section("Document Editor"),
                card(
                    "Always show the Document Editor",
                    "Opens the Document Editor pane (Navigation, Find, Replace and Spelling) \
                     every time the app starts. You can still close it while you work.",
                    toggle(self.options.document_editor, context.callback(Msg::DocumentEditor)),
                ),
                card(
                    "Show heading styles in Navigation",
                    "Adds each heading's style after its name, such as (H2), so new content \
                     can be styled to match what is already there.",
                    toggle(self.options.heading_styles, context.callback(Msg::HeadingStyles)),
                ),
                section("Visual editor"),
                card(
                    "Paragraph font size",
                    "The size of normal text, lists and quotes on the page, so it can match \
                     how your SharePoint site shows them. It changes only how the page looks \
                     here, not the Markdown that is saved.",
                    ComboBox::new()
                        .items_source(PARAGRAPH_SIZES.iter().map(|&(name, _)| name))
                        .selected_index(paragraph_size)
                        .on_selection_changed(context.callback(Msg::ParagraphSize))
                        .width(220.0)
                        .horizontal_alignment(HorizontalAlignment::Right)
                        .into(),
                ),
                section("Typing"),
                card(
                    "Space below the cursor",
                    "Keeps blank page below the line you are typing on, so you are not always \
                     writing at the bottom of the window.",
                    ComboBox::new()
                        .items_source(CURSOR_SPACES.iter().map(|&(name, _)| name))
                        .selected_index(space)
                        .on_selection_changed(context.callback(Msg::CursorSpace))
                        .width(220.0)
                        .horizontal_alignment(HorizontalAlignment::Right)
                        .into(),
                ),
            ],
        )
    }
}

/// Settings › About: what was the About box. Help › About opens it.
fn about_page() -> View {
    category_page(
        "About",
        vec![
            app_card(),
            TextBlock::new()
                .text(
                    "A native Windows editor for SharePoint Knowledge Base articles. It works \
                     like Word, has a Markdown source view, and saves the Markdown files a \
                     knowledge base imports.",
                )
                .text_wrapping(TextWrapping::Wrap)
                .margin(Thickness::new(0.0, 16.0, 0.0, 4.0))
                .into(),
            HyperlinkButton::new()
                // A fixed, valid address, so this cannot fail.
                .navigate_uri(PROJECT_URL)
                .unwrap_or_default()
                // Line the link's text up with the text above.
                .margin(Thickness::new(-11.0, 0.0, 0.0, 0.0))
                .content("View the project on GitHub")
                .into(),
        ],
    )
}

/// The app's icon, name and version, on a card like a setting's.
fn app_card() -> View {
    Border::new()
        .background(ThemeBrush::CardBackground)
        .border_brush(ThemeBrush::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(6.0))
        .padding(Thickness::new(16.0, 12.0, 16.0, 12.0))
        .min_height(68.0)
        .content(
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(16.0)
                .children((
                    Image::new()
                        .source_data(EncodedImage::from_static(APP_ICON_LARGE))
                        .width(48.0)
                        .height(48.0)
                        .vertical_alignment(VerticalAlignment::Center),
                    StackPanel::new()
                        .spacing(2.0)
                        .vertical_alignment(VerticalAlignment::Center)
                        .children((
                            TextBlock::new().text(APP_NAME),
                            TextBlock::new()
                                .text(format!("Version {APP_VERSION}"))
                                .font_size(12.0)
                                .foreground(MUTED),
                        )),
                )),
        )
        .into()
}

/// A category: its name as a large header, then its sections and cards, all left-aligned in
/// one column. As in Windows 11 Settings, the column fills the space beside the categories up
/// to a limit, then stays centered in it (WinUI centers a stretched element that its maximum
/// width holds back).
fn category_page(title: &str, children: Vec<View>) -> View {
    let header = TextBlock::new()
        .text(title)
        .font_size(28.0)
        .font_weight(FontWeight::SEMI_BOLD)
        .margin(Thickness::new(0.0, 0.0, 0.0, 12.0));
    let rows = std::iter::once(header.into())
        .chain(children)
        .enumerate()
        .map(|(index, view)| (index as u64, view))
        .collect::<Vec<_>>();
    ScrollViewer::new()
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
        .content(
            StackPanel::new()
                .spacing(4.0)
                .margin(Thickness::new(PAGE_MARGIN, 32.0, PAGE_MARGIN, 32.0))
                .max_width(PAGE_MAX_WIDTH)
                .horizontal_alignment(HorizontalAlignment::Stretch)
                .keyed_children(rows),
        )
}

/// A heading over a group of cards.
fn section(title: &str) -> View {
    TextBlock::new()
        .text(title)
        .font_size(14.0)
        .font_weight(FontWeight::SEMI_BOLD)
        .margin(Thickness::new(0.0, 16.0, 0.0, 4.0))
        .into()
}

/// An on/off switch for a card, as in Windows 11 Settings: "On" or "Off" to the left of the
/// switch, and the switch against the card's right edge. The switch's own label sits to its
/// right and reserves room (a minimum width), which would leave a gap there, so the switch is
/// cut to its track and labelled here instead.
fn toggle(on: bool, toggled: Callback<bool>) -> View {
    // The track of WinUI's ToggleSwitch.
    const TRACK_WIDTH: f64 = 40.0;
    StackPanel::new()
        .orientation(Orientation::Horizontal)
        .spacing(12.0)
        .children((
            TextBlock::new()
                .text(if on { "On" } else { "Off" })
                .vertical_alignment(VerticalAlignment::Center),
            ToggleSwitch::new()
                .is_on(on)
                .on_toggled(toggled)
                .min_width(0.0)
                .width(TRACK_WIDTH)
                .vertical_alignment(VerticalAlignment::Center),
        ))
        .into()
}

/// One setting: its name and description on the left, its control on the right.
fn card(title: &str, description: &str, control: View) -> View {
    let text = StackPanel::new()
        .spacing(2.0)
        .vertical_alignment(VerticalAlignment::Center)
        .grid_column(0)
        .children((
            TextBlock::new().text(title),
            TextBlock::new()
                .text(description)
                .font_size(12.0)
                .foreground(MUTED)
                .text_wrapping(TextWrapping::Wrap),
        ));
    Border::new()
        .background(ThemeBrush::CardBackground)
        .border_brush(ThemeBrush::CardStroke)
        .border_thickness(Thickness::uniform(1.0))
        .corner_radius(CornerRadius::uniform(6.0))
        .padding(Thickness::new(16.0, 12.0, 16.0, 12.0))
        .min_height(68.0)
        .content(
            Grid::new()
                .columns([GridLength::STAR, GridLength::Auto])
                .column_spacing(24.0)
                .children((
                    text,
                    // Every control ends at the card's right edge (less its padding).
                    Border::new()
                        .vertical_alignment(VerticalAlignment::Center)
                        .horizontal_alignment(HorizontalAlignment::Right)
                        .grid_column(1)
                        .content(control),
                )),
        )
        .into()
}
