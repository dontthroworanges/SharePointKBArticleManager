//! Adds the Home tab's commands to the editor's right-click menu.
//!
//! WinUI's RichEditBox shows a `TextCommandBarFlyout`: a row of commands (Bold, Italic,
//! Underline) above a list (Cut, Copy, Paste, Select All). It rebuilds its commands each time
//! it opens, so, as documented for that control, changes are made in its `Opening` event. The
//! row is emptied, and the list gains the Font and Paragraph commands (when text is selected)
//! and the paragraph styles, one item each, so each is a single click away. The popup of
//! commands that RichEditBox shows over selected text is turned off.

use super::view::icons;
use super::{Command, Id, Msg, QUOTE_STYLE, STYLES};
use crate::markdown::ListKind;
use crate::rich::InlineCommand;
use crate::winui::*;
use std::cell::RefCell;
use std::rc::Rc;
use windows_core::{Interface, Result};
use windows_reactor::LocalSender;

/// The Home tab's Font and Paragraph commands, offered in the menu when text is selected.
/// (Quote is with the styles, which are always offered.)
const FORMAT_COMMANDS: [(&str, &str, Command); 12] = [
    ("Bold", icons::BOLD, Command::Inline(InlineCommand::Bold)),
    ("Italic", icons::ITALIC, Command::Inline(InlineCommand::Italic)),
    (
        "Strikethrough",
        icons::STRIKETHROUGH,
        Command::Inline(InlineCommand::Strikethrough),
    ),
    ("Highlight", icons::HIGHLIGHT, Command::Inline(InlineCommand::Highlight)),
    ("Inline code", icons::INLINE_CODE, Command::Inline(InlineCommand::Code)),
    ("Inline equation", icons::MATH, Command::Inline(InlineCommand::Math)),
    ("Bulleted list", icons::BULLETS, Command::List(ListKind::Bullet)),
    ("Numbered list", icons::NUMBERED, Command::List(ListKind::Numbered)),
    (
        "Task list",
        icons::TASKS,
        Command::List(ListKind::Task { checked: false }),
    ),
    ("Mark task done", icons::MARK_DONE, Command::ToggleDone),
    ("Decrease list level", icons::OUTDENT, Command::Outdent),
    ("Increase list level", icons::INDENT, Command::Indent),
];

/// Customizes the right-click menu of a flow editor, and turns off the popup over selected
/// text. The returned revokers keep the handlers alive.
pub fn attach(
    editor: &IRichEditBox,
    id: Id,
    sender: &LocalSender<Msg>,
) -> Result<Vec<windows_core::EventRevoker>> {
    editor.SetSelectionFlyout(None::<&FlyoutBase>)?;
    let element: IUIElement = editor.cast()?;
    let mut revokers = Vec::new();
    if let Ok(flyout) = element.ContextFlyout()
        && let Ok(menu) = flyout.cast::<ICommandBarFlyout>()
    {
        revokers.extend(customize(&flyout, menu, editor, id, sender)?);
    }
    Ok(revokers)
}

fn customize(
    flyout: &FlyoutBase,
    menu: ICommandBarFlyout,
    editor: &IRichEditBox,
    id: Id,
    sender: &LocalSender<Msg>,
) -> Result<Vec<windows_core::EventRevoker>> {
    let base: IFlyoutBase = flyout.cast()?;
    let format = format_items(id, sender, &base)?;
    let styles = style_items(id, sender, &base)?;
    let document = editor.TextDocument()?;
    // The document's formatting when the menu opened, to notice Bold or Italic from the menu.
    let before = Rc::new(RefCell::new(String::new()));

    // The editors on a page can share one menu, so each handler acts only when the menu opens
    // for its own editor.
    let editor_identity = editor.cast::<windows_core::IUnknown>()?.as_raw() as usize;
    let opening = {
        let before = Rc::clone(&before);
        let document = document.clone();
        let menu = menu.clone();
        let target = base.clone();
        base.Opening(move |_, _| {
            // Taken for every editor, so closing the menu only reports a real change.
            *before.borrow_mut() = formatted_text(&document);
            let for_this_editor = target
                .Target()
                .and_then(|element| element.cast::<windows_core::IUnknown>())
                .is_ok_and(|element| element.as_raw() as usize == editor_identity);
            if !for_this_editor {
                return;
            }
            let selected = document
                .Selection()
                .and_then(|selection| selection.cast::<ITextRange>())
                .and_then(|range| Ok(range.EndPosition()? > range.StartPosition()?))
                .unwrap_or(false);
            let added = add_items(&menu, &format, &styles, selected);
            if let Err(error) = added.and_then(|()| theme_commands(&menu)) {
                crate::log::log!("customizing the text menu failed: {error:?}");
            }
        })?
    };
    // The menu's frame (its "…" button, background) is only in the visual tree once it is open.
    let opened = base.Opened(move |_, _| {
        if let Err(error) = theme_popup(&menu).and_then(|()| style_style_items(&menu)) {
            crate::log::log!("theming the text menu failed: {error:?}");
        }
    })?;
    let closed = {
        let sender = sender.clone();
        base.Closed(move |_, _| {
            if *before.borrow() != formatted_text(&document) {
                sender.send(Msg::Edited(id));
            }
        })?
    };
    Ok(vec![opening, opened, closed])
}

/// The window's theme, for the menu. The editor sits on the page, which is kept light, and the
/// menu's commands take the editor's theme while its frame follows the window, which gave dark
/// text on a dark menu. Every part of the menu is given the window's theme instead.
fn window_theme() -> ElementTheme {
    if super::theme::current().dark {
        ElementTheme::Dark
    } else {
        ElementTheme::Light
    }
}

/// Gives each command in the menu, and each submenu (the spell checker's Proofing), the
/// window's theme.
fn theme_commands(menu: &ICommandBarFlyout) -> Result<()> {
    let theme = window_theme();
    for commands in [menu.PrimaryCommands()?, menu.SecondaryCommands()?] {
        for index in 0..commands.Size()? {
            let command = commands.GetAt(index)?;
            command
                .cast::<IFrameworkElement>()?
                .SetRequestedTheme(theme)?;
        }
    }
    match_submenus_to_theme(&menu.SecondaryCommands()?)
}

/// Gives the open menu's whole popup the window's theme: every element from one of its
/// commands up to the popup's root, since the menu's own command bar carries the editor's
/// theme and would otherwise keep it (its "…" button stayed dark on dark).
fn theme_popup(menu: &ICommandBarFlyout) -> Result<()> {
    // The list, as the row above it is emptied.
    let commands = menu.SecondaryCommands()?;
    if commands.Size()? == 0 {
        return Ok(());
    }
    let theme = window_theme();
    let mut current: DependencyObject = commands.GetAt(0)?.cast()?;
    while let Ok(parent) = VisualTreeHelper::GetParent(&current) {
        if let Ok(element) = parent.cast::<IFrameworkElement>() {
            element.SetRequestedTheme(theme)?;
        }
        current = parent;
    }
    Ok(())
}

/// Puts the app's items in the menu's list: the Font and Paragraph commands when text is
/// selected, then the styles. Empties the row of buttons above the list.
fn add_items(
    menu: &ICommandBarFlyout,
    format: &[ICommandBarElement],
    styles: &[ICommandBarElement],
    selected: bool,
) -> Result<()> {
    // The row of buttons above the list (Bold, Italic, Underline) goes: the list has the
    // Font commands, and Markdown cannot store underline.
    menu.PrimaryCommands()?.Clear()?;
    // The menu is reused (and can be shared by the page's editors), and whether text is
    // selected changes between openings, so the app's items are taken out and put back each
    // time. They follow the menu's own items at the end of the list, so they are removed from
    // the end while they are the app's: a separator or one of its labels.
    let list = menu.SecondaryCommands()?;
    let ours = |command: &ICommandBarElement| {
        command.cast::<IAppBarSeparator>().is_ok()
            || label_of(command).is_some_and(|label| {
                FORMAT_COMMANDS.iter().any(|(name, _, _)| *name == label)
                    || STYLES.iter().chain([&QUOTE_STYLE]).any(|(name, _)| *name == label)
            })
    };
    while let Some(last) = list.Size()?.checked_sub(1) {
        if !ours(&list.GetAt(last)?) {
            break;
        }
        list.RemoveAt(last)?;
    }
    if selected {
        for item in format {
            list.Append(item)?;
        }
    }
    for item in styles {
        list.Append(item)?;
    }
    Ok(())
}

/// A separator followed by the Font and Paragraph commands, with the ribbon's icons.
fn format_items(
    id: Id,
    sender: &LocalSender<Msg>,
    parent: &IFlyoutBase,
) -> Result<Vec<ICommandBarElement>> {
    let mut items = vec![activate::<AppBarSeparator>()?.cast()?];
    for (name, icon, command) in FORMAT_COMMANDS {
        let button: AppBarButton = activate()?;
        button.SetLabel(name)?;
        button.SetIcon(&menu_icon(icon)?)?;
        let sender = sender.clone();
        let parent = parent.clone();
        let revoker = button.cast::<IButtonBase>()?.Click(move |_, _| {
            _ = parent.Hide();
            sender.send(Msg::ContextCommand(id, command));
        })?;
        // The item owns its handler for as long as the menu exists.
        std::mem::forget(revoker);
        items.push(button.cast()?);
    }
    Ok(items)
}

/// A ribbon icon for a menu item: a 20-pixel PathIcon, which the menu item's template scales
/// to its 16-pixel icon box.
fn menu_icon(data: &str) -> Result<IconElement> {
    XamlReader::Load(&format!(
        "<PathIcon xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
         Width='20' Height='20' Data='{data}'/>"
    ))?
    .cast()
}

/// The spell checker's Proofing submenu takes its theme from the editor, which the page keeps
/// light, while the rest of the menu follows the window. Its presenter is given the window's
/// theme so the whole menu matches.
fn match_submenus_to_theme(
    list: &windows_collections::IObservableVector<ICommandBarElement>,
) -> Result<()> {
    let theme = if super::theme::current().dark {
        "Dark"
    } else {
        "Light"
    };
    for index in 0..list.Size()? {
        let submenu = list
            .GetAt(index)
            .and_then(|command| command.cast::<IButton>())
            .and_then(|button| button.Flyout())
            .and_then(|flyout| flyout.cast::<IMenuFlyout>());
        if let Ok(submenu) = submenu {
            let style: Style = XamlReader::Load(&format!(
                "<Style xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
                 TargetType='MenuFlyoutPresenter'>\
                 <Setter Property='RequestedTheme' Value='{theme}'/></Style>"
            ))?
            .cast()?;
            submenu.SetMenuFlyoutPresenterStyle(&style)?;
        }
    }
    Ok(())
}

fn label_of(command: &ICommandBarElement) -> Option<String> {
    if let Ok(button) = command.cast::<IAppBarToggleButton>() {
        return button.Label().ok();
    }
    command.cast::<IAppBarButton>().ok()?.Label().ok()
}

/// A separator followed by one item per paragraph style.
fn style_items(
    id: Id,
    sender: &LocalSender<Msg>,
    parent: &IFlyoutBase,
) -> Result<Vec<ICommandBarElement>> {
    let mut items = vec![activate::<AppBarSeparator>()?.cast()?];
    for (name, kind) in STYLES.into_iter().chain([QUOTE_STYLE]) {
        let button: AppBarButton = activate()?;
        button.SetLabel(name)?;
        let sender = sender.clone();
        let parent = parent.clone();
        let revoker = button.cast::<IButtonBase>()?.Click(move |_, _| {
            _ = parent.Hide();
            sender.send(Msg::ContextCommand(id, Command::Kind(kind)));
        })?;
        // The item owns its handler for as long as the menu exists.
        std::mem::forget(revoker);
        items.push(button.cast()?);
    }
    Ok(items)
}

fn activate<T: Interface + windows_core::RuntimeName>() -> Result<T> {
    windows_core::factory::<T, windows_core::imp::IGenericFactory>()?.ActivateInstance()
}

/// The document as RTF, which changes when its formatting does.
fn formatted_text(document: &RichEditTextDocument) -> String {
    let mut text = windows_core::HSTRING::new();
    _ = document.GetText(TextGetOptions::FormatRtf, &mut text);
    text.to_string_lossy()
}

/// Shows each style item in its style's look, as the Styles box does. Its label only exists
/// once the menu is open.
fn style_style_items(menu: &ICommandBarFlyout) -> Result<()> {
    let list = menu.SecondaryCommands()?;
    for index in 0..list.Size()? {
        let command = list.GetAt(index)?;
        let Some(label) = label_of(&command) else {
            continue;
        };
        if let Some(style) = STYLES.iter().position(|(name, _)| *name == label) {
            super::styles_box::style_menu_label(&command.cast()?, style)?;
        }
    }
    Ok(())
}
