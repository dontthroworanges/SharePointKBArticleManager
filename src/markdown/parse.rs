//! Markdown → [`Document`] using comrak with the extensions SharePoint KB pages use.

use super::model::*;
use comrak::nodes::{ListType, NodeValue, TableAlignment};
use comrak::{Arena, Node, Options, parse_document};

/// Info string of the fenced block that stands in for a `:::image` line while comrak parses.
const MEDIA_FENCE: &str = "kb-media";

pub fn parse(source: &str) -> Document {
    let prepared = wrap_media_lines(&source.replace("\r\n", "\n"));
    let arena = Arena::new();
    let root = parse_document(&arena, &prepared, &options());
    let mut parser = Parser {
        lines: prepared.lines().collect(),
        document: Document::default(),
    };
    for node in root.children() {
        parser.block(node);
    }
    parser.document
}

pub(crate) fn options() -> Options<'static> {
    let mut options = Options::default();
    let extension = &mut options.extension;
    extension.strikethrough = true;
    extension.table = true;
    extension.tasklist = true;
    extension.highlight = true;
    extension.math_dollars = true;
    extension.math_dollars_block = true;
    extension.wikilinks_title_after_pipe = true;
    extension.front_matter_delimiter = Some("---".into());
    options
}

struct Parser<'s> {
    lines: Vec<&'s str>,
    document: Document,
}

impl Parser<'_> {
    fn block<'a>(&mut self, node: Node<'a>) {
        let value = node.data().value.clone();
        match value {
            NodeValue::FrontMatter(text) => self.document.properties = parse_properties(&text),
            NodeValue::Paragraph => {
                if let Some(math) = display_math(node) {
                    self.push(Block::Math(math));
                } else {
                    self.push(Block::Paragraph(Paragraph {
                        kind: ParagraphKind::Normal,
                        list: None,
                        inlines: inlines(node),
                    }));
                }
            }
            NodeValue::Heading(heading) => self.push(Block::Paragraph(Paragraph {
                kind: ParagraphKind::Heading(heading.level.clamp(1, 6)),
                list: None,
                inlines: inlines(node),
            })),
            NodeValue::BlockQuote if is_simple_quote(node) => {
                for child in node.children() {
                    self.flow(child, true, 0);
                }
            }
            NodeValue::List(_) if is_simple_list(node) => self.flow(node, false, 0),
            NodeValue::CodeBlock(code) if code.info.trim() == MEDIA_FENCE => {
                let line = code.literal.trim();
                match parse_media(line) {
                    Some(media) => self.push(Block::Media(media)),
                    None => self.push(Block::Raw(line.to_string())),
                }
            }
            NodeValue::CodeBlock(code) => self.push(Block::Code {
                language: code
                    .info
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .into(),
                code: code
                    .literal
                    .strip_suffix('\n')
                    .unwrap_or(&code.literal)
                    .into(),
            }),
            NodeValue::MathBlock(math) => {
                self.push(Block::Math(math.literal.trim_end_matches('\n').into()));
            }
            NodeValue::Table(_) => self.push(Block::Table(table(node))),
            NodeValue::ThematicBreak(_) => self.push(Block::Rule),
            _ => {
                let raw = self.source_of(node);
                self.push(Block::Raw(raw));
            }
        }
    }

    /// Emits the paragraphs of a quote or list (already checked to be simple).
    fn flow<'a>(&mut self, node: Node<'a>, quote: bool, level: u8) {
        let value = node.data().value.clone();
        let kind = if quote {
            ParagraphKind::Quote
        } else {
            ParagraphKind::Normal
        };
        match value {
            NodeValue::Paragraph => self.push(Block::Paragraph(Paragraph {
                kind,
                list: None,
                inlines: inlines(node),
            })),
            NodeValue::List(list) => {
                for item in node.children() {
                    let list_kind = match &item.data().value {
                        NodeValue::TaskItem(task) => ListKind::Task {
                            checked: task.symbol.is_some_and(|symbol| symbol != ' '),
                        },
                        _ if list.list_type == ListType::Ordered => ListKind::Numbered,
                        _ => ListKind::Bullet,
                    };
                    let mut wrote_text = false;
                    for child in item.children() {
                        if matches!(child.data().value, NodeValue::Paragraph) {
                            self.push(Block::Paragraph(Paragraph {
                                kind,
                                list: Some(ListItem {
                                    kind: list_kind,
                                    level,
                                }),
                                inlines: inlines(child),
                            }));
                            wrote_text = true;
                        } else {
                            if !wrote_text {
                                self.push(empty_item(kind, list_kind, level));
                                wrote_text = true;
                            }
                            self.flow(child, quote, level + 1);
                        }
                    }
                    if !wrote_text {
                        self.push(empty_item(kind, list_kind, level));
                    }
                }
            }
            _ => {}
        }
    }

    fn push(&mut self, block: Block) {
        self.document.blocks.push(block);
    }

    fn source_of<'a>(&self, node: Node<'a>) -> String {
        let position = node.data().sourcepos;
        let start = position.start.line.saturating_sub(1);
        let end = position.end.line.min(self.lines.len());
        let text = self.lines.get(start..end).unwrap_or_default().join("\n");
        unwrap_media_lines(&text)
    }
}

fn empty_item(kind: ParagraphKind, list_kind: ListKind, level: u8) -> Block {
    Block::Paragraph(Paragraph {
        kind,
        list: Some(ListItem {
            kind: list_kind,
            level,
        }),
        inlines: Vec::new(),
    })
}

/// A quote the editor can show as quote paragraphs: only paragraphs and simple lists.
fn is_simple_quote<'a>(node: Node<'a>) -> bool {
    node.children().all(|child| match child.data().value {
        NodeValue::Paragraph => display_math(child).is_none(),
        NodeValue::List(_) => is_simple_list(child),
        _ => false,
    })
}

/// A list whose items hold at most one paragraph followed by nested simple lists.
fn is_simple_list<'a>(node: Node<'a>) -> bool {
    node.children().all(|item| {
        let mut paragraphs = 0;
        let mut seen_list = false;
        item.children().all(|child| match child.data().value {
            NodeValue::Paragraph => {
                paragraphs += 1;
                paragraphs == 1 && !seen_list && display_math(child).is_none()
            }
            NodeValue::List(_) => {
                seen_list = true;
                is_simple_list(child)
            }
            _ => false,
        })
    })
}

/// Returns the LaTeX when a paragraph is exactly one `$$ … $$` expression.
fn display_math<'a>(node: Node<'a>) -> Option<String> {
    let mut children = node.children();
    let first = children.next()?;
    if children.next().is_some() {
        return None;
    }
    match &first.data().value {
        NodeValue::Math(math) if math.display_math => Some(math.literal.clone()),
        _ => None,
    }
}

pub(crate) fn inlines<'a>(node: Node<'a>) -> Vec<Inline> {
    let mut output = Vec::new();
    collect_inlines(node, Style::default(), None, &mut output);
    merge_inlines(output)
}

fn collect_inlines<'a>(node: Node<'a>, style: Style, link: Option<&Link>, out: &mut Vec<Inline>) {
    for child in node.children() {
        let value = child.data().value.clone();
        let push = |out: &mut Vec<Inline>, text: String, style: Style| {
            out.push(Inline {
                text,
                style,
                link: link.cloned(),
            });
        };
        match value {
            NodeValue::Text(text) => push(out, text.into_owned(), style),
            NodeValue::SoftBreak => push(out, " ".into(), style),
            NodeValue::LineBreak => push(out, "\n".into(), style),
            NodeValue::Code(code) => push(
                out,
                code.literal,
                Style {
                    code: true,
                    ..style
                },
            ),
            NodeValue::Math(math) if math.display_math => {
                push(
                    out,
                    format!("$${}$$", math.literal),
                    Style { raw: true, ..style },
                );
            }
            NodeValue::Math(math) => push(
                out,
                math.literal,
                Style {
                    math: true,
                    ..style
                },
            ),
            NodeValue::Emph(_) => {
                collect_inlines(
                    child,
                    Style {
                        italic: true,
                        ..style
                    },
                    link,
                    out,
                );
            }
            NodeValue::Strong(_) => {
                collect_inlines(
                    child,
                    Style {
                        bold: true,
                        ..style
                    },
                    link,
                    out,
                );
            }
            NodeValue::Strikethrough(_) => collect_inlines(
                child,
                Style {
                    strikethrough: true,
                    ..style
                },
                link,
                out,
            ),
            NodeValue::Highlight => collect_inlines(
                child,
                Style {
                    highlight: true,
                    ..style
                },
                link,
                out,
            ),
            NodeValue::Link(target) => {
                let target = Link::Url(target.url.clone());
                collect_inlines(child, style, Some(&target), out);
            }
            NodeValue::WikiLink(target) => {
                let target = Link::Page(target.url.clone());
                collect_inlines(child, style, Some(&target), out);
            }
            NodeValue::Image(image) => {
                let alt = plain_text(child);
                let title = if image.title.is_empty() {
                    String::new()
                } else {
                    format!(" \"{}\"", image.title)
                };
                push(
                    out,
                    format!("![{alt}]({}{title})", image.url),
                    Style { raw: true, ..style },
                );
            }
            NodeValue::HtmlInline(html) => push(out, html, Style { raw: true, ..style }),
            NodeValue::FootnoteReference(reference) => push(
                out,
                format!("[^{}]", reference.name),
                Style { raw: true, ..style },
            ),
            _ => collect_inlines(child, style, link, out),
        }
    }
}

fn plain_text<'a>(node: Node<'a>) -> String {
    let mut text = String::new();
    for descendant in node.descendants() {
        match &descendant.data().value {
            NodeValue::Text(value) => text.push_str(value),
            NodeValue::Code(code) => text.push_str(&code.literal),
            NodeValue::SoftBreak | NodeValue::LineBreak => text.push(' '),
            _ => {}
        }
    }
    text
}

/// Joins neighboring runs that share style and link.
pub fn merge_inlines(inlines: Vec<Inline>) -> Vec<Inline> {
    let mut merged: Vec<Inline> = Vec::with_capacity(inlines.len());
    for inline in inlines {
        if inline.text.is_empty() {
            continue;
        }
        match merged.last_mut() {
            Some(last)
                if last.style == inline.style
                    && last.link == inline.link
                    && !inline.style.raw
                    && !inline.style.math =>
            {
                last.text.push_str(&inline.text);
            }
            _ => merged.push(inline),
        }
    }
    merged
}

fn table<'a>(node: Node<'a>) -> Table {
    let alignments = match &node.data().value {
        NodeValue::Table(table) => table
            .alignments
            .iter()
            .map(|alignment| match alignment {
                TableAlignment::None => Alignment::None,
                TableAlignment::Left => Alignment::Left,
                TableAlignment::Center => Alignment::Center,
                TableAlignment::Right => Alignment::Right,
            })
            .collect(),
        _ => Vec::new(),
    };
    let mut rows = node
        .children()
        .map(|row| row.children().map(inlines).collect::<Vec<_>>());
    let header = rows.next().unwrap_or_default();
    Table {
        alignments,
        header,
        rows: rows.collect(),
    }
}

fn parse_properties(front_matter: &str) -> Properties {
    let body: Vec<&str> = front_matter
        .trim_end()
        .lines()
        .skip(1)
        .collect::<Vec<_>>()
        .split_last()
        .map(|(_closing, lines)| lines.to_vec())
        .unwrap_or_default();
    let mut entries = Vec::new();
    for line in &body {
        if line.trim().is_empty() {
            continue;
        }
        let simple = !line.starts_with([' ', '\t', '#', '-'])
            && line.split_once(':').is_some_and(|(key, value)| {
                !key.is_empty() && (value.is_empty() || value.starts_with(' '))
            });
        if !simple {
            return Properties::Raw(body.join("\n"));
        }
        let (key, value) = line.split_once(':').unwrap_or_default();
        entries.push(Property {
            key: key.trim().into(),
            value: value.trim().into(),
        });
    }
    if entries.is_empty() {
        Properties::None
    } else {
        Properties::Entries(entries)
    }
}

/// Parses `:::image type="content" source="…" alt-text="…":::`.
pub(crate) fn parse_media(line: &str) -> Option<Media> {
    let inner = line.trim().strip_prefix(":::image")?.strip_suffix(":::")?;
    let mut attributes = Vec::new();
    let mut rest = inner.trim_start();
    while !rest.is_empty() {
        let (key, after_key) = rest.split_once('=')?;
        let key = key.trim();
        if key.is_empty() || key.contains(char::is_whitespace) {
            return None;
        }
        let after_key = after_key.trim_start();
        let (value, remainder) = if let Some(quoted) = after_key.strip_prefix('"') {
            let end = quoted.find('"')?;
            (&quoted[..end], &quoted[end + 1..])
        } else {
            let end = after_key
                .find(char::is_whitespace)
                .unwrap_or(after_key.len());
            (&after_key[..end], &after_key[end..])
        };
        attributes.push((key.to_string(), value.to_string()));
        rest = remainder.trim_start();
    }
    Some(Media { attributes })
}

fn is_media_line(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with(":::image")
        && trimmed.ends_with(":::")
        && trimmed.len() > ":::image:::".len() - 1
}

/// Replaces each `:::image …:::` line outside code fences with a fenced block so comrak keeps
/// it intact (URLs and alt text would otherwise be parsed as emphasis or links).
fn wrap_media_lines(source: &str) -> String {
    let mut output = String::with_capacity(source.len() + 64);
    let mut fence: Option<String> = None;
    for line in source.split('\n') {
        let trimmed = line.trim_start();
        if let Some(open) = &fence {
            if trimmed.starts_with(open.as_str())
                && trimmed
                    .trim_end()
                    .chars()
                    .all(|c| c == open.chars().next().unwrap_or('`'))
            {
                fence = None;
            }
        } else if let Some(marker) = fence_marker(trimmed) {
            fence = Some(marker);
        } else if is_media_line(line) {
            let indent = &line[..line.len() - trimmed.len()];
            output.push_str(&format!(
                "{indent}~~~{MEDIA_FENCE}\n{indent}{}\n{indent}~~~\n",
                trimmed.trim_end()
            ));
            continue;
        }
        output.push_str(line);
        output.push('\n');
    }
    if !source.ends_with('\n') {
        output.pop();
    }
    output
}

fn fence_marker(trimmed: &str) -> Option<String> {
    for symbol in ['`', '~'] {
        let count = trimmed.chars().take_while(|&c| c == symbol).count();
        if count >= 3 {
            return Some(symbol.to_string().repeat(count));
        }
    }
    None
}

/// Reverses [`wrap_media_lines`] inside text kept as raw Markdown.
fn unwrap_media_lines(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut output = Vec::with_capacity(lines.len());
    let mut index = 0;
    while index < lines.len() {
        if lines[index].trim() == format!("~~~{MEDIA_FENCE}")
            && index + 2 < lines.len()
            && lines[index + 2].trim() == "~~~"
        {
            output.push(lines[index + 1]);
            index += 3;
        } else {
            output.push(lines[index]);
            index += 1;
        }
    }
    output.join("\n")
}
