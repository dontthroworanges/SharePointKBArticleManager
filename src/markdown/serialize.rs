//! [`Document`] → Markdown in the form SharePoint Knowledge Base pages use.

use super::model::*;

pub fn serialize(document: &Document) -> String {
    let mut output = String::new();
    match &document.properties {
        Properties::None => {}
        Properties::Entries(entries) => {
            output.push_str("---\n");
            for entry in entries.iter().filter(|entry| !entry.key.trim().is_empty()) {
                if entry.value.is_empty() {
                    output.push_str(&format!("{}:\n", entry.key.trim()));
                } else {
                    output.push_str(&format!("{}: {}\n", entry.key.trim(), entry.value));
                }
            }
            output.push_str("---\n\n");
        }
        Properties::Raw(yaml) => output.push_str(&format!("---\n{}\n---\n\n", yaml.trim_end())),
    }

    let mut lists = ListState::default();
    let mut previous: Option<&Block> = None;
    for block in &document.blocks {
        if let Block::Paragraph(paragraph) = block
            && paragraph.list.is_none()
            && paragraph.is_empty()
        {
            continue;
        }
        if let Some(previous) = previous {
            output.push_str(separator(previous, block));
        }
        match block {
            Block::Paragraph(paragraph) => output.push_str(&lists.paragraph(paragraph)),
            other => {
                lists = ListState::default();
                output.push_str(&object(other));
            }
        }
        previous = Some(block);
    }
    let trimmed = output.trim_end().len();
    output.truncate(trimmed);
    output.push('\n');
    output
}

fn separator(previous: &Block, next: &Block) -> &'static str {
    match (previous, next) {
        (Block::Paragraph(a), Block::Paragraph(b)) => {
            let a_quote = a.kind == ParagraphKind::Quote;
            let b_quote = b.kind == ParagraphKind::Quote;
            let numbered = |item: &ListItem| item.kind == ListKind::Numbered;
            let new_list = matches!((a.list, b.list), (Some(x), Some(y))
                if y.level == 0 && numbered(&x) != numbered(&y));
            if a.list.is_some() && b.list.is_some() && a_quote == b_quote && !new_list {
                "\n"
            } else if a_quote && b_quote {
                "\n>\n"
            } else {
                "\n\n"
            }
        }
        _ => "\n\n",
    }
}

/// Tracks list indentation and numbering across consecutive list paragraphs.
#[derive(Default)]
struct ListState {
    /// Column where item content starts, for each open level.
    content_columns: Vec<usize>,
    /// Next number for each open level, when that level is a numbered list.
    numbers: Vec<Option<u32>>,
}

impl ListState {
    fn paragraph(&mut self, paragraph: &Paragraph) -> String {
        let quote = if paragraph.kind == ParagraphKind::Quote {
            "> "
        } else {
            ""
        };
        let Some(item) = paragraph.list else {
            *self = Self::default();
            let body = match paragraph.kind {
                ParagraphKind::Heading(level) => {
                    let text = inlines_to_markdown(&paragraph.inlines, false).replace("\\\n", " ");
                    return format!("{} {}", "#".repeat(level.clamp(1, 6) as usize), text.trim());
                }
                _ => escape_line_start(&inlines_to_markdown(&paragraph.inlines, false)),
            };
            return prefix_lines(&body, quote, quote);
        };

        let level = (item.level as usize).min(self.content_columns.len());
        let indent = if level == 0 {
            0
        } else {
            self.content_columns[level - 1]
        };
        self.content_columns.truncate(level);
        self.numbers.truncate(level + 1);
        let marker = match item.kind {
            ListKind::Bullet => "- ".to_string(),
            ListKind::Task { checked } => format!("- [{}] ", if checked { 'x' } else { ' ' }),
            ListKind::Numbered => {
                let number = self.numbers.get(level).copied().flatten().unwrap_or(1);
                self.numbers.resize(level + 1, None);
                self.numbers[level] = Some(number + 1);
                format!("{number}. ")
            }
        };
        if item.kind != ListKind::Numbered {
            self.numbers.resize(level + 1, None);
            self.numbers[level] = None;
        }
        let content_column = indent
            + if matches!(item.kind, ListKind::Task { .. }) {
                2
            } else {
                marker.len()
            };
        self.content_columns.push(content_column);

        let body = escape_line_start(&inlines_to_markdown(&paragraph.inlines, false));
        let first = format!("{quote}{}{marker}", " ".repeat(indent));
        let rest = format!("{quote}{}", " ".repeat(content_column));
        prefix_lines(&body, &first, &rest)
    }
}

fn prefix_lines(body: &str, first: &str, rest: &str) -> String {
    let mut output = String::new();
    for (index, line) in body.split('\n').enumerate() {
        if index > 0 {
            output.push('\n');
            output.push_str(rest);
        } else {
            output.push_str(first);
        }
        output.push_str(line);
    }
    output
}

fn object(block: &Block) -> String {
    match block {
        Block::Paragraph(_) => unreachable!("paragraphs are written by ListState"),
        Block::Code { language, code } => {
            let longest = longest_run(code, '`');
            let fence = "`".repeat(longest.max(2) + 1);
            format!("{fence}{}\n{code}\n{fence}", language.trim())
        }
        Block::Math(tex) => format!("$$\n{}\n$$", tex.trim()),
        Block::Table(table) => table_to_markdown(table),
        Block::Media(media) => media_to_markdown(media),
        Block::Rule => "---".into(),
        Block::Raw(text) => text.trim_end().to_string(),
    }
}

pub fn media_to_markdown(media: &Media) -> String {
    let attributes: Vec<String> = media
        .attributes
        .iter()
        .map(|(key, value)| format!("{key}=\"{}\"", value.replace('"', "'")))
        .collect();
    format!(":::image {}:::", attributes.join(" "))
}

fn table_to_markdown(table: &Table) -> String {
    let columns = table.columns().max(1);
    let row = |cells: &[Vec<Inline>]| {
        let texts: Vec<String> = (0..columns)
            .map(|column| {
                cells
                    .get(column)
                    .map(|cell| inlines_to_markdown(cell, true))
                    .unwrap_or_default()
            })
            .collect();
        format!("| {} |", texts.join(" | "))
    };
    let mut lines = vec![row(&table.header)];
    let alignments: Vec<&str> = (0..columns)
        .map(
            |column| match table.alignments.get(column).copied().unwrap_or_default() {
                Alignment::None => "---",
                Alignment::Left => ":---",
                Alignment::Center => ":---:",
                Alignment::Right => "---:",
            },
        )
        .collect();
    lines.push(format!("| {} |", alignments.join(" | ")));
    lines.extend(table.rows.iter().map(|cells| row(cells)));
    lines.join("\n")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mark {
    Bold,
    Italic,
    Strikethrough,
    Highlight,
}

impl Mark {
    fn delimiter(self) -> &'static str {
        match self {
            Self::Bold => "**",
            Self::Italic => "*",
            Self::Strikethrough => "~~",
            Self::Highlight => "==",
        }
    }
}

fn marks(style: Style) -> Vec<Mark> {
    let mut marks = Vec::new();
    if style.bold {
        marks.push(Mark::Bold);
    }
    if style.italic {
        marks.push(Mark::Italic);
    }
    if style.strikethrough {
        marks.push(Mark::Strikethrough);
    }
    if style.highlight {
        marks.push(Mark::Highlight);
    }
    marks
}

/// Serializes inline runs. Line breaks become `\` + newline; in tables they become spaces.
pub fn inlines_to_markdown(inlines: &[Inline], in_table: bool) -> String {
    let mut output = String::new();
    let mut index = 0;
    while index < inlines.len() {
        let link = inlines[index].link.clone();
        let mut end = index + 1;
        while end < inlines.len() && inlines[end].link == link {
            end += 1;
        }
        let group = &inlines[index..end];
        match &link {
            None => output.push_str(&marked_runs(group, in_table)),
            Some(Link::Url(url)) => {
                let label = marked_runs(group, in_table);
                output.push_str(&format!("[{label}]({})", escape_url(url)));
            }
            Some(Link::Page(page)) => {
                let label: String = group.iter().map(|inline| inline.text.as_str()).collect();
                if label.trim() == page.trim() || label.trim().is_empty() {
                    output.push_str(&format!("[[{}]]", page.trim()));
                } else {
                    output.push_str(&format!("[[{}|{}]]", page.trim(), label.trim()));
                }
            }
        }
        index = end;
    }
    output
}

fn desired_marks(run: &Inline) -> Vec<Mark> {
    if run.style.raw {
        Vec::new()
    } else {
        marks(run.style)
    }
}

fn common_prefix(a: &[Mark], b: &[Mark]) -> usize {
    a.iter().zip(b).take_while(|(a, b)| a == b).count()
}

/// Writes runs with the fewest delimiters: marks shared by neighboring runs stay open.
/// Whitespace where a mark opens or closes moves outside it, because `** bold**` is not
/// valid emphasis.
fn marked_runs(runs: &[Inline], in_table: bool) -> String {
    let mut output = String::new();
    let mut open: Vec<Mark> = Vec::new();
    let mut pending_space = String::new();
    for (index, run) in runs.iter().enumerate() {
        let plain_text = !(run.style.code || run.style.math || run.style.raw);
        let mut desired = desired_marks(run);
        if plain_text && run.text.trim().is_empty() {
            // Whitespace alone never opens a mark.
            desired.truncate(common_prefix(&open, &desired));
        }
        let keep = common_prefix(&open, &desired);
        for mark in open.drain(keep..).rev() {
            output.push_str(mark.delimiter());
        }
        output.push_str(&pending_space);
        pending_space.clear();

        let previous = index
            .checked_sub(1)
            .and_then(|i| runs[i].text.chars().last());
        let next = runs.get(index + 1).and_then(|r| r.text.chars().next());
        let mut content = run_content(run, in_table, previous, next);

        if plain_text && desired.len() > keep {
            let trimmed = content.trim_start();
            output.push_str(&content[..content.len() - trimmed.len()]);
            content = trimmed.to_string();
        }
        for mark in &desired[keep..] {
            output.push_str(mark.delimiter());
            open.push(*mark);
        }

        let next_desired = runs.get(index + 1).map(desired_marks).unwrap_or_default();
        let closing = common_prefix(&open, &next_desired) < open.len();
        if plain_text && closing {
            let trimmed = content.trim_end();
            pending_space = content[trimmed.len()..].to_string();
            content.truncate(trimmed.len());
        }
        output.push_str(&content);
    }
    for mark in open.into_iter().rev() {
        output.push_str(mark.delimiter());
    }
    output.push_str(&pending_space);
    output
}

fn run_content(run: &Inline, in_table: bool, previous: Option<char>, next: Option<char>) -> String {
    let text = if in_table {
        run.text.replace('\n', " ")
    } else {
        run.text.clone()
    };
    if run.style.raw {
        return text;
    }
    if run.style.math {
        return format!("${}$", text.trim());
    }
    if run.style.code {
        let ticks = "`".repeat(longest_run(&text, '`') + 1);
        let pad = if text.starts_with('`') || text.ends_with('`') {
            " "
        } else {
            ""
        };
        let text = if in_table {
            text.replace('|', "\\|")
        } else {
            text
        };
        return format!("{ticks}{pad}{text}{pad}{ticks}");
    }
    escape_text(&text, in_table, previous, next).replace('\n', "\\\n")
}

fn escape_text(text: &str, in_table: bool, before: Option<char>, after: Option<char>) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut output = String::with_capacity(text.len() + 8);
    for (index, &ch) in chars.iter().enumerate() {
        let previous = if index == 0 {
            before
        } else {
            Some(chars[index - 1])
        };
        let next = chars.get(index + 1).copied().or(after);
        let escape = match ch {
            '\\' | '*' | '`' | '[' | ']' | '<' | '$' | '~' => true,
            '|' => in_table,
            '_' => {
                !(previous.is_some_and(char::is_alphanumeric)
                    && next.is_some_and(char::is_alphanumeric))
            }
            '=' => previous == Some('=') || next == Some('='),
            '!' => next == Some('['),
            // Only text that would read as an entity, such as `&amp;` or `&#38;`.
            '&' => {
                let rest = &chars[index + 1..];
                let name = rest
                    .iter()
                    .take_while(|c| c.is_alphanumeric() || **c == '#')
                    .count();
                name > 0 && rest.get(name) == Some(&';')
            }
            _ => false,
        };
        if escape {
            output.push('\\');
        }
        output.push(ch);
    }
    output
}

/// Escapes characters that would turn the start of a paragraph line into another block.
fn escape_line_start(body: &str) -> String {
    body.split('\n')
        .map(|line| {
            let trimmed = line.trim_start();
            let indent = &line[..line.len() - trimmed.len()];
            let digits = trimmed.chars().take_while(char::is_ascii_digit).count();
            let needs_escape = trimmed.starts_with(['#', '>', '+', '-'])
                || trimmed.starts_with(":::")
                || (digits > 0 && trimmed[digits..].starts_with(['.', ')']));
            if !needs_escape {
                return line.to_string();
            }
            if digits > 0 {
                format!("{indent}{}\\{}", &trimmed[..digits], &trimmed[digits..])
            } else {
                format!("{indent}\\{trimmed}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn escape_url(url: &str) -> String {
    if url.contains([' ', '(', ')', '<', '>']) {
        format!("<{}>", url.replace('<', "%3C").replace('>', "%3E"))
    } else {
        url.to_string()
    }
}

fn longest_run(text: &str, symbol: char) -> usize {
    let mut longest = 0;
    let mut current = 0;
    for ch in text.chars() {
        if ch == symbol {
            current += 1;
            longest = longest.max(current);
        } else {
            current = 0;
        }
    }
    longest
}
