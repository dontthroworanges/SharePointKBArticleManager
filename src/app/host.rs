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
    /// The hand pointer and address tooltip over links, for a text flow.
    link_hover: Option<LinkHover>,
}

impl Host {
    /// Scrolls the page so the text at `position` sits `ratio` of the way down the window.
    pub fn scroll_to(&self, position: i32, ratio: f64) -> Result<()> {
        let mut rect = rich::rect_at(&self.document, position)?;
        // The rectangle starts above the paragraph's space before (points; here pixels), so
        // without it every heading lands at the same place whatever is above it.
        let before = self
            .document
            .GetRange(position, position)?
            .ParagraphFormat()?
            .SpaceBefore()?
            * 96.0
            / 72.0;
        rect.y += before;
        rect.height = (rect.height - before).max(1.0);
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

    /// Scrolls the page as little as needed to keep `room` (in page units) of blank space
    /// below the caret, so typing does not happen at the very bottom of the window.
    pub fn keep_room_below_caret(&self, room: f64) -> Result<()> {
        let (_, end) = rich::caret(&self.document)?;
        let rect = rich::rect_at(&self.document, end)?;
        let options: BringIntoViewOptions =
            windows_core::factory::<BringIntoViewOptions, windows_core::imp::IGenericFactory>()?
                .ActivateInstance()?;
        // The caret's line plus the room under it; with no alignment ratio, WinUI scrolls only
        // when part of that is out of view.
        options.SetTargetRect(Some(Rect {
            x: rect.x,
            y: rect.y,
            width: rect.width.max(1.0),
            height: rect.height + room as f32,
        }))?;
        options.SetAnimationDesired(false)?;
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

    /// Replaces a plain (Markdown) editor's text, in its code font, with a fresh undo history,
    /// without the change counting as an edit.
    pub fn write_plain(&self, text: &str) -> Result<()> {
        // Set here, not taken from the editor's default: WinUI resets that once it has loaded.
        let format = self.document.GetDefaultCharacterFormat()?;
        format.SetName(rich::CODE_FONT)?;
        format.SetSize(rich::plain_size())?;
        let result = self
            .document
            .SetText(TextSetOptions::None, text)
            .and_then(|()| self.document.GetRange(0, i32::MAX))
            .and_then(|range| range.SetCharacterFormat(&format))
            .and_then(|()| self.document.ClearUndoRedoHistory());
        *self.text.borrow_mut() = document_text(&self.document);
        result
    }

    /// Replaces the content without the change counting as an edit.
    pub fn write(&self, paragraphs: &[Paragraph]) -> Result<()> {
        let result = rich::write(&self.document, paragraphs);
        *self.text.borrow_mut() = document_text(&self.document);
        result
    }

    /// Replaces the content after a paste. The paste opened an undo group (see `attach`), which
    /// this closes, so one Ctrl+Z takes back both the paste and this clean-up.
    /// Returns where the caret's text now is.
    pub fn rewrite_after_paste(&self, paragraphs: &[Paragraph], caret: i32) -> Result<i32> {
        let result = rich::rewrite(&self.document, paragraphs, caret);
        *self.text.borrow_mut() = document_text(&self.document);
        self.document.EndUndoGroup()?;
        result
    }

    /// The size, in points, the next typed character gets (the caret's own format), when
    /// nothing is selected. Read before a zoom, for `fit_zoom`.
    pub fn typing_size(&self) -> Option<f32> {
        let (start, end) = rich::caret(&self.document).ok()?;
        if start != end {
            return None;
        }
        self.document
            .Selection()
            .and_then(|selection| selection.cast::<ITextRange>())
            .and_then(|selection| selection.CharacterFormat())
            .and_then(|format| format.Size())
            .ok()
    }

    /// Finishes a zoom from `from` to `to` after the text has been rescaled (see
    /// `rich::rescale`).
    ///
    /// - The control's own font size becomes the zoomed default (Normal text, or code in a
    ///   plain editor). Changing it is also what has WinUI measure the editor again: without
    ///   that, an editor without focus kept the height it had, so shrunk text left it a line
    ///   short with its last line cut off (invalidating its measure did not help).
    /// - Changing it also resets the caret's own format, which the next typed character gets,
    ///   to that size; `typing_size` (read before the zoom) puts it back, rescaled, so typing
    ///   in a heading, say, keeps the heading's size.
    pub fn fit_zoom(&self, typing_size: Option<f32>, from: f32, to: f32) {
        let points = match self.kind {
            HostKind::Flow => rich::body_size(),
            HostKind::Plain => rich::PLAIN_SIZE,
        } * to;
        let fitted = self
            .editor
            .cast::<IControl>()
            // The control's size is in pixels.
            .and_then(|control| control.SetFontSize(f64::from(points) * 96.0 / 72.0))
            .and_then(|()| match typing_size {
                Some(size) => self
                    .document
                    .Selection()?
                    .cast::<ITextRange>()?
                    .CharacterFormat()?
                    .SetSize(rich::rezoomed(size, from, to)),
                None => Ok(()),
            });
        if let Err(error) = fitted {
            crate::log::log!("fitting the editor to the zoom failed: {error:?}");
        }
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
    // The editor grows with its text and the page scrolls, never the editor itself. Left to
    // scroll, a key that adds a last line had RichEdit scroll the editor's own content up to
    // show the caret, before WinUI made the editor taller and the content dropped back: the
    // text jumped up and down when typing at the end.
    disable_own_scrolling(&editor)?;
    let kind = match content {
        Content::Flow(paragraphs) => {
            // Diagnostic: KB_NO_SPELL=1 turns off the editor's own spell checking (the red
            // underlines), to measure what it costs while typing.
            if crate::log::switch("KB_NO_SPELL") {
                editor.SetIsSpellCheckEnabled(false)?;
            }
            rich::write(&document, paragraphs)?;
            HostKind::Flow
        }
        Content::Plain(text) => {
            editor.SetIsSpellCheckEnabled(false)?;
            let format = document.GetDefaultCharacterFormat()?;
            format.SetName(rich::CODE_FONT)?;
            format.SetSize(rich::plain_size())?;
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
        let started = std::time::Instant::now();
        let current = document_text(document);
        crate::log::log!(
            "TextChanged: read {} chars in {:.2} ms",
            current.len(),
            started.elapsed().as_secs_f64() * 1000.0
        );
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
    if kind == HostKind::Flow {
        // The rules under headings follow the text's layout: its first layout, and any change
        // in width or height (wrapping, or text above a heading growing or shrinking).
        let send = UiThread(sender.clone());
        events.push(editor.cast::<IFrameworkElement>()?.SizeChanged(move |_, _| {
            send.0.send(Msg::Resized(id));
        })?);
    }
    // Without it, links still work; they only lack the hand and the tooltip.
    let link_hover = if kind == HostKind::Flow {
        LinkHover::new(&editor)
            .inspect_err(|error| crate::log::log!("link hover is unavailable: {error:?}"))
            .ok()
    } else {
        None
    };
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
        link_hover,
    }))
}

/// Sets `ScrollViewer.VerticalScrollBarVisibility` to Disabled on the editor, which its
/// template passes to the ScrollViewer inside it: that one can then not scroll at all (the
/// editor still grows to fit its text). The ScrollViewer class is left out of the bindings,
/// whose filter would otherwise pull in its whole interface, so its statics are fetched by
/// name.
fn disable_own_scrolling(editor: &IRichEditBox) -> Result<()> {
    struct ScrollViewerClass;
    impl windows_core::RuntimeName for ScrollViewerClass {
        const NAME: &'static str = "Microsoft.UI.Xaml.Controls.ScrollViewer";
    }
    let statics = windows_core::factory::<ScrollViewerClass, IScrollViewerStatics>()?;
    let target: DependencyObject = editor.cast()?;
    // SAFETY: a call through the statics' vtable with live interface pointers.
    unsafe {
        (Interface::vtable(&statics).SetVerticalScrollBarVisibility)(
            Interface::as_raw(&statics),
            Interface::as_raw(&target),
            ScrollBarVisibility::Disabled,
        )
        .ok()
    }
}

/// Over a link, the pointer is a hand and a tooltip gives the link's address, as in a web
/// page. RichEdit itself shows the I-beam over links and says nothing about where they go.
/// Driven by `watch_pointer`, as the editor's text raises no pointer events of its own.
struct LinkHover {
    editor: UIElement,
    target: DependencyObject,
    cursor: IUIElementProtected,
    /// Whether the pointer is the hand.
    hand: std::cell::Cell<bool>,
    /// The link whose address is showing, and its tooltip.
    shown: std::cell::RefCell<Option<(String, ToolTip)>>,
    /// A link the pointer rests on, where it came onto it, and the ticks left before its
    /// address shows.
    pending: std::cell::RefCell<Option<(String, Point, u32)>>,
}

/// Ticks (see `POINTER_TICK`) the pointer rests on a link before its address shows, as
/// tooltips wait, so passing over links does not flash addresses (and one tooltip has closed
/// before the next opens).
const TOOLTIP_DELAY: u32 = 6;

impl LinkHover {
    fn new(editor: &IRichEditBox) -> Result<Self> {
        Ok(Self {
            editor: editor.cast()?,
            target: editor.cast()?,
            cursor: editor.cast()?,
            hand: std::cell::Cell::new(false),
            shown: std::cell::RefCell::default(),
            pending: std::cell::RefCell::default(),
        })
    }

    /// Whether an address is waiting to show, so the watch keeps ticking.
    fn waiting(&self) -> bool {
        self.pending.borrow().is_some()
    }

    /// The pointer is on `link` at `at` (in the editor), or on no link: the hand at once, the
    /// address below the pointer once it has rested there. The tooltip stays where it opened
    /// while the pointer stays on the same link, as in a browser.
    fn show(&self, link: Option<String>, at: Point) -> Result<()> {
        let hand = link.is_some();
        if self.hand.replace(hand) != hand {
            let shape = if hand {
                InputSystemCursorShape::Hand
            } else {
                InputSystemCursorShape::IBeam
            };
            self.cursor
                .SetProtectedCursor(&InputSystemCursor::Create(shape)?)?;
        }
        if self.shown.borrow().as_ref().map(|(text, _)| text) == link.as_ref() {
            return Ok(());
        }
        if let Some((_, tooltip)) = self.shown.borrow_mut().take() {
            tooltip.SetIsOpen(false)?;
            ToolTipService::SetToolTip(&self.target, None::<&windows_core::IInspectable>)?;
        }
        let Some(text) = link else {
            *self.pending.borrow_mut() = None;
            return Ok(());
        };
        let mut pending = self.pending.borrow_mut();
        match pending.as_mut() {
            Some((waiting, _, left)) if *waiting == text && *left > 0 => *left -= 1,
            Some((waiting, from, _)) if *waiting == text => {
                let from = *from;
                *pending = None;
                drop(pending);
                self.open(text, from)?;
            }
            _ => *pending = Some((text, at, TOOLTIP_DELAY)),
        }
        Ok(())
    }

    /// Opens a tooltip with `text` below `at`.
    fn open(&self, text: String, at: Point) -> Result<()> {
        // Below the pointer, clear of it. XAML does not know where the pointer is (the text
        // keeps pointer input to itself), so a Mouse placement would not follow it. The
        // rectangle is set in the markup: setting it in code passes a boxed Rect that XAML
        // cannot read.
        let tooltip: ToolTip = XamlReader::Load(&format!(
            "<ToolTip xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation' \
             Placement='Bottom' PlacementRect='{},{},1,{POINTER_HEIGHT}'>{}</ToolTip>",
            at.x,
            at.y,
            xml_escape(&text)
        ))?
        .cast()?;
        ToolTipService::SetToolTip(&self.target, &tooltip)?;
        tooltip.SetIsOpen(true)?;
        *self.shown.borrow_mut() = Some((text, tooltip));
        Ok(())
    }
}

impl Host {
    /// Shows the link under `point`, in physical pixels from the window's client area, if any.
    fn hover(&self, point: (i32, i32)) -> Result<()> {
        let Some(hover) = &self.link_hover else {
            return Ok(());
        };
        // The window's content fills its client area (the title bar is drawn by the app), in
        // pixels scaled by the display's scale.
        let root = hover.editor.XamlRoot()?;
        let scale = root.RasterizationScale()? as f32;
        let point = Point {
            x: point.0 as f32 / scale,
            y: point.1 as f32 / scale,
        };
        let local = root
            .Content()?
            .TransformToVisual(&hover.editor)?
            .TransformPoint(point)?;
        hover.show(rich::link_at(&self.document, local.x, local.y)?, local)
    }

    /// Takes away the hand and the tooltip.
    fn unhover(&self) {
        if let Some(hover) = &self.link_hover {
            _ = hover.show(None, Point::default());
        }
    }

    /// Whether a link's address is waiting to show.
    fn hover_waiting(&self) -> bool {
        self.link_hover.as_ref().is_some_and(LinkHover::waiting)
    }
}

/// What the pointer watch needs between ticks (see `watch_pointer`).
struct PointerWatch {
    window: Box<dyn Fn() -> isize>,
    flows: Rc<std::cell::RefCell<Vec<std::rc::Weak<Host>>>>,
    visual: Rc<std::cell::Cell<bool>>,
    /// Where the pointer was at the last tick, in screen pixels.
    last: std::cell::Cell<Option<(i32, i32)>>,
}

thread_local! {
    static POINTER_WATCH: std::cell::RefCell<Option<PointerWatch>> = const { std::cell::RefCell::new(None) };
}

/// How often the pointer is looked at, in milliseconds: often enough that the hand and the
/// tooltip follow it without a visible lag.
const POINTER_TICK: u32 = 50;

/// Room for the pointer below where it points, so the tooltip opens clear of it.
const POINTER_HEIGHT: f32 = 20.0;

/// Watches the pointer over the window for the text flows' links (see `LinkHover`). The
/// editor's text takes pointer input inside RichEdit, before XAML routes it, so no XAML event
/// (nor the window's input source) reports movement over text. So a timer on this (the UI)
/// thread looks at where the pointer is and, when it has moved, at what is under it.
/// `window` is the app's window (0 until it is known); `flows` the text flows' hosts; `visual`
/// whether the visual page is showing (not the Markdown view or Settings). Runs for the life
/// of the thread; call once.
pub fn watch_pointer(
    window: Box<dyn Fn() -> isize>,
    flows: Rc<std::cell::RefCell<Vec<std::rc::Weak<Host>>>>,
    visual: Rc<std::cell::Cell<bool>>,
) {
    POINTER_WATCH.with(|watch| {
        *watch.borrow_mut() = Some(PointerWatch {
            window,
            flows,
            visual,
            last: std::cell::Cell::new(None),
        });
    });
    // SAFETY: a thread timer (no window) with a callback that runs on this thread's message
    // loop; the callback only reads the thread-local state set above.
    unsafe { SetTimer(0, 0, POINTER_TICK, Some(pointer_tick)) };
    if crate::log::enabled() {
        // SAFETY: as above; the callback only touches its own thread-local.
        unsafe { SetTimer(0, 0, STALL_TICK, Some(stall_tick)) };
    }
}

/// Diagnostic, while timing is logged: how often the UI thread is checked for stalls, in
/// milliseconds, and the gap between checks worth reporting. A timer message is only made
/// when the thread gets back to its message loop, so a late tick means the thread was busy:
/// in this app's code, in Reactor, or in WinUI's own layout and text work.
const STALL_TICK: u32 = 10;
const STALL_REPORT_MS: f64 = 40.0;

thread_local! {
    static LAST_STALL_TICK: std::cell::Cell<Option<std::time::Instant>> =
        const { std::cell::Cell::new(None) };
}

extern "system" fn stall_tick(_: isize, _: u32, _: usize, _: u32) {
    let now = std::time::Instant::now();
    if let Some(last) = LAST_STALL_TICK.replace(Some(now)) {
        let gap = now.duration_since(last).as_secs_f64() * 1000.0;
        if gap > STALL_REPORT_MS {
            crate::log::log!("UI thread busy: {gap:.0} ms without getting to its message loop");
        }
    }
}

extern "system" fn pointer_tick(_: isize, _: u32, _: usize, _: u32) {
    POINTER_WATCH.with(|watch| {
        let Ok(watch) = watch.try_borrow() else { return };
        let Some(watch) = watch.as_ref() else { return };
        let window = (watch.window)();
        let mut point = WinPoint { x: 0, y: 0 };
        // SAFETY: plain Win32 calls with a valid out pointer.
        if window == 0 || unsafe { GetCursorPos(&mut point) } == 0 {
            return;
        }
        let mut flows = watch.flows.borrow_mut();
        flows.retain(|host| host.strong_count() > 0);
        // Nothing to do while the pointer rests, unless an address is waiting to show.
        let still = watch.last.replace(Some((point.x, point.y))) == Some((point.x, point.y));
        if still
            && !flows
                .iter()
                .filter_map(std::rc::Weak::upgrade)
                .any(|host| host.hover_waiting())
        {
            return;
        }
        // Over this window (not a window in front of it), in its client area's pixels.
        // SAFETY: plain Win32 calls; `window` is this app's window.
        let over = unsafe { GetAncestor(WindowFromPoint(point), GA_ROOT) } == window
            && unsafe { ScreenToClient(window, &mut point) } != 0;
        for host in flows.iter().filter_map(std::rc::Weak::upgrade) {
            if over && watch.visual.get() {
                if let Err(error) = host.hover((point.x, point.y)) {
                    crate::log::log!("showing a link's address failed: {error:?}");
                }
            } else {
                host.unhover();
            }
        }
    });
}

/// How long typing must pause, in milliseconds, before `Msg::Idle` is sent.
const IDLE_DELAY: u32 = 250;

thread_local! {
    /// The running idle timer and where to send `Msg::Idle` when it fires.
    static IDLE: std::cell::RefCell<Option<(usize, LocalSender<Msg>)>> =
        const { std::cell::RefCell::new(None) };
}

/// Sends `Msg::Idle` once no key has changed the text for `IDLE_DELAY`: each call restarts the
/// wait. Lets work that reads the whole document run once per pause instead of once per key.
pub fn after_typing(sender: &LocalSender<Msg>) {
    IDLE.with(|idle| {
        let mut idle = idle.borrow_mut();
        if let Some((timer, _)) = idle.take() {
            // SAFETY: a thread timer this thread made.
            unsafe { KillTimer(0, timer) };
        }
        // SAFETY: a thread timer (no window), as in `watch_pointer`; with no window the id is
        // chosen by Windows, so the old timer is killed above rather than replaced.
        let timer = unsafe { SetTimer(0, 0, IDLE_DELAY, Some(idle_tick)) };
        if timer == 0 {
            sender.send(Msg::Idle);
        } else {
            *idle = Some((timer, sender.clone()));
        }
    });
}

extern "system" fn idle_tick(_: isize, _: u32, timer: usize, _: u32) {
    // SAFETY: the timer is this thread's; a thread timer repeats until killed.
    unsafe { KillTimer(0, timer) };
    let sender = IDLE.with(|idle| {
        let mut idle = idle.borrow_mut();
        match idle.as_ref() {
            Some((current, _)) if *current == timer => idle.take().map(|(_, sender)| sender),
            _ => None,
        }
    });
    // Sent with the borrow released, in case the message is handled at once.
    if let Some(sender) = sender {
        sender.send(Msg::Idle);
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WinPoint {
    x: i32,
    y: i32,
}

const GA_ROOT: u32 = 2;

#[link(name = "user32")]
unsafe extern "system" {
    fn SetTimer(
        window: isize,
        id: usize,
        elapse: u32,
        callback: Option<extern "system" fn(isize, u32, usize, u32)>,
    ) -> usize;
    fn KillTimer(window: isize, id: usize) -> i32;
    fn GetCursorPos(point: *mut WinPoint) -> i32;
    fn WindowFromPoint(point: WinPoint) -> isize;
    fn GetAncestor(window: isize, flags: u32) -> isize;
    fn ScreenToClient(window: isize, point: *mut WinPoint) -> i32;
}

/// `text` for the content of a XAML element.
fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
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
/// Any key also ends a Navigation pane jump (see `hook_page`): from then on the page scrolls
/// to follow the caret as usual.
pub fn hook_root(
    object: windows_core::IUnknown,
    sender: LocalSender<Msg>,
    navigating: Rc<std::cell::Cell<bool>>,
) -> Result<()> {
    let element: IUIElement = object.cast()?;
    let navigating = UiThread(navigating);
    let revoker = element.PreviewKeyDown(move |_, args| {
        navigating.0.set(false);
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

/// Hooks the page (its Grid), which sees events on their way up from the editors before the
/// ScrollViewer around it acts on them. The handlers live as long as the returned revokers.
///
/// - Ctrl+mouse wheel zooms, as in Word: sends the wheel's movement (120 a notch; up is
///   positive) and stops the scroll.
/// - While `navigating` (a Navigation pane heading was chosen, until the next key), the scroll
///   that focusing an editor asks for is dropped. It would bring the whole editor into view,
///   undoing the scroll that puts the heading at the top. That one is told apart by asking
///   for an alignment (see `Host::scroll_to`); focus asks for none.
pub fn hook_page(
    object: windows_core::IUnknown,
    sender: LocalSender<Msg>,
    navigating: Rc<std::cell::Cell<bool>>,
) -> Result<Vec<windows_core::EventRevoker>> {
    let element: IUIElement = object.cast()?;
    let page: UIElement = object.cast()?;
    let bring = UiThread(navigating);
    let into_view = element.BringIntoViewRequested(move |_, args| {
        let Some(args) = args.as_ref() else { return };
        // Focus asks for no alignment; `scroll_to` (Navigation, search) does.
        if !args.VerticalAlignmentRatio().is_ok_and(f64::is_nan) {
            return;
        }
        if bring.0.get() || clicked_into(args) {
            _ = args.SetHandled(true);
        }
    })?;
    let send = UiThread((sender, page));
    let wheel = element.PointerWheelChanged(move |_, args| {
        let (sender, page) = &send.0;
        let Some(args) = args.as_ref() else { return };
        if !is_down(VirtualKey::Control) {
            return;
        }
        let delta = args
            .GetCurrentPoint(page)
            .and_then(|point| point.Properties())
            .and_then(|properties| properties.MouseWheelDelta());
        if let Ok(delta) = delta {
            _ = args.SetHandled(true);
            sender.send(Msg::WheelZoom(delta));
        }
    })?;
    Ok(vec![into_view, wheel])
}

/// Whether a request to bring a whole element into view comes from clicking it, which gives
/// it focus. Clicking an editor taller than the window had the page scroll to the editor's
/// top while the button was still down, so the text moved under the pointer and the click
/// became a drag that selected it. A clicked element is already in view, so the request is
/// not needed. The caret's own requests (following a drag past the window's edge, say) are
/// for a line, not the whole editor, and are kept.
fn clicked_into(args: &BringIntoViewRequestedEventArgs) -> bool {
    if !is_down(VirtualKey::LeftButton) {
        return false;
    }
    let (Ok(rect), Ok(target)) = (args.TargetRect(), args.TargetElement()) else {
        return false;
    };
    let Ok(height) = target
        .cast::<IFrameworkElement>()
        .and_then(|target| target.ActualHeight())
    else {
        return false;
    };
    rect.y <= 1.0 && f64::from(rect.height) >= height - 1.0
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
