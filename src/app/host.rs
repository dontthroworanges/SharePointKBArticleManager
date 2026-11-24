//! Native `RichEditBox` hosts: the WinUI objects behind the document's text editors.
//!
//! Reactor renders each editor as an uncontrolled `RichEditBox`; once it is published the app
//! takes the native object (through the vendored `request_native_object` patch), writes the
//! initial content, and listens to its events.

use super::{Id, Msg, Shortcut};
use crate::markdown::{ListKind, Paragraph};
use crate::rich;
use crate::winui::*;
use std::rc::Rc;
use windows_core::{Interface, Result};
use windows_reactor::LocalSender;

/// Whether an editor holds formatted text flow or plain monospace text (code, Markdown).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostKind {
    Flow,
    Plain,
}

/// What an editor starts with when it is attached.
#[derive(Clone, Debug)]
pub enum Content {
    Flow(Vec<Paragraph>),
    Plain(String),
}

pub struct Host {
    pub document: RichEditTextDocument,
    editor: IRichEditBox,
    /// Handlers on the right-click menus, which exist only once the editor has loaded.
    menu_events: std::cell::RefCell<Vec<windows_core::EventRevoker>>,
    element: IUIElement,
    pub kind: HostKind,
    loaded: std::cell::Cell<bool>,
    /// The text as last seen, to tell edits from formatting passes.
    text: Rc<std::cell::RefCell<String>>,
    _events: Vec<windows_core::EventRevoker>,
}

impl Host {
    /// Scrolls the page so the text at `position` sits `ratio` of the way down the window.
    pub fn scroll_to(&self, position: i32, ratio: f64) -> Result<()> {
        let rect = rich::rect_at(&self.document, position)?;
        let options: BringIntoViewOptions =
            windows_core::factory::<BringIntoViewOptions, windows_core::imp::IGenericFactory>()?
                .ActivateInstance()?;
        // A little room above the heading, as Word leaves when it jumps to one.
        let margin = 16.0;
        options.SetTargetRect(Some(Rect {
            x: rect.x,
            y: (rect.y - margin).max(0.0),
            width: rect.width.max(1.0),
            height: rect.height + margin,
        }))?;
        options.SetVerticalAlignmentRatio(ratio)?;
        options.SetAnimationDesired(true)?;
        self.element.StartBringIntoViewWithOptions(&options)
    }

    /// WinUI resets fonts and text colors to the control's own once the editor has loaded (before
    /// its first selection event), so the content is written again at that point.
    pub fn refresh_after_load(&self) -> bool {
        if self.loaded.replace(true) {
            return false;
        }
        let result = match self.kind {
            HostKind::Flow => {
                rich::read(&self.document).and_then(|paragraphs| self.write(&paragraphs))
            }
            HostKind::Plain => self
                .document
                .GetRange(0, i32::MAX)
                .and_then(|range| range.CharacterFormat())
                .and_then(|format| format.SetName(rich::CODE_FONT)),
        };
        if let Err(error) = result {
            crate::log::log!("refreshing the editor failed: {error:?}");
        }
        true
    }

    /// Adds the paragraph styles to the editor's right-click menus. Call after it has loaded.
    pub fn attach_menus(&self, id: Id, sender: &LocalSender<Msg>) {
        if self.kind != HostKind::Flow || !self.menu_events.borrow().is_empty() {
            return;
        }
        match super::context_menu::attach(&self.editor, id, sender) {
            Ok(revokers) => *self.menu_events.borrow_mut() = revokers,
            Err(error) => crate::log::log!("could not add styles to the text menu: {error:?}"),
        }
    }

    /// Replaces the content without the change counting as an edit.
    pub fn write(&self, paragraphs: &[Paragraph]) -> Result<()> {
        let result = rich::write(&self.document, paragraphs);
        *self.text.borrow_mut() = document_text(&self.document);
        result
    }

    /// Replaces the content after a paste. The paste opened an undo group (see `attach`), which
    /// this closes, so one Ctrl+Z takes back both the paste and this clean-up.
    pub fn rewrite_after_paste(&self, paragraphs: &[Paragraph]) -> Result<()> {
        let result = rich::rewrite(&self.document, paragraphs);
        *self.text.borrow_mut() = document_text(&self.document);
        self.document.EndUndoGroup()?;
        result
    }

    pub fn paragraphs(&self) -> Vec<Paragraph> {
        rich::read(&self.document).unwrap_or_default()
    }

    pub fn plain_text(&self) -> String {
        let mut text = windows_core::HSTRING::new();
        if self
            .document
            .GetText(TextGetOptions::None, &mut text)
            .is_err()
        {
            return String::new();
        }
        let text = text
            .to_string_lossy()
            .replace("\r\n", "\n")
            .replace('\r', "\n");
        text.strip_suffix('\n').unwrap_or(&text).to_string()
    }
}

/// XAML raises these events on the UI thread that owns the editor, which is the thread that
/// runs the Reactor component, so the non-`Send` sender is only ever used there.
struct UiThread<T>(T);
// SAFETY: see the type documentation; the wrapped value never leaves the UI thread.
unsafe impl<T> Send for UiThread<T> {}

pub fn attach(
    object: windows_core::IUnknown,
    id: Id,
    content: &Content,
    sender: LocalSender<Msg>,
) -> Result<Rc<Host>> {
    let editor: IRichEditBox = object.cast()?;
    let document = editor.TextDocument()?;
    editor.SetDisabledFormattingAccelerators(DisabledFormattingAccelerators::Underline)?;
    let kind = match content {
        Content::Flow(paragraphs) => {
            rich::write(&document, paragraphs)?;
            HostKind::Flow
        }
        Content::Plain(text) => {
            editor.SetIsSpellCheckEnabled(false)?;
            let format = document.GetDefaultCharacterFormat()?;
            format.SetName(rich::CODE_FONT)?;
            format.SetSize(10.0)?;
            document.SetDefaultCharacterFormat(&format)?;
            document.SetText(TextSetOptions::None, text)?;
            document
                .GetRange(0, i32::MAX)?
                .SetCharacterFormat(&format)?;
            document.ClearUndoRedoHistory()?;
            HostKind::Plain
        }
    };

    let mut events = Vec::new();
    let send = UiThread(sender.clone());
    events.push(editor.SelectionChanged(move |_, _| {
        send.0.send(Msg::Selection(id));
    })?);
    let text = Rc::new(std::cell::RefCell::new(document_text(&document)));
    let send = UiThread((sender.clone(), Rc::clone(&text), document.clone()));
    events.push(editor.TextChanged(move |_, _| {
        // Formatting passes (loading, re-reading after a paste) raise change events but leave
        // the text alone; only a change to the text counts as an edit here. Formatting edits
        // come through ribbon commands and shortcuts, which mark the page edited themselves.
        let (sender, last, document) = &send.0;
        let current = document_text(document);
        if *last.borrow() != current {
            *last.borrow_mut() = current;
            sender.send(Msg::Changed(id));
        }
    })?);
    if kind == HostKind::Flow {
        let send = UiThread((sender.clone(), document.clone()));
        events.push(editor.Paste(move |_, _| {
            // Runs before the paste; the message is handled after it lands. The undo group
            // spans the paste and the clean-up that follows it.
            let (sender, document) = &send.0;
            _ = document.BeginUndoGroup();
            sender.send(Msg::Pasted(id));
        })?);
    }
    let key_document = document.clone();
    let element: IUIElement = editor.cast()?;
    events.push(element.PreviewKeyDown(move |_, args| {
        let Some(args) = args.as_ref() else { return };
        let Ok(args) = args.cast::<IKeyRoutedEventArgs>() else {
            return;
        };
        let Ok(key) = args.Key() else { return };
        if key == VirtualKey::Enter
            && kind == HostKind::Flow
            && !is_down(VirtualKey::Shift)
            && !is_down(VirtualKey::Control)
            && (rich::enter_after_heading(&key_document).unwrap_or(false)
                || rich::enter_in_list(&key_document).unwrap_or(false))
        {
            _ = args.SetHandled(true);
            sender.send(Msg::Selection(id));
            return;
        }
        let control = is_down(VirtualKey::Control);
        let shift = is_down(VirtualKey::Shift);
        let alt = is_down(VirtualKey::Menu);
        // Ctrl+Shift+Z redoes, as in most editors; RichEdit itself only knows Ctrl+Y.
        if control && shift && !alt && key == VirtualKey::Z {
            _ = args.SetHandled(true);
            _ = key_document.Redo();
            sender.send(Msg::Edited(id));
            return;
        }
        if blocked_formatting(key, control, shift, alt) {
            _ = args.SetHandled(true);
            return;
        }
        // Native formatting and undo shortcuts change the page without changing its text.
        if control
            && !alt
            && [VirtualKey::B, VirtualKey::I, VirtualKey::Z, VirtualKey::Y].contains(&key)
        {
            sender.send(Msg::Edited(id));
        }
        let in_list = kind == HostKind::Flow
            && rich::selection_state(&key_document).is_ok_and(|state| state.list.is_some());
        if let Some(shortcut) = shortcut(key, in_list, kind) {
            _ = args.SetHandled(true);
            sender.send(Msg::Shortcut(id, shortcut));
        }
    })?);

    Ok(Rc::new(Host {
        document,
        editor,
        menu_events: std::cell::RefCell::default(),
        element,
        kind,
        loaded: std::cell::Cell::new(false),
        text,
        _events: events,
    }))
}

#[link(name = "user32")]
unsafe extern "system" {
    fn GetKeyState(key: i32) -> i16;
}

fn is_down(key: VirtualKey) -> bool {
    // SAFETY: GetKeyState has no preconditions.
    unsafe { GetKeyState(key.0) < 0 }
}

/// The + and - keys of the main keyboard, which VirtualKey does not name.
const VK_OEM_PLUS: i32 = 0xBB;
const VK_OEM_MINUS: i32 = 0xBD;

const VK_OEM_COMMA: i32 = 0xBC;
const VK_OEM_PERIOD: i32 = 0xBE;
const VK_OEM_OPEN_BRACKET: i32 = 0xDB;
const VK_OEM_CLOSE_BRACKET: i32 = 0xDD;

/// RichEdit's own formatting keys for things Markdown cannot store: paragraph alignment
/// (Ctrl+L/E/R/J), line spacing (Ctrl+1/2/5), font size (Ctrl+[ ], Ctrl+Shift+< >) and all caps
/// (Ctrl+Shift+A). The editor ignores them, as it does Underline, so the page never shows
/// formatting that would be lost on saving.
fn blocked_formatting(key: VirtualKey, control: bool, shift: bool, alt: bool) -> bool {
    if !control || alt {
        return false;
    }
    let plain = [
        VirtualKey::L,
        VirtualKey::E,
        VirtualKey::R,
        VirtualKey::J,
        VirtualKey::Number1,
        VirtualKey::Number2,
        VirtualKey::Number5,
    ];
    if shift {
        key == VirtualKey::A || key.0 == VK_OEM_COMMA || key.0 == VK_OEM_PERIOD
    } else {
        plain.contains(&key) || key.0 == VK_OEM_OPEN_BRACKET || key.0 == VK_OEM_CLOSE_BRACKET
    }
}

/// Shortcuts that work wherever focus is in the window (the editors, the ribbon, the panes).
pub fn app_shortcut(key: VirtualKey) -> Option<Shortcut> {
    let shift = is_down(VirtualKey::Shift);
    let control = is_down(VirtualKey::Control);
    if is_down(VirtualKey::Menu) {
        return None;
    }
    // Function keys, as in Word and Notepad.
    match key {
        VirtualKey::F3 if !control => {
            return Some(if shift {
                Shortcut::FindPrevious
            } else {
                Shortcut::FindNext
            });
        }
        VirtualKey::F12 if control && !shift => return Some(Shortcut::Open),
        VirtualKey::F12 if shift && !control => return Some(Shortcut::Save),
        VirtualKey::F12 if !control => return Some(Shortcut::SaveAs),
        _ => {}
    }
    if !control {
        return None;
    }
    Some(match key {
        VirtualKey::S if shift => Shortcut::SaveAs,
        VirtualKey::S => Shortcut::Save,
        VirtualKey::O if !shift => Shortcut::Open,
        VirtualKey::N if !shift => Shortcut::New,
        VirtualKey::F if !shift => Shortcut::Find,
        VirtualKey::H if !shift => Shortcut::Replace,
        // Ctrl with + or - on the main keyboard (also =, the unshifted +) or the number pad.
        key if key.0 == VK_OEM_PLUS || key == VirtualKey::Add => Shortcut::ZoomIn,
        key if key.0 == VK_OEM_MINUS || key == VirtualKey::Subtract => Shortcut::ZoomOut,
        VirtualKey::Number0 | VirtualKey::NumberPad0 if !shift => Shortcut::ZoomReset,
        _ => return None,
    })
}

/// Hooks the window's root element so `app_shortcut`s are seen before any control handles them.
pub fn hook_root(object: windows_core::IUnknown, sender: LocalSender<Msg>) -> Result<()> {
    let element: IUIElement = object.cast()?;
    let revoker = element.PreviewKeyDown(move |_, args| {
        let Some(args) = args.as_ref() else { return };
        let Ok(args) = args.cast::<IKeyRoutedEventArgs>() else {
            return;
        };
        if let Some(shortcut) = args.Key().ok().and_then(app_shortcut) {
            _ = args.SetHandled(true);
            sender.send(Msg::AppShortcut(shortcut));
        }
    })?;
    // The root lives as long as the window, and so does its handler.
    std::mem::forget(revoker);
    Ok(())
}

/// Word-compatible keyboard shortcuts the app handles itself.
fn shortcut(key: VirtualKey, in_list: bool, kind: HostKind) -> Option<Shortcut> {
    let control = is_down(VirtualKey::Control);
    let shift = is_down(VirtualKey::Shift);
    let alt = is_down(VirtualKey::Menu);
    let flow = kind == HostKind::Flow;
    Some(match key {
        VirtualKey::N if control && shift && flow => Shortcut::Normal,
        VirtualKey::K if control && flow => Shortcut::Link,
        VirtualKey::L if control && shift && flow => Shortcut::List(ListKind::Bullet),
        VirtualKey::H if control && shift && flow => Shortcut::Highlight,
        VirtualKey::X if control && shift && flow => Shortcut::Strikethrough,
        VirtualKey::C if control && alt && flow => Shortcut::Code,
        key if control
            && alt
            && flow
            && (VirtualKey::Number1.0..=VirtualKey::Number6.0).contains(&key.0) =>
        {
            Shortcut::Heading((key.0 - VirtualKey::Number0.0) as u8)
        }
        VirtualKey::Tab if in_list && !control && !alt => {
            if shift {
                Shortcut::Outdent
            } else {
                Shortcut::Indent
            }
        }
        _ => return None,
    })
}

impl std::fmt::Debug for Host {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Host")
            .field("kind", &self.kind)
            .finish()
    }
}

fn document_text(document: &RichEditTextDocument) -> String {
    let mut text = windows_core::HSTRING::new();
    _ = document.GetText(TextGetOptions::None, &mut text);
    text.to_string_lossy()
}
