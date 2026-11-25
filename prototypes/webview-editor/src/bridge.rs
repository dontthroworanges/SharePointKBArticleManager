//! Converts between the app's document model (`src/markdown`, used read-only) and the
//! ProseMirror JSON that Tiptap edits.
//!
//! The app's own parser and serializer stay in charge of Markdown, so the prototype edits
//! exactly the SharePoint dialect the app does (front matter, `==highlight==`, `[[Page]]`
//! links, `:::image:::`, math). Tiptap only ever sees a tree of nodes, and anything it has
//! no editor for (math, media, raw Markdown) travels as an atom node holding its source.
//!
//! Node and mark names match `web/editor.js`.

use crate::markdown::*;
use serde_json::{Value, json};

/// The document's blocks as a ProseMirror `doc` node. Front matter is not sent; the caller
/// keeps it and puts it back on save.
pub fn to_pm(document: &Document) -> Value {
    let blocks = &document.blocks;
    let mut content = Vec::new();
    let mut index = 0;
    while index < blocks.len() {
        match &blocks[index] {
            Block::Paragraph(paragraph) if paragraph.list.is_some() => {
                let start = index;
                while matches!(blocks.get(index), Some(Block::Paragraph(p)) if p.list.is_some()) {
                    index += 1;
                }
                let items: Vec<&Paragraph> = blocks[start..index]
                    .iter()
                    .filter_map(|block| match block {
                        Block::Paragraph(paragraph) => Some(paragraph),
                        _ => None,
                    })
                    .collect();
                let mut at = 0;
                while at < items.len() {
                    let level = level_of(items[at]);
                    content.push(list_node(&items, &mut at, level));
                }
                continue;
            }
            Block::Paragraph(paragraph) if paragraph.kind == ParagraphKind::Quote => {
                let mut quoted = Vec::new();
                while let Some(Block::Paragraph(p)) = blocks.get(index) {
                    if p.kind != ParagraphKind::Quote || p.list.is_some() {
                        break;
                    }
                    quoted.push(paragraph_node(p));
                    index += 1;
                }
                content.push(json!({ "type": "blockquote", "content": quoted }));
                continue;
            }
            Block::Paragraph(paragraph) => content.push(paragraph_node(paragraph)),
            Block::Code { language, code } => content.push(node(
                "codeBlock",
                json!({ "language": language }),
                text_content(code),
            )),
            Block::Math(latex) => content.push(atom("mathBlock", json!({ "latex": latex }))),
            Block::Table(table) => content.push(table_node(table)),
            Block::Media(media) => content.push(atom(
                "media",
                json!({ "attributes": media.attributes.iter()
                    .map(|(key, value)| json!([key, value]))
                    .collect::<Vec<_>>() }),
            )),
            Block::Rule => content.push(json!({ "type": "horizontalRule" })),
            Block::Raw(markdown) => content.push(atom("rawBlock", json!({ "markdown": markdown }))),
        }
        index += 1;
    }
    if content.is_empty() {
        content.push(json!({ "type": "paragraph" }));
    }
    json!({ "type": "doc", "content": content })
}

/// The blocks of a ProseMirror `doc` node.
pub fn from_pm(doc: &Value) -> Vec<Block> {
    let mut blocks = Vec::new();
    for child in children(doc) {
        block(child, false, &mut blocks);
    }
    // The editor always keeps a paragraph to type in at the end; an empty one is not content.
    while matches!(blocks.last(), Some(Block::Paragraph(p)) if p.list.is_none() && p.kind == ParagraphKind::Normal && p.is_empty())
    {
        blocks.pop();
    }
    blocks
}

fn level_of(paragraph: &Paragraph) -> u8 {
    paragraph.list.map_or(0, |list| list.level)
}

/// Lists of one family nest by level; a change of family at the same level starts a new list.
fn family(kind: ListKind) -> &'static str {
    match kind {
        ListKind::Bullet => "bulletList",
        ListKind::Numbered => "orderedList",
        ListKind::Task { .. } => "taskList",
    }
}

fn list_node(items: &[&Paragraph], at: &mut usize, level: u8) -> Value {
    let kind = items[*at].list.map_or(ListKind::Bullet, |list| list.kind);
    let list_type = family(kind);
    let mut entries: Vec<Value> = Vec::new();
    while let Some(item) = items.get(*at) {
        let Some(list) = item.list else { break };
        if list.level < level {
            break;
        }
        if list.level > level {
            // A deeper item belongs in the item before it.
            if entries.is_empty() {
                entries.push(item_node(kind, &Paragraph::default()));
            }
            let nested = list_node(items, at, list.level);
            if let Some(Value::Array(content)) = entries
                .last_mut()
                .and_then(|entry| entry.get_mut("content"))
            {
                content.push(nested);
            }
            continue;
        }
        if family(list.kind) != list_type {
            break;
        }
        entries.push(item_node(list.kind, item));
        *at += 1;
    }
    json!({ "type": list_type, "content": entries })
}

fn item_node(kind: ListKind, paragraph: &Paragraph) -> Value {
    let text = paragraph_node(&Paragraph {
        kind: ParagraphKind::Normal,
        list: None,
        inlines: paragraph.inlines.clone(),
    });
    match kind {
        ListKind::Task { checked } => json!({
            "type": "taskItem",
            "attrs": { "checked": checked },
            "content": [text],
        }),
        _ => json!({ "type": "listItem", "content": [text] }),
    }
}

fn paragraph_node(paragraph: &Paragraph) -> Value {
    let inlines = inline_nodes(&paragraph.inlines);
    match paragraph.kind {
        ParagraphKind::Heading(level) => node("heading", json!({ "level": level }), inlines),
        _ => {
            let mut value = json!({ "type": "paragraph" });
            if !inlines.is_empty() {
                value["content"] = Value::Array(inlines);
            }
            value
        }
    }
}

fn table_node(table: &Table) -> Value {
    let cell = |kind: &str, inlines: &[Inline]| {
        json!({
            "type": kind,
            "content": [paragraph_node(&Paragraph { inlines: inlines.to_vec(), ..Paragraph::default() })],
        })
    };
    let mut rows = vec![json!({
        "type": "tableRow",
        "content": table.header.iter().map(|inlines| cell("tableHeader", inlines)).collect::<Vec<_>>(),
    })];
    for row in &table.rows {
        rows.push(json!({
            "type": "tableRow",
            "content": row.iter().map(|inlines| cell("tableCell", inlines)).collect::<Vec<_>>(),
        }));
    }
    let alignments: Vec<&str> = table
        .alignments
        .iter()
        .map(|alignment| match alignment {
            Alignment::None => "none",
            Alignment::Left => "left",
            Alignment::Center => "center",
            Alignment::Right => "right",
        })
        .collect();
    json!({ "type": "table", "attrs": { "alignments": alignments }, "content": rows })
}

fn inline_nodes(inlines: &[Inline]) -> Vec<Value> {
    let mut nodes = Vec::new();
    for inline in inlines {
        if inline.style.math {
            nodes.push(atom("mathInline", json!({ "latex": inline.text })));
            continue;
        }
        if inline.style.raw {
            nodes.push(atom("rawInline", json!({ "markdown": inline.text })));
            continue;
        }
        let marks = marks(inline);
        for (index, part) in inline.text.split('\n').enumerate() {
            if index > 0 {
                nodes.push(json!({ "type": "hardBreak" }));
            }
            if !part.is_empty() {
                let mut text = json!({ "type": "text", "text": part });
                if !marks.is_empty() {
                    text["marks"] = Value::Array(marks.clone());
                }
                nodes.push(text);
            }
        }
    }
    nodes
}

fn marks(inline: &Inline) -> Vec<Value> {
    let mut marks = Vec::new();
    match &inline.link {
        Some(Link::Url(href)) => marks.push(json!({ "type": "link", "attrs": { "href": href } })),
        Some(Link::Page(page)) => {
            marks.push(json!({ "type": "pageLink", "attrs": { "page": page } }))
        }
        None => {}
    }
    let style = inline.style;
    for (on, name) in [
        (style.bold, "bold"),
        (style.italic, "italic"),
        (style.strikethrough, "strike"),
        (style.highlight, "highlight"),
        (style.code, "code"),
    ] {
        if on {
            marks.push(json!({ "type": name }));
        }
    }
    marks
}

fn block(node: &Value, quote: bool, out: &mut Vec<Block>) {
    let attrs = &node["attrs"];
    match type_of(node) {
        "paragraph" => out.push(Block::Paragraph(Paragraph {
            kind: if quote {
                ParagraphKind::Quote
            } else {
                ParagraphKind::Normal
            },
            list: None,
            inlines: inlines(node),
        })),
        "heading" => out.push(Block::Paragraph(Paragraph {
            kind: ParagraphKind::Heading(attrs["level"].as_u64().unwrap_or(1).clamp(1, 6) as u8),
            list: None,
            inlines: inlines(node),
        })),
        "blockquote" => {
            for child in children(node) {
                block(child, true, out);
            }
        }
        "bulletList" | "orderedList" | "taskList" => list(node, 0, out),
        "codeBlock" => out.push(Block::Code {
            language: attrs["language"].as_str().unwrap_or_default().to_string(),
            code: plain_text(node),
        }),
        "mathBlock" => out.push(Block::Math(string(&attrs["latex"]))),
        "media" => out.push(Block::Media(Media {
            attributes: attrs["attributes"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|pair| Some((pair[0].as_str()?.to_string(), pair[1].as_str()?.to_string())))
                .collect(),
        })),
        "rawBlock" => out.push(Block::Raw(string(&attrs["markdown"]))),
        "horizontalRule" => out.push(Block::Rule),
        "table" => out.push(Block::Table(table(node))),
        // Nothing else is in the editor's schema; keep any text rather than drop it.
        _ => out.push(Block::Paragraph(Paragraph {
            inlines: vec![Inline::plain(plain_text(node))],
            ..Paragraph::default()
        })),
    }
}

fn list(node: &Value, level: u8, out: &mut Vec<Block>) {
    let list_type = type_of(node);
    for item in children(node) {
        let kind = match list_type {
            "orderedList" => ListKind::Numbered,
            "taskList" => ListKind::Task {
                checked: item["attrs"]["checked"].as_bool().unwrap_or(false),
            },
            _ => ListKind::Bullet,
        };
        for child in children(item) {
            match type_of(child) {
                "bulletList" | "orderedList" | "taskList" => list(child, level + 1, out),
                "paragraph" | "heading" => out.push(Block::Paragraph(Paragraph {
                    kind: ParagraphKind::Normal,
                    list: Some(ListItem { kind, level }),
                    inlines: inlines(child),
                })),
                _ => block(child, false, out),
            }
        }
    }
}

fn table(node: &Value) -> Table {
    let cells = |row: &Value| -> Vec<Vec<Inline>> {
        children(row)
            .map(|cell| {
                let mut inlines_out = Vec::new();
                for (index, paragraph) in children(cell).enumerate() {
                    if index > 0 {
                        inlines_out.push(Inline::plain("\n"));
                    }
                    inlines_out.extend(inlines(paragraph));
                }
                merge(inlines_out)
            })
            .collect()
    };
    let mut rows = children(node);
    let header = rows.next().map(cells).unwrap_or_default();
    let rows: Vec<_> = rows.map(cells).collect();
    let mut alignments: Vec<Alignment> = node["attrs"]["alignments"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|alignment| match alignment.as_str() {
            Some("left") => Alignment::Left,
            Some("center") => Alignment::Center,
            Some("right") => Alignment::Right,
            _ => Alignment::None,
        })
        .collect();
    // Columns added in the editor have no alignment yet.
    alignments.resize(header.len(), Alignment::None);
    Table {
        alignments,
        header,
        rows,
    }
}

fn inlines(node: &Value) -> Vec<Inline> {
    let mut out: Vec<Inline> = Vec::new();
    for child in children(node) {
        let attrs = &child["attrs"];
        match type_of(child) {
            "text" => {
                let mut inline = Inline::plain(string(&child["text"]));
                for mark in child["marks"].as_array().into_iter().flatten() {
                    match type_of(mark) {
                        "bold" => inline.style.bold = true,
                        "italic" => inline.style.italic = true,
                        "strike" => inline.style.strikethrough = true,
                        "highlight" => inline.style.highlight = true,
                        "code" => inline.style.code = true,
                        "link" => inline.link = Some(Link::Url(string(&mark["attrs"]["href"]))),
                        "pageLink" => {
                            inline.link = Some(Link::Page(string(&mark["attrs"]["page"])))
                        }
                        _ => {}
                    }
                }
                out.push(inline);
            }
            // A line break carries the formatting around it, so it joins that run.
            "hardBreak" => {
                let mut inline = out
                    .last()
                    .filter(|last| !last.style.math && !last.style.raw)
                    .cloned()
                    .unwrap_or_default();
                inline.text = "\n".into();
                out.push(inline);
            }
            "mathInline" => out.push(Inline::styled(
                string(&attrs["latex"]),
                Style {
                    math: true,
                    ..Style::default()
                },
            )),
            "rawInline" => out.push(Inline::styled(
                string(&attrs["markdown"]),
                Style {
                    raw: true,
                    ..Style::default()
                },
            )),
            _ => out.push(Inline::plain(plain_text(child))),
        }
    }
    merge(out)
}

fn node(kind: &str, attrs: Value, content: Vec<Value>) -> Value {
    let mut value = json!({ "type": kind, "attrs": attrs });
    if !content.is_empty() {
        value["content"] = Value::Array(content);
    }
    value
}

fn atom(kind: &str, attrs: Value) -> Value {
    json!({ "type": kind, "attrs": attrs })
}

fn text_content(text: &str) -> Vec<Value> {
    if text.is_empty() {
        Vec::new()
    } else {
        vec![json!({ "type": "text", "text": text })]
    }
}

fn type_of(node: &Value) -> &str {
    node["type"].as_str().unwrap_or_default()
}

fn children(node: &Value) -> impl Iterator<Item = &Value> {
    node["content"].as_array().into_iter().flatten()
}

fn string(value: &Value) -> String {
    value.as_str().unwrap_or_default().to_string()
}

fn plain_text(node: &Value) -> String {
    if let Some(text) = node["text"].as_str() {
        return text.to_string();
    }
    if type_of(node) == "hardBreak" {
        return "\n".into();
    }
    children(node).map(plain_text).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every sample survives model → editor JSON → model with the same Markdown, so what
    /// Tiptap is handed is all it needs to give the file back unchanged.
    #[test]
    fn samples_round_trip() {
        let samples = concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples");
        let mut checked = 0;
        for entry in std::fs::read_dir(samples).expect("samples folder") {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|extension| extension != "md") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let document = parse(&text);
            let expected = serialize(&document);
            let blocks = from_pm(&to_pm(&document));
            let actual = serialize(&Document {
                properties: document.properties.clone(),
                blocks,
            });
            assert_eq!(actual, expected, "{}", path.display());
            checked += 1;
        }
        assert!(checked > 0);
    }

    #[test]
    fn nested_and_mixed_lists() {
        let text = "- one\n  - two\n    - three\n- four\n\n1. first\n2. second\n\n- [ ] task\n- [x] done\n";
        let document = parse(text);
        let back = serialize(&Document {
            properties: Properties::None,
            blocks: from_pm(&to_pm(&document)),
        });
        assert_eq!(back, serialize(&document));
    }

    #[test]
    fn empty_document() {
        let document = parse("");
        assert!(from_pm(&to_pm(&document)).is_empty());
    }
}
