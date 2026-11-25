//! Bridge between the text-flow paragraphs of the model and a WinUI `RichEditBox`, through the
//! RichEdit text object model (TOM).
//!
//! Formatting conventions inside the editor:
//! - Headings use the TOM heading paragraph styles; size, weight and color are set to match.
//! - Quotes are paragraphs with a right indent (and matching left indent).
//! - List nesting is the left indent in steps of [`LEVEL_INDENT`].
//! - Task items are paragraphs that start with a ☐/☑ glyph.
//! - Inline code and math are identified by font; highlight by background color.
//! - `[[Page]]` links are hyperlinks to `kbpage:Page`.

use crate::markdown::*;
use crate::winui::*;
// The Markdown inline style, not the XAML Style that the WinUI bindings also define.
use crate::markdown::Style;
use windows_core::{Interface, Result};

pub const BODY_FONT: &str = "Segoe UI";
pub const CODE_FONT: &str = "Cascadia Mono";
pub const MATH_FONT: &str = "Cambria Math";
const SYMBOL_FONT: &str = "Segoe UI Symbol";
/// Normal text's size in points when Settings › Options › Paragraph font size is not set:
/// 14px (10.5pt), as SharePoint shows it.
pub const DEFAULT_BODY_SIZE: f32 = 10.5;
/// SharePoint's headings 1 to 4: 28, 24, 20 and 18px. Levels 5 and 6 are at body size.
const HEADING_SIZES: [f32; 4] = [21.0, 18.0, 15.0, 13.5];
/// Each list level, and the hanging indent of an item: its marker sits in the hang and its
/// text (wrapped lines and line breaks too) lines up after it, with the next level's marker
/// under it, as in Word.
const LEVEL_INDENT: f32 = 18.0;
const LIST_HANG: f32 = 18.0;
const QUOTE_INDENT: f32 = 24.0;
const TASK_OPEN: char = '\u{2610}';
/// 🗹 rather than ☑, which RichEdit draws as a color emoji.
const TASK_DONE: char = '\u{1F5F9}';
const PAGE_SCHEME: &str = "kbpage:";

fn task_prefix(checked: bool) -> String {
    // A tab, not a space: it reaches the hanging indent, so the text lines up exactly.
    format!("{}\t", if checked { TASK_DONE } else { TASK_OPEN })
}

/// Splits a task glyph prefix off paragraph text, returning whether the task is checked.
fn strip_task_prefix(text: &str) -> Option<(bool, &str)> {
    let rest = text.strip_prefix([TASK_OPEN, TASK_DONE, '\u{2611}', '\u{2612}'])?;
    let rest = rest.strip_prefix('\u{FE0E}').unwrap_or(rest);
    Some((
        !text.starts_with(TASK_OPEN),
        rest.strip_prefix([' ', '\t']).unwrap_or(rest),
    ))
}

/// The colors text is drawn in on the page. The light set is SharePoint's default (Fluent)
/// theme for modern pages; the dark one is for View › Switch Background, as in Word.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageColors {
    /// Body text and headings (SharePoint's neutralPrimary).
    pub text: Color,
    pub quote: Color,
    pub highlight: Color,
    pub code_background: Color,
    pub math_background: Color,
    /// Links (SharePoint's themePrimary).
    pub link: Color,
}

pub const LIGHT_PAGE: PageColors = PageColors {
    text: rgb(0x32, 0x31, 0x30),
    quote: rgb(0x60, 0x5E, 0x5C),
    highlight: rgb(0xFF, 0xF1, 0x76),
    code_background: rgb(0xF3, 0xF2, 0xF1),
    math_background: rgb(0xEE, 0xF0, 0xFF),
    link: rgb(0x00, 0x78, 0xD4),
};

pub const DARK_PAGE: PageColors = PageColors {
    text: rgb(0xF3, 0xF2, 0xF1),
    quote: rgb(0xC8, 0xC6, 0xC4),
    highlight: rgb(0x6B, 0x5E, 0x00),
    code_background: rgb(0x32, 0x31, 0x30),
    math_background: rgb(0x2B, 0x2E, 0x45),
    link: rgb(0x47, 0x9E, 0xF5),
};

thread_local! {
    static DARK: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static BODY: std::cell::Cell<f32> = const { std::cell::Cell::new(DEFAULT_BODY_SIZE) };
    static ZOOM: std::cell::Cell<f32> = const { std::cell::Cell::new(1.0) };
}

/// The page zoom (1 is 100%). Zoom is real, as in Word: font sizes, spacing and indents are
/// written into the editors multiplied by it, and divided by it when read back, so the page is
/// always drawn at its natural scale. (Scaling the drawn page instead, with a Viewbox or the
/// ScrollViewer's zoom, made WinUI redraw far more on every key.) Change it with [`rescale`].
pub fn zoom() -> f32 {
    ZOOM.with(std::cell::Cell::get)
}

/// A length in points at 100%, as written at the current zoom.
fn zoomed(points: f32) -> f32 {
    points * zoom()
}

/// A length in points as read from an editor, back at 100%.
fn unzoomed(points: f32) -> f32 {
    points / zoom()
}

/// Every size, spacing and indent the app writes is a multiple of half a point at 100%, so a
/// value read back at `zoom` rounds to exactly what was written, however often zoom changes.
fn base_points(points: f32, zoom: f32) -> f32 {
    (points / zoom * 2.0).round() / 2.0
}

/// A size written at zoom `from` (see `base_points`), as it is to be at zoom `to`.
pub fn rezoomed(points: f32, from: f32, to: f32) -> f32 {
    base_points(points, from) * to
}

/// The size of code in the Markdown view and in code, math and raw blocks, in points at 100%.
pub const PLAIN_SIZE: f32 = 10.0;

/// The code font size at the current zoom, for plain (Markdown and code) editors.
pub fn plain_size() -> f32 {
    zoomed(PLAIN_SIZE)
}

/// Sets the zoom for text written from now on, and rewrites `documents` (every editor) at it:
/// each font size, paragraph spacing, indent and list tab. Their undo history is cleared,
/// since undoing past this would bring back sizes from the old zoom. Caret and selection stay.
pub fn rescale(documents: &[&RichEditTextDocument], zoom: f32) -> Result<()> {
    let from = ZOOM.with(|cell| cell.replace(zoom));
    if (from - zoom).abs() < f32::EPSILON {
        return Ok(());
    }
    let mut result = Ok(());
    for document in documents {
        // Held, so the many small changes are laid out and drawn once.
        let batched = document.BatchDisplayUpdates().is_ok();
        let outcome = rescale_document(document, from, zoom)
            .and_then(|()| document.ClearUndoRedoHistory());
        if batched {
            _ = document.ApplyDisplayUpdates();
        }
        result = result.and(outcome);
    }
    result
}

fn rescale_document(document: &RichEditTextDocument, from: f32, to: f32) -> Result<()> {
    let scale = |points: f32| rezoomed(points, from, to);
    // Typing into an empty editor takes the default format.
    let default = document.GetDefaultCharacterFormat()?;
    default.SetSize(scale(default.Size()?))?;
    document.SetDefaultCharacterFormat(&default)?;

    let story = document.GetRange(0, 0)?.StoryLength()?;
    // Paragraphs: spacing, indents and the list tab.
    let range = document.GetRange(0, 0)?;
    loop {
        range.Expand(TextRangeUnit::Paragraph)?;
        let start = range.StartPosition()?;
        let end = range.EndPosition()?;
        let format = range.ParagraphFormat()?;
        format.SetSpaceBefore(scale(format.SpaceBefore()?))?;
        format.SetSpaceAfter(scale(format.SpaceAfter()?))?;
        format.SetListTab(scale(format.ListTab()?))?;
        format.SetIndents(
            scale(format.FirstLineIndent()?),
            scale(format.LeftIndent()?),
            scale(format.RightIndent()?),
        )?;
        if end >= story || end <= start {
            break;
        }
        range.SetRange(end, end)?;
    }
    // Runs of one character format (hidden link addresses included): their size.
    let mut position = 0;
    while position < story {
        let run = document.GetRange(position, position + 1)?;
        run.Expand(TextRangeUnit::CharacterFormat)?;
        let next = run.EndPosition()?.max(position + 1);
        run.SetStartPosition(position)?;
        let format = run.CharacterFormat()?;
        format.SetSize(scale(format.Size()?))?;
        position = next;
    }
    Ok(())
}

/// Sets the size, in points, of Normal text (and quotes, lists and headings 5 and 6) styled
/// from now on: Settings › Options › Paragraph font size.
pub fn set_body_size(points: f32) {
    BODY.with(|cell| cell.set(points));
}

pub fn body_size() -> f32 {
    BODY.with(std::cell::Cell::get)
}

/// Chooses the dark or light page colors for text styled from now on.
pub fn set_dark_page(dark: bool) {
    DARK.with(|cell| cell.set(dark));
}

pub fn page_colors() -> PageColors {
    if DARK.with(std::cell::Cell::get) {
        DARK_PAGE
    } else {
        LIGHT_PAGE
    }
}
/// Code and math are tagged with a language because WinUI resets font names after loading.
/// TOM stores languages as LCIDs, so the tags must be distinct real locales: the Windows
/// pseudo-locales, which real text never uses and which have no spelling dictionary.
const CODE_TAG: &str = "qps-ploc";
const MATH_TAG: &str = "qps-plocm";

/// The document's automatic colors. RichEdit draws a transparent color as black, and an
/// explicit text color would also override the automatic (blue) color of links.
#[derive(Clone, Copy)]
struct AutoColors {
    text: Color,
    background: Color,
}

fn auto_colors(document: &RichEditTextDocument) -> Result<AutoColors> {
    let format = document.GetDefaultCharacterFormat()?;
    Ok(AutoColors {
        text: format.ForegroundColor()?,
        background: format.BackgroundColor()?,
    })
}

fn auto_background(document: &RichEditTextDocument) -> Result<Color> {
    Ok(auto_colors(document)?.background)
}

const fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { a: 255, r, g, b }
}

fn utf16_len(text: &str) -> i32 {
    text.encode_utf16().count() as i32
}

fn selection(document: &RichEditTextDocument) -> Result<ITextRange> {
    document.Selection()?.cast()
}

fn text_of(range: &ITextRange, options: TextGetOptions) -> Result<String> {
    let mut text = windows_core::HSTRING::new();
    range.GetText(options, &mut text)?;
    Ok(text.to_string_lossy())
}

/// Replaces the whole document with `paragraphs`, starting a fresh undo history (for loading).
pub fn write(document: &RichEditTextDocument, paragraphs: &[Paragraph]) -> Result<()> {
    // Styling is many small edits; on an editor already on screen (switching back from the
    // Markdown view) each would be laid out and drawn. Held, they are laid out once.
    let batched = document.BatchDisplayUpdates().is_ok();
    let result = document
        .SetText(TextSetOptions::None, &layout(paragraphs).0)
        .and_then(|()| style(document, paragraphs))
        .and_then(|()| document.ClearUndoRedoHistory());
    if batched {
        _ = document.ApplyDisplayUpdates();
    }
    result
}

/// Rewrites the document as `paragraphs`, as an edit the user can undo, and returns where the
/// text at `caret` now is.
///
/// A replaced range that holds the caret drags it to the range's start, and the page scrolls
/// there and back. So when the text is unchanged (usually, after a paste: only its formatting
/// changes) it is left alone; otherwise all of it is replaced, so no hidden link text is left
/// behind, in two pieces split at the caret, which is never inside or at the end of the piece
/// being replaced.
pub fn rewrite(
    document: &RichEditTextDocument,
    paragraphs: &[Paragraph],
    caret: i32,
) -> Result<i32> {
    let new = layout(paragraphs).0;
    let all = document.GetRange(0, i32::MAX)?;
    // With every option off, hidden text (a link's address) is included: it matches the
    // layout only when there is none, and none is left to remove.
    let old = text_of(&all, TextGetOptions::None)?;
    // The story's final paragraph mark, which the layout leaves out and cannot be deleted.
    if old.strip_suffix('\r').unwrap_or(&old) == new {
        style(document, paragraphs)?;
        return Ok(caret);
    }
    let caret = caret.clamp(0, all.EndPosition()?);
    // Where the caret falls in the new text: the visible text before it.
    let split = new_position(
        &new,
        utf16_len(&text_of(&document.GetRange(0, caret)?, TextGetOptions::NoHidden)?),
    );
    let new: Vec<u16> = new.encode_utf16().collect();
    let piece = |from: usize, to: usize| String::from_utf16_lossy(&new[from..to]);
    // After the caret first: the caret is at that piece's start, where it stays.
    document
        .GetRange(caret, i32::MAX)?
        .SetText(&piece(split, new.len()))?;
    // Then before it, with the caret stepped past the piece so it moves along with the end of
    // the piece rather than to its start.
    if caret > 0 {
        set_caret(document, (caret + 1, caret + 1))?;
        document.GetRange(0, caret)?.SetText(&piece(0, split))?;
    }
    style(document, paragraphs)?;
    Ok(split as i32)
}

/// A UTF-16 position in `text`, kept within it and off the second half of a surrogate pair.
fn new_position(text: &str, position: i32) -> usize {
    let mut units = 0;
    for character in text.chars() {
        let next = units + character.len_utf16();
        if next as i32 > position {
            return units;
        }
        units = next;
    }
    units
}

type Layout<'a> = (
    String,
    Vec<(i32, i32, &'a Inline)>,
    Vec<(i32, i32, &'a Paragraph)>,
);

/// The text for `paragraphs`, with where each inline and paragraph lands in it.
fn layout(paragraphs: &[Paragraph]) -> Layout<'_> {
    let mut text = String::new();
    let mut spans = Vec::new();
    let mut ranges = Vec::new();
    let mut position = 0;
    for (index, paragraph) in paragraphs.iter().enumerate() {
        if index > 0 {
            text.push('\r');
            position += 1;
        }
        let start = position;
        if let Some(ListItem {
            kind: ListKind::Task { checked },
            ..
        }) = paragraph.list
        {
            let glyph = task_prefix(checked);
            text.push_str(&glyph);
            position += utf16_len(&glyph);
        }
        for inline in &paragraph.inlines {
            let content = inline.text.replace('\n', "\u{b}");
            let length = utf16_len(&content);
            spans.push((position, position + length, inline));
            text.push_str(&content);
            position += length;
        }
        ranges.push((start, position, paragraph));
    }
    (text, spans, ranges)
}

/// Formats text already laid out for `paragraphs`.
fn style(document: &RichEditTextDocument, paragraphs: &[Paragraph]) -> Result<()> {
    let (_, spans, ranges) = layout(paragraphs);
    let all = document.GetRange(0, i32::MAX)?;
    let auto = auto_colors(document)?;
    reset_character_format(&all.CharacterFormat()?, auto)?;

    for &(start, end, paragraph) in &ranges {
        let range = document.GetRange(start, end)?;
        apply_paragraph(&range, paragraph.kind, paragraph.list, auto)?;
        if let Some(ListItem {
            kind: ListKind::Task { .. },
            ..
        }) = paragraph.list
        {
            document
                .GetRange(start, start + 2)?
                .CharacterFormat()?
                .SetName(SYMBOL_FONT)?;
        }
    }
    for &(start, end, inline) in &spans {
        apply_style(
            &document.GetRange(start, end)?.CharacterFormat()?,
            inline.style,
        )?;
    }
    // Links insert hidden field text, so apply them from the end backwards.
    for &(start, end, inline) in spans.iter().rev() {
        if let Some(link) = &inline.link {
            let range = document.GetRange(start, end)?;
            // Color first: after SetLink the range also spans the hidden field code.
            range.CharacterFormat()?.SetForegroundColor(page_colors().link)?;
            range.SetLink(&quote_link(link))?;
        }
    }
    collapse_heading_spacing(document)
}

fn quote_link(link: &Link) -> String {
    match link {
        Link::Url(url) => format!("\"{url}\""),
        Link::Page(page) => format!("\"{PAGE_SCHEME}{page}\""),
    }
}

fn reset_character_format(format: &ITextCharacterFormat, auto: AutoColors) -> Result<()> {
    format.SetName(BODY_FONT)?;
    format.SetSize(zoomed(body_size()))?;
    format.SetWeight(400)?;
    format.SetItalic(FormatEffect::Off)?;
    format.SetStrikethrough(FormatEffect::Off)?;
    format.SetUnderline(UnderlineType::None)?;
    format.SetBackgroundColor(auto.background)?;
    format.SetForegroundColor(auto.text)
}

fn apply_style(format: &ITextCharacterFormat, style: Style) -> Result<()> {
    if style.bold {
        format.SetWeight(700)?;
    }
    if style.italic {
        format.SetItalic(FormatEffect::On)?;
    }
    if style.strikethrough {
        format.SetStrikethrough(FormatEffect::On)?;
    }
    if style.code || style.raw {
        format.SetName(CODE_FONT)?;
        format.SetLanguageTag(CODE_TAG)?;
        format.SetBackgroundColor(page_colors().code_background)?;
    } else if style.math {
        format.SetName(MATH_FONT)?;
        format.SetLanguageTag(MATH_TAG)?;
        format.SetBackgroundColor(page_colors().math_background)?;
    }
    if style.highlight {
        format.SetBackgroundColor(page_colors().highlight)?;
    }
    Ok(())
}

/// Applies the paragraph conventions (style, indents, list marker) to every paragraph in `range`.
fn apply_paragraph(
    range: &ITextRange,
    kind: ParagraphKind,
    list: Option<ListItem>,
    auto: AutoColors,
) -> Result<()> {
    paragraph_format(range, kind, list)?;
    kind_characters(&range.CharacterFormat()?, kind, auto)
}

/// A paragraph's space before and after, in points. Heading 1 and 2 have the room for the
/// rule SharePoint draws under them, and below it (see `heading_rules`).
fn spacing(kind: ParagraphKind, list: Option<ListItem>) -> (f32, f32) {
    match (kind, list) {
        (ParagraphKind::Heading(1), _) => (18.0, 16.5),
        (ParagraphKind::Heading(2), _) => (12.0, 14.0),
        (ParagraphKind::Heading(_), _) => (12.0, 4.0),
        (_, Some(_)) => (0.0, 3.0),
        _ => (0.0, 8.0),
    }
}

/// SharePoint's CSS collapses margins: a heading straight under a Heading 1 or 2 shares the
/// space below its rule rather than adding its own to it. RichEdit adds space after and
/// before, so such a heading's space before is cut by the space after above it. Run after
/// any change to paragraph kinds.
fn collapse_heading_spacing(document: &RichEditTextDocument) -> Result<()> {
    let story = document.GetRange(0, 0)?.StoryLength()?;
    let range = document.GetRange(0, 0)?;
    let mut above: Option<u8> = None;
    loop {
        range.Expand(TextRangeUnit::Paragraph)?;
        let start = range.StartPosition()?;
        let end = range.EndPosition()?;
        let format = range.ParagraphFormat()?;
        let level = heading_level(format.Style()?);
        if let Some(level) = level {
            let own = spacing(ParagraphKind::Heading(level), None).0;
            let before = match above {
                Some(above @ (1 | 2)) => {
                    (own - spacing(ParagraphKind::Heading(above), None).1).max(0.0)
                }
                _ => own,
            };
            // Only when it differs, so an unchanged page gains no undo steps.
            if (unzoomed(format.SpaceBefore()?) - before).abs() > 0.05 {
                format.SetSpaceBefore(zoomed(before))?;
            }
        }
        above = level;
        if end >= story || end <= start {
            break;
        }
        range.SetRange(end, end)?;
    }
    Ok(())
}

/// The paragraph part of [`apply_paragraph`], leaving the text's own formatting alone.
fn paragraph_format(range: &ITextRange, kind: ParagraphKind, list: Option<ListItem>) -> Result<()> {
    let format = range.ParagraphFormat()?;
    format.SetStyle(match kind {
        ParagraphKind::Heading(level) => heading_style(level),
        _ => ParagraphStyle::Normal,
    })?;
    let quote = kind == ParagraphKind::Quote;
    let level = list.map_or(0.0, |item| f32::from(item.level));
    let left = level * LEVEL_INDENT + if quote { QUOTE_INDENT } else { 0.0 };
    let right = if quote { QUOTE_INDENT } else { 0.0 };
    if list.is_some() {
        format.SetIndents(
            zoomed(-LIST_HANG),
            zoomed(left + LIST_HANG),
            zoomed(right),
        )?;
        format.SetListTab(zoomed(LIST_HANG))?;
    } else {
        format.SetIndents(0.0, zoomed(left), zoomed(right))?;
    }
    let (before, after) = spacing(kind, list);
    format.SetSpaceBefore(zoomed(before))?;
    format.SetSpaceAfter(zoomed(after))?;
    match list.map(|item| item.kind) {
        Some(ListKind::Bullet) => format.SetListType(MarkerType::Bullet)?,
        Some(ListKind::Numbered) => {
            format.SetListType(MarkerType::Arabic)?;
            format.SetListStyle(MarkerStyle::Period)?;
            format.SetListStart(1)?;
        }
        _ => format.SetListType(MarkerType::None)?,
    }
    Ok(())
}

/// The character formatting that goes with a paragraph kind (heading size, weight and color).
fn kind_characters(
    characters: &ITextCharacterFormat,
    kind: ParagraphKind,
    auto: AutoColors,
) -> Result<()> {
    match kind {
        ParagraphKind::Heading(level) => {
            let size = HEADING_SIZES
                .get(usize::from(level.max(1)) - 1)
                .copied()
                .unwrap_or_else(body_size);
            characters.SetSize(zoomed(size))?;
            characters.SetWeight(600)?;
            characters.SetForegroundColor(page_colors().text)?;
        }
        ParagraphKind::Quote => {
            characters.SetSize(zoomed(body_size()))?;
            characters.SetForegroundColor(page_colors().quote)?;
        }
        ParagraphKind::Normal => {
            characters.SetSize(zoomed(body_size()))?;
            characters.SetForegroundColor(auto.text)?;
        }
    }
    Ok(())
}

fn heading_style(level: u8) -> ParagraphStyle {
    ParagraphStyle(ParagraphStyle::Heading1.0 + i32::from(level.clamp(1, 6)) - 1)
}

fn heading_level(style: ParagraphStyle) -> Option<u8> {
    let level = style.0 - ParagraphStyle::Heading1.0 + 1;
    (1..=6).contains(&level).then_some(level as u8)
}

/// Reads every paragraph of the document back into the model.
pub fn read(document: &RichEditTextDocument) -> Result<Vec<Paragraph>> {
    let story = document.GetRange(0, 0)?.StoryLength()?;
    read_range(document, 0, story)
}

fn read_range(document: &RichEditTextDocument, from: i32, to: i32) -> Result<Vec<Paragraph>> {
    let mut paragraphs = Vec::new();
    let range = document.GetRange(from, from)?;
    loop {
        range.Expand(TextRangeUnit::Paragraph)?;
        let start = range.StartPosition()?;
        let end = range.EndPosition()?;
        paragraphs.push(read_paragraph(document, &range)?);
        if end >= to || end <= start {
            break;
        }
        range.SetRange(end, end)?;
    }
    // The story always ends with an empty paragraph mark; it is not content.
    if paragraphs.len() > 1
        && paragraphs
            .last()
            .is_some_and(|p| p.is_empty() && p.list.is_none())
    {
        paragraphs.pop();
    }
    Ok(paragraphs)
}

fn read_paragraph(document: &RichEditTextDocument, range: &ITextRange) -> Result<Paragraph> {
    let format = range.ParagraphFormat()?;
    let quote = unzoomed(format.RightIndent()?) >= QUOTE_INDENT / 2.0;
    let kind = match heading_level(format.Style()?) {
        Some(level) => ParagraphKind::Heading(level),
        None if quote => ParagraphKind::Quote,
        None => ParagraphKind::Normal,
    };
    // Where the first line starts: a list item's hanging indent is not nesting.
    let indent = unzoomed(format.LeftIndent()? + format.FirstLineIndent()?.min(0.0))
        - if quote { QUOTE_INDENT } else { 0.0 };
    let level = (indent / LEVEL_INDENT).round().clamp(0.0, 8.0) as u8;
    let list_type = format.ListType()?;

    let start = range.StartPosition()?;
    let mut end = range.EndPosition()?;
    let mut inlines = read_inlines(document, start, end)?;
    if inlines
        .last()
        .is_some_and(|inline| inline.text.ends_with('\r'))
    {
        let last = inlines.last_mut().unwrap();
        last.text.pop();
        end -= 1;
    }
    let _ = end;

    let mut list = None;
    let plain: String = inlines.iter().map(|inline| inline.text.as_str()).collect();
    if let Some((checked, rest)) = strip_task_prefix(&plain) {
        // The glyph and its space can sit in separate runs (the glyph has its own font).
        remove_leading(&mut inlines, plain.len() - rest.len());
        list = Some(ListItem {
            kind: ListKind::Task { checked },
            level,
        });
    }
    if list.is_none() {
        list = if list_type == MarkerType::Bullet {
            Some(ListKind::Bullet)
        } else if list_type == MarkerType::None || list_type == MarkerType::Undefined {
            None
        } else {
            Some(ListKind::Numbered)
        }
        .map(|kind| ListItem { kind, level });
    }
    let mut inlines = crate::markdown::merge(inlines);
    if let ParagraphKind::Heading(_) = kind {
        // Headings are bold by style; bold inside a heading carries no meaning in Markdown.
        for inline in &mut inlines {
            inline.style.bold = false;
        }
        inlines = crate::markdown::merge(inlines);
    }
    Ok(Paragraph {
        kind,
        list,
        inlines,
    })
}

fn read_inlines(document: &RichEditTextDocument, start: i32, end: i32) -> Result<Vec<Inline>> {
    let mut inlines = Vec::new();
    let mut position = start;
    let mut guard = 0;
    while position < end && guard < 10_000 {
        guard += 1;
        let run = document.GetRange(position, position + 1)?;
        let link = parse_link(&run.Link()?);
        if link.is_some() {
            run.Expand(TextRangeUnit::Link)?;
        } else {
            run.Expand(TextRangeUnit::CharacterFormat)?;
        }
        if run.StartPosition()? < position {
            run.SetStartPosition(position)?;
        }
        if run.EndPosition()? > end {
            run.SetEndPosition(end)?;
        }
        let next = run.EndPosition()?.max(position + 1);
        let text = text_of(&run, TextGetOptions::NoHidden)?;
        if !text.is_empty() {
            // Take the style from the last visible character (a link starts with hidden text).
            let probe = document.GetRange(next - 1, next)?;
            let style = read_style(&probe.CharacterFormat()?)?;
            inlines.push(Inline {
                text: text.replace('\u{b}', "\n"),
                style,
                link,
            });
        }
        position = next;
    }
    Ok(inlines)
}

/// The link under the point (x, y), relative to the editor, as it should read in a tooltip: a
/// web address as it is, a page link as the page's name. None off text or on plain text: the
/// editor maps a point to the nearest place between characters, so the characters either side
/// are checked to really be under the point.
pub fn link_at(document: &RichEditTextDocument, x: f32, y: f32) -> Result<Option<String>> {
    let place = document
        .GetRangeFromPoint(Point { x, y }, PointOptions::ClientCoordinates)?
        .StartPosition()?;
    for position in [place, place - 1] {
        if position < 0 {
            continue;
        }
        let rect = rect_at(document, position)?;
        let inside = x >= rect.x
            && x <= rect.x + rect.width
            && y >= rect.y
            && y <= rect.y + rect.height;
        if !inside {
            continue;
        }
        let link = document.GetRange(position, position + 1)?.Link()?;
        return Ok(parse_link(&link).map(|link| match link {
            Link::Url(url) => url,
            Link::Page(page) => format!("Page: {page}"),
        }));
    }
    Ok(None)
}

fn parse_link(field: &str) -> Option<Link> {
    let url = field.trim().trim_matches('"').trim();
    if url.is_empty() {
        return None;
    }
    Some(match url.strip_prefix(PAGE_SCHEME) {
        Some(page) => Link::Page(page.to_string()),
        None => Link::Url(url.to_string()),
    })
}

fn read_style(format: &ITextCharacterFormat) -> Result<Style> {
    let name = format.Name()?;
    let tag = format.LanguageTag()?;
    let background = format.BackgroundColor()?;
    Ok(Style {
        bold: format.Weight()? >= 600 || format.Bold()? == FormatEffect::On,
        italic: format.Italic()? == FormatEffect::On,
        strikethrough: format.Strikethrough()? == FormatEffect::On,
        highlight: background.a == 255
            && ![LIGHT_PAGE, DARK_PAGE].iter().any(|colors| {
                background == colors.code_background || background == colors.math_background
            }),
        code: tag.eq_ignore_ascii_case(CODE_TAG) || name == CODE_FONT,
        math: tag.eq_ignore_ascii_case(MATH_TAG) || name == MATH_FONT,
        raw: false,
    })
}

/// Formatting at the current selection, for the toolbar's toggle states.
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

pub fn selection_state(document: &RichEditTextDocument) -> Result<SelectionState> {
    let selection = selection(document)?;
    let format = selection.CharacterFormat()?;
    let style = read_style(&format)?;
    let start = selection.StartPosition()?;
    let paragraph = document.GetRange(start, start)?;
    paragraph.Expand(TextRangeUnit::Paragraph)?;
    let read = read_paragraph(document, &paragraph)?;
    Ok(SelectionState {
        // Headings are drawn semi-bold by their style, which is not bold formatting.
        bold: !matches!(read.kind, ParagraphKind::Heading(_))
            && (format.Bold()? == FormatEffect::On || format.Weight()? >= 600),
        italic: format.Italic()? == FormatEffect::On,
        strikethrough: format.Strikethrough()? == FormatEffect::On,
        highlight: style.highlight,
        code: style.code,
        kind: read.kind,
        list: read.list.map(|item| item.kind),
        has_selection: selection.EndPosition()? > start,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InlineCommand {
    Bold,
    Italic,
    Strikethrough,
    Highlight,
    Code,
    Math,
}

pub fn toggle_inline(document: &RichEditTextDocument, command: InlineCommand) -> Result<()> {
    let selection = selection(document)?;
    let format = selection.CharacterFormat()?;
    let state = read_style(&format)?;
    match command {
        InlineCommand::Bold => {
            let on = !(format.Bold()? == FormatEffect::On || format.Weight()? >= 600);
            format.SetWeight(if on { 700 } else { 400 })?;
        }
        InlineCommand::Italic => format.SetItalic(FormatEffect::Toggle)?,
        InlineCommand::Strikethrough => format.SetStrikethrough(FormatEffect::Toggle)?,
        InlineCommand::Highlight => format.SetBackgroundColor(if state.highlight {
            auto_background(document)?
        } else {
            page_colors().highlight
        })?,
        InlineCommand::Code | InlineCommand::Math => {
            let (on, font, tag, background) = if command == InlineCommand::Code {
                (state.code, CODE_FONT, CODE_TAG, page_colors().code_background)
            } else {
                (state.math, MATH_FONT, MATH_TAG, page_colors().math_background)
            };
            if on {
                let defaults = document.GetDefaultCharacterFormat()?;
                format.SetName(BODY_FONT)?;
                format.SetLanguageTag(&defaults.LanguageTag()?)?;
                format.SetBackgroundColor(defaults.BackgroundColor()?)?;
            } else {
                format.SetName(font)?;
                format.SetLanguageTag(tag)?;
                format.SetBackgroundColor(background)?;
            }
        }
    }
    selection.SetCharacterFormat(&format)
}

/// Each paragraph touched by the selection, as (start, end) positions.
fn selected_paragraphs(document: &RichEditTextDocument) -> Result<Vec<(i32, i32)>> {
    let selection = selection(document)?;
    let from = selection.StartPosition()?;
    let to = selection.EndPosition()?;
    let range = document.GetRange(from, from)?;
    let mut paragraphs = Vec::new();
    loop {
        range.Expand(TextRangeUnit::Paragraph)?;
        let start = range.StartPosition()?;
        let end = range.EndPosition()?;
        paragraphs.push((start, end));
        if end >= to || end <= start || paragraphs.len() > 10_000 {
            break;
        }
        range.SetRange(end, end)?;
    }
    Ok(paragraphs)
}

pub fn set_paragraph_kind(document: &RichEditTextDocument, kind: ParagraphKind) -> Result<()> {
    let auto = auto_colors(document)?;
    let leaving_heading = |from: ParagraphKind| {
        matches!(from, ParagraphKind::Heading(_)) && !matches!(kind, ParagraphKind::Heading(_))
    };
    document.BeginUndoGroup()?;
    let mut was_heading = false;
    for (start, end) in selected_paragraphs(document)?.into_iter().rev() {
        let range = document.GetRange(start, end)?;
        let current = read_paragraph(document, &range)?;
        // A heading leaves its list: Markdown headings cannot be list items here.
        let list = if matches!(kind, ParagraphKind::Heading(_)) {
            if matches!(
                current.list,
                Some(ListItem {
                    kind: ListKind::Task { .. },
                    ..
                })
            ) {
                remove_task_glyph(document, start)?;
            }
            None
        } else {
            current.list
        };
        let range = document.GetRange(start, end)?;
        range.Expand(TextRangeUnit::Paragraph)?;
        apply_paragraph(&range, kind, list, auto)?;
        if leaving_heading(current.kind) {
            // Heading text is bold by style only; it must not stay bold as body text.
            range.CharacterFormat()?.SetWeight(400)?;
            was_heading = true;
        }
    }
    // With nothing selected, also set the format of the text about to be typed. For an empty
    // paragraph the formatting above only reaches its paragraph mark.
    let selection = selection(document)?;
    if selection.StartPosition()? == selection.EndPosition()? {
        let format = selection.CharacterFormat()?;
        kind_characters(&format, kind, auto)?;
        if was_heading {
            format.SetWeight(400)?;
        }
        selection.SetCharacterFormat(&format)?;
    }
    collapse_heading_spacing(document)?;
    document.EndUndoGroup()
}

/// Enter at the end of a heading starts a Normal paragraph, as in Word. Returns whether the
/// key was handled.
pub fn enter_after_heading(document: &RichEditTextDocument) -> Result<bool> {
    let selection = selection(document)?;
    let caret = selection.StartPosition()?;
    if selection.EndPosition()? != caret {
        return Ok(false);
    }
    let paragraph = document.GetRange(caret, caret)?;
    paragraph.Expand(TextRangeUnit::Paragraph)?;
    let current = read_paragraph(document, &paragraph)?;
    // The caret must sit just before the paragraph mark.
    if !matches!(current.kind, ParagraphKind::Heading(_)) || caret != paragraph.EndPosition()? - 1 {
        return Ok(false);
    }
    document.BeginUndoGroup()?;
    selection.SetText("\r")?;
    selection.SetRange(caret + 1, caret + 1)?;
    set_paragraph_kind(document, ParagraphKind::Normal)?;
    document.EndUndoGroup()?;
    Ok(true)
}

/// Enter in a list item, as in Word: the item splits at the caret and the new one is the same
/// kind of item (a task gets its own box, unticked); Enter on an empty item ends the list
/// there instead. Returns whether the key was handled.
pub fn enter_in_list(document: &RichEditTextDocument) -> Result<bool> {
    let selection = selection(document)?;
    let caret = selection.StartPosition()?;
    if selection.EndPosition()? != caret {
        return Ok(false);
    }
    let paragraph = document.GetRange(caret, caret)?;
    paragraph.Expand(TextRangeUnit::Paragraph)?;
    let start = paragraph.StartPosition()?;
    let current = read_paragraph(document, &paragraph)?;
    let Some(item) = current.list else {
        return Ok(false);
    };
    let task = matches!(item.kind, ListKind::Task { .. });
    let prefix = if task {
        let text = text_of(&document.GetRange(start, start + 3)?, TextGetOptions::None)?;
        strip_task_prefix(&text).map_or(0, |(_, rest)| utf16_len(&text) - utf16_len(rest))
    } else {
        0
    };
    // In the task box itself, Enter does what it always does.
    if caret < start + prefix {
        return Ok(false);
    }
    let auto = auto_colors(document)?;
    document.BeginUndoGroup()?;
    if current.is_empty() {
        if task {
            remove_task_glyph(document, start)?;
        }
        let range = document.GetRange(start, start)?;
        range.Expand(TextRangeUnit::Paragraph)?;
        apply_paragraph(&range, current.kind, None, auto)?;
    } else {
        selection.SetText("\r")?;
        let next = caret + 1;
        let mut text_start = next;
        let kind = match item.kind {
            ListKind::Task { .. } => {
                insert_task_glyph(document, next, false)?;
                text_start += utf16_len(&task_prefix(false));
                ListKind::Task { checked: false }
            }
            other => other,
        };
        let range = document.GetRange(next, next)?;
        range.Expand(TextRangeUnit::Paragraph)?;
        paragraph_format(&range, current.kind, Some(ListItem { kind, ..item }))?;
        selection.SetRange(text_start, text_start)?;
    }
    document.EndUndoGroup()?;
    Ok(true)
}

pub fn toggle_list(document: &RichEditTextDocument, kind: ListKind) -> Result<()> {
    let paragraphs = selected_paragraphs(document)?;
    let all_match = paragraphs.iter().try_fold(true, |all, &(start, end)| {
        let current = read_paragraph(document, &document.GetRange(start, end)?)?;
        Ok::<_, windows_core::Error>(
            all && current
                .list
                .is_some_and(|item| same_list_kind(item.kind, kind)),
        )
    })?;
    document.BeginUndoGroup()?;
    for (start, end) in paragraphs.into_iter().rev() {
        let range = document.GetRange(start, end)?;
        let current = read_paragraph(document, &range)?;
        let level = current.list.map_or(0, |item| item.level);
        let was_task = matches!(
            current.list,
            Some(ListItem {
                kind: ListKind::Task { .. },
                ..
            })
        );
        if was_task {
            remove_task_glyph(document, start)?;
        }
        let list = if all_match {
            None
        } else {
            Some(ListItem { kind, level })
        };
        if let Some(ListItem {
            kind: ListKind::Task { checked },
            ..
        }) = list
        {
            insert_task_glyph(document, start, checked)?;
        }
        let end = document.GetRange(start, start)?;
        end.Expand(TextRangeUnit::Paragraph)?;
        apply_paragraph(&end, current.kind, list, auto_colors(document)?)?;
    }
    caret_after_task_box(document)?;
    document.EndUndoGroup()
}

/// A caret left in front of a task's box (the box was just put in front of it) moves after
/// it, where the item's text goes.
fn caret_after_task_box(document: &RichEditTextDocument) -> Result<()> {
    let selection = selection(document)?;
    let caret = selection.StartPosition()?;
    if selection.EndPosition()? != caret {
        return Ok(());
    }
    let paragraph = document.GetRange(caret, caret)?;
    paragraph.Expand(TextRangeUnit::Paragraph)?;
    let start = paragraph.StartPosition()?;
    let text = text_of(&document.GetRange(start, start + 3)?, TextGetOptions::None)?;
    if let Some((_, rest)) = strip_task_prefix(&text) {
        let after = start + utf16_len(&text) - utf16_len(rest);
        if caret < after {
            selection.SetRange(after, after)?;
        }
    }
    Ok(())
}

fn same_list_kind(a: ListKind, b: ListKind) -> bool {
    matches!(
        (a, b),
        (ListKind::Bullet, ListKind::Bullet)
            | (ListKind::Numbered, ListKind::Numbered)
            | (ListKind::Task { .. }, ListKind::Task { .. })
    )
}

fn insert_task_glyph(document: &RichEditTextDocument, start: i32, checked: bool) -> Result<()> {
    let range = document.GetRange(start, start)?;
    range.SetText(&task_prefix(checked))?;
    document
        .GetRange(start, start + 2)?
        .CharacterFormat()?
        .SetName(SYMBOL_FONT)
}

fn remove_task_glyph(document: &RichEditTextDocument, start: i32) -> Result<()> {
    let range = document.GetRange(start, start + 3)?;
    let text = text_of(&range, TextGetOptions::None)?;
    let length = match strip_task_prefix(&text) {
        Some((_, rest)) => utf16_len(&text) - utf16_len(rest),
        None => 1,
    };
    document.GetRange(start, start + length)?.SetText("")
}

/// Checks or unchecks the task items in the selection.
pub fn toggle_task_done(document: &RichEditTextDocument) -> Result<()> {
    // Each selected task: where it starts, how long its box is, and whether it is done.
    let mut tasks = Vec::new();
    for (start, _) in selected_paragraphs(document)? {
        let head = text_of(&document.GetRange(start, start + 4)?, TextGetOptions::None)?;
        if let Some((checked, rest)) = strip_task_prefix(&head) {
            tasks.push((start, utf16_len(&head) - utf16_len(rest), checked));
        }
    }
    // As in Word: all done, unless every one already is.
    let done = !tasks.iter().all(|&(_, _, checked)| checked);
    document.BeginUndoGroup()?;
    // From the last up: the two boxes differ in length, so changing one moves those after it.
    for &(start, length, _) in tasks.iter().rev() {
        let prefix = document.GetRange(start, start + length)?;
        prefix.SetText(&task_prefix(done))?;
        prefix.CharacterFormat()?.SetName(SYMBOL_FONT)?;
    }
    document.EndUndoGroup()
}

/// Changes the list level of the selected list paragraphs by `delta`.
pub fn change_level(document: &RichEditTextDocument, delta: i8) -> Result<()> {
    document.BeginUndoGroup()?;
    for (start, end) in selected_paragraphs(document)? {
        let range = document.GetRange(start, end)?;
        let current = read_paragraph(document, &range)?;
        if let Some(mut item) = current.list {
            item.level = item.level.saturating_add_signed(delta).min(8);
            apply_paragraph(&range, current.kind, Some(item), auto_colors(document)?)?;
        }
    }
    document.EndUndoGroup()
}

/// Turns the selection into a link, or inserts `label` as a link when nothing is selected.
pub fn insert_link(document: &RichEditTextDocument, link: &Link, label: &str) -> Result<()> {
    let selection = selection(document)?;
    let range = document.GetRange(selection.StartPosition()?, selection.EndPosition()?)?;
    if range.StartPosition()? == range.EndPosition()? {
        let text = if label.is_empty() {
            match link {
                Link::Url(url) => url.clone(),
                Link::Page(page) => page.clone(),
            }
        } else {
            label.to_string()
        };
        range.SetText(&text)?;
    }
    range.CharacterFormat()?.SetForegroundColor(page_colors().link)?;
    range.SetLink(&quote_link(link))?;
    let end = range.EndPosition()?;
    selection.SetRange(end, end)
}

/// The paragraphs before and after the caret's paragraph, used to place a block object at the
/// caret. The caret paragraph stays with `before` unless it is empty.
pub fn split_at_selection(
    document: &RichEditTextDocument,
) -> Result<(Vec<Paragraph>, Vec<Paragraph>)> {
    let selection = selection(document)?;
    let caret = selection.EndPosition()?;
    let range = document.GetRange(caret, caret)?;
    range.Expand(TextRangeUnit::Paragraph)?;
    let split = range.EndPosition()?;
    let story = document.GetRange(0, 0)?.StoryLength()?;
    let mut before = read_range(document, 0, split)?;
    let after = if split < story {
        read_range(document, split, story)?
    } else {
        Vec::new()
    };
    if before
        .last()
        .is_some_and(|p| p.is_empty() && p.list.is_none())
    {
        before.pop();
    }
    Ok((before, after))
}

/// The selection as (start, end), for restoring after the document is rewritten.
pub fn caret(document: &RichEditTextDocument) -> Result<(i32, i32)> {
    let selection = selection(document)?;
    Ok((selection.StartPosition()?, selection.EndPosition()?))
}

pub fn set_caret(document: &RichEditTextDocument, (start, end): (i32, i32)) -> Result<()> {
    selection(document)?.SetRange(start, end)
}

pub fn selected_text(document: &RichEditTextDocument) -> Result<String> {
    Ok(text_of(&selection(document)?, TextGetOptions::NoHidden)?
        .trim()
        .to_string())
}

/// Removes the first `bytes` bytes of text across runs, dropping runs that become empty.
fn remove_leading(inlines: &mut Vec<Inline>, mut bytes: usize) {
    while bytes > 0 && !inlines.is_empty() {
        let first = &mut inlines[0];
        if first.text.len() <= bytes {
            bytes -= first.text.len();
            inlines.remove(0);
        } else {
            first.text.drain(..bytes);
            bytes = 0;
        }
    }
}

/// A heading in a text flow, for the Navigation pane.
#[derive(Clone, Debug, PartialEq)]
pub struct HeadingInfo {
    pub level: u8,
    pub text: String,
    /// Where the heading starts in its editor.
    pub position: i32,
}

/// The headings of a document, in order. Reads only paragraph styles and text, so it is cheap
/// enough to run while typing.
pub fn headings(document: &RichEditTextDocument) -> Result<Vec<HeadingInfo>> {
    let story = document.GetRange(0, 0)?.StoryLength()?;
    let range = document.GetRange(0, 0)?;
    let mut headings = Vec::new();
    loop {
        range.Expand(TextRangeUnit::Paragraph)?;
        let start = range.StartPosition()?;
        let end = range.EndPosition()?;
        if let Some(level) = heading_level(range.ParagraphFormat()?.Style()?) {
            let text = text_of(&range, TextGetOptions::NoHidden)?;
            headings.push(HeadingInfo {
                level,
                text: text.trim().replace('\u{b}', " "),
                position: start,
            });
        }
        if end >= story || end <= start {
            break;
        }
        range.SetRange(end, end)?;
    }
    Ok(headings)
}

/// How far below the bottom of a heading's text its rule sits, in pixels, as SharePoint has
/// it: about 0.55 of the heading's size.
fn rule_gap(level: u8) -> f32 {
    if level == 1 { 15.0 } else { 13.0 }
}

/// Where SharePoint's rule under Heading 1 and Heading 2 goes in an editor: the top of each,
/// in pixels from the top of the editor. RichEdit cannot draw paragraph borders, so the page
/// lays the rules over the editor; they sit in the heading's space after.
pub fn heading_rules(document: &RichEditTextDocument) -> Result<Vec<f32>> {
    let story = document.GetRange(0, 0)?.StoryLength()?;
    let range = document.GetRange(0, 0)?;
    let mut rules = Vec::new();
    loop {
        range.Expand(TextRangeUnit::Paragraph)?;
        let start = range.StartPosition()?;
        let end = range.EndPosition()?;
        if let Some(level @ (1 | 2)) = heading_level(range.ParagraphFormat()?.Style()?) {
            // The last character before the paragraph mark is on the heading's last line; an
            // empty heading has only its mark.
            let last = (end - 2).max(start);
            let rect = rect_at(document, last)?;
            // The last line's rectangle takes in the paragraph's space after (points, here
            // turned to pixels); without it, it ends just below the text's baseline.
            let after = zoomed(spacing(ParagraphKind::Heading(level), None).1) * 96.0 / 72.0;
            rules.push(rect.y + rect.height - after + rule_gap(level) * zoom());
        }
        if end >= story || end <= start {
            break;
        }
        range.SetRange(end, end)?;
    }
    Ok(rules)
}

/// Where the text at `position` is drawn, relative to the editor.
pub fn rect_at(document: &RichEditTextDocument, position: i32) -> Result<Rect> {
    let mut rect = Rect::default();
    let mut hit = 0;
    document.GetRange(position, position + 1)?.GetRect(
        PointOptions::ClientCoordinates,
        &mut rect,
        &mut hit,
    )?;
    Ok(rect)
}

/// Where `query` occurs in the document, as (start, end) positions.
pub fn find_all(
    document: &RichEditTextDocument,
    query: &str,
    match_case: bool,
    whole_words: bool,
) -> Result<Vec<(i32, i32)>> {
    let mut found = Vec::new();
    if query.is_empty() {
        return Ok(found);
    }
    // RichEdit rejects Case and Word together ("the parameter is incorrect"), so whole
    // words are checked here, by the characters either side of each match.
    let options = if match_case {
        FindOptions::Case
    } else {
        FindOptions::None
    };
    let story = document.GetRange(0, 0)?.StoryLength()?;
    let range = document.GetRange(0, 0)?;
    while found.len() < 10_000 {
        if range.FindText(query, story, options)? == 0 {
            break;
        }
        let (start, end) = (range.StartPosition()?, range.EndPosition()?);
        if end <= start {
            break;
        }
        // Skip matches inside the hidden field code of a link.
        if !text_of(&range, TextGetOptions::NoHidden)?.is_empty()
            && (!whole_words || is_whole_word(document, start, end, story)?)
        {
            found.push((start, end));
        }
        range.SetRange(end, end)?;
    }
    Ok(found)
}

/// Whether the text from `start` to `end` is not part of a longer word.
fn is_whole_word(document: &RichEditTextDocument, start: i32, end: i32, story: i32) -> Result<bool> {
    let is_word_char = |position: i32| -> Result<bool> {
        if position < 0 || position >= story {
            return Ok(false);
        }
        let text = text_of(&document.GetRange(position, position + 1)?, TextGetOptions::None)?;
        Ok(text
            .chars()
            .next()
            .is_some_and(|c| c.is_alphanumeric() || c == '_'))
    };
    Ok(!is_word_char(start - 1)? && !is_word_char(end)?)
}

/// Each paragraph's start position and visible text, for the spell checker.
pub fn paragraph_texts(document: &RichEditTextDocument) -> Result<Vec<(i32, i32, String)>> {
    let story = document.GetRange(0, 0)?.StoryLength()?;
    let mut paragraphs = Vec::new();
    let mut position = 0;
    while position < story {
        let range = document.GetRange(position, position)?;
        range.Expand(TextRangeUnit::Paragraph)?;
        let (start, end) = (range.StartPosition()?, range.EndPosition()?);
        if end <= position {
            break;
        }
        paragraphs.push((start, end, text_of(&range, TextGetOptions::NoHidden)?));
        position = end;
    }
    Ok(paragraphs)
}

/// Where `word` (as a whole word, matching case) next appears between `from` and `to`.
pub fn find_word(
    document: &RichEditTextDocument,
    from: i32,
    to: i32,
    word: &str,
) -> Result<Option<(i32, i32)>> {
    Ok(find_all(document, word, true, true)?
        .into_iter()
        .find(|&(start, end)| start >= from && end <= to))
}

/// Whether the text at `position` is inline code or math, which is not spell checked.
pub fn is_code_or_math(document: &RichEditTextDocument, position: i32) -> Result<bool> {
    let style = read_style(
        &document
            .GetRange(position, position + 1)?
            .CharacterFormat()?,
    )?;
    Ok(style.code || style.math)
}

/// The text around a match, within its paragraph: (before, match, after).
pub fn snippet(
    document: &RichEditTextDocument,
    start: i32,
    end: i32,
) -> Result<(String, String, String)> {
    const CONTEXT: i32 = 40;
    let paragraph = document.GetRange(start, start)?;
    paragraph.Expand(TextRangeUnit::Paragraph)?;
    let from = paragraph.StartPosition()?.max(start - CONTEXT);
    let to = paragraph.EndPosition()?.min(end + CONTEXT);
    let clean = |text: String| text.replace(['\r', '\u{b}', '\n'], " ");
    let mut before = clean(text_of(
        &document.GetRange(from, start)?,
        TextGetOptions::NoHidden,
    )?);
    let found = clean(text_of(
        &document.GetRange(start, end)?,
        TextGetOptions::NoHidden,
    )?);
    let mut after = clean(text_of(
        &document.GetRange(end, to)?,
        TextGetOptions::NoHidden,
    )?);
    if from > paragraph.StartPosition()? {
        before = format!("…{}", before.trim_start());
    }
    if to < paragraph.EndPosition()? {
        after = format!("{}…", after.trim_end());
    }
    Ok((before, found, after))
}

/// Replaces each range with its own text, as one undo step. Returns how many were replaced.
pub fn replace_each(
    document: &RichEditTextDocument,
    changes: &[(i32, i32, String)],
) -> Result<usize> {
    let mut changes = changes.to_vec();
    changes.sort_unstable_by_key(|&(start, _, _)| start);
    document.BeginUndoGroup()?;
    // From the end backwards, so earlier positions stay valid.
    for (start, end, text) in changes.iter().rev() {
        document.GetRange(*start, *end)?.SetText(text)?;
    }
    document.EndUndoGroup()?;
    Ok(changes.len())
}

pub fn replace_ranges(
    document: &RichEditTextDocument,
    ranges: &[(i32, i32)],
    replacement: &str,
) -> Result<usize> {
    let mut ranges = ranges.to_vec();
    ranges.sort_unstable();
    document.BeginUndoGroup()?;
    for &(start, end) in ranges.iter().rev() {
        document.GetRange(start, end)?.SetText(replacement)?;
    }
    document.EndUndoGroup()?;
    Ok(ranges.len())
}

#[cfg(test)]
mod tests {
    use super::{base_points, new_position};

    #[test]
    fn zooming_back_and_forth_does_not_drift_sizes() {
        // Every zoom step (10% to 500% by tens) and the app's sizes, spacing and indents.
        let values = [0.0, 1.5, 3.0, 4.0, 8.0, 10.0, 10.5, 12.0, 13.5, 16.5, 18.0, 21.0, 36.0];
        for step in 1..=50 {
            let zoom = step as f32 / 10.0;
            for value in values {
                // What the editor holds at this zoom: whole twips (1/20 point).
                let stored = (value * zoom * 20.0).round() / 20.0;
                assert_eq!(base_points(stored, zoom), value, "{value} at {zoom}");
            }
        }
    }

    #[test]
    fn a_caret_position_stays_within_the_text_and_whole_characters() {
        assert_eq!(new_position("abc", 1), 1);
        assert_eq!(new_position("abc", 0), 0);
        assert_eq!(new_position("abc", 9), 3);
        // An emoji is two UTF-16 units: a position between them moves before it.
        assert_eq!(new_position("a😀b", 2), 1);
        assert_eq!(new_position("a😀b", 3), 3);
    }
}
