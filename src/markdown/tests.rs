use super::*;

/// Canonical Markdown must survive parse → serialize unchanged.
fn assert_round_trip(markdown: &str) {
    let document = parse(markdown);
    let output = serialize(&document);
    assert_eq!(output, markdown, "model: {document:#?}");
}

/// Serializing a model and parsing it back must give the same model.
fn assert_model_round_trip(document: &Document) {
    let markdown = serialize(document);
    let parsed = parse(&markdown);
    assert_eq!(&parsed, document, "markdown:\n{markdown}");
}

#[test]
fn headings() {
    assert_round_trip("# One\n\n## Two\n\n### Three\n\n#### Four\n\n##### Five\n\n###### Six\n");
}

#[test]
fn inline_formatting() {
    assert_round_trip("Plain **bold** *italic* ~~old~~ ==review== `code` and $x^2$ math.\n");
}

#[test]
fn nested_formatting() {
    assert_round_trip("**bold *both* bold** and ***all***\n");
}

#[test]
fn links() {
    assert_round_trip(
        "See [Microsoft Support](https://support.microsoft.com) or [[Page name]] or [[Other page|this page]].\n",
    );
    let document = parse("[[Page name]]\n");
    let Block::Paragraph(paragraph) = &document.blocks[0] else {
        panic!()
    };
    assert_eq!(
        paragraph.inlines[0].link,
        Some(Link::Page("Page name".into()))
    );
}

#[test]
fn lists() {
    assert_round_trip("- First item\n- Second item\n  - Nested item\n- Third item\n");
    assert_round_trip("1. First step\n2. Second step\n   1. Sub step\n3. Third step\n");
    assert_round_trip("- [ ] To do\n- [x] Completed\n");
}

#[test]
fn quotes() {
    assert_round_trip("> Quoted text\n>\n> Second paragraph\n");
    assert_round_trip("> - Quoted list\n> - Second item\n");
}

#[test]
fn code_blocks() {
    assert_round_trip("```rust\nfn main() {}\n```\n");
    assert_round_trip("```mermaid\ngraph TD\n  A --> B\n```\n");
    assert_round_trip("````\nhas ``` inside\n````\n");
}

#[test]
fn math_block() {
    assert_round_trip("$$\nx^2 + y^2 = z^2\n$$\n");
    let document = parse("$$\nE = mc^2\n$$\n");
    assert_eq!(document.blocks, vec![Block::Math("E = mc^2".into())]);
}

#[test]
fn media() {
    let markdown = ":::image type=\"content\" source=\"https://contoso.com/a_b_c.png\" alt-text=\"A *diagram*.\":::\n";
    assert_round_trip(markdown);
    let document = parse(markdown);
    let Block::Media(media) = &document.blocks[0] else {
        panic!("{document:#?}")
    };
    assert_eq!(media.source(), "https://contoso.com/a_b_c.png");
    assert_eq!(media.alt_text(), "A *diagram*.");
}

#[test]
fn media_inside_code_fence_is_code() {
    assert_round_trip("```\n:::image type=\"content\" source=\"x\" alt-text=\"y\":::\n```\n");
}

#[test]
fn tables() {
    assert_round_trip(
        "| Name | Role |\n| --- | :---: |\n| **Ada** | Lead |\n| Linus | a \\| b |\n",
    );
}

#[test]
fn rule() {
    assert_round_trip("Above\n\n---\n\nBelow\n");
}

#[test]
fn front_matter() {
    let markdown = "---\ntitle: Onboarding\nowner: \"IT team\"\n---\n\n# Welcome\n";
    assert_round_trip(markdown);
    let document = parse(markdown);
    let Properties::Entries(entries) = &document.properties else {
        panic!()
    };
    assert_eq!(entries[1].key, "owner");
    assert_eq!(entries[1].value, "\"IT team\"");
}

#[test]
fn complex_front_matter_is_kept_raw() {
    let markdown = "---\ntags:\n  - a\n  - b\n---\n\nText\n";
    assert_round_trip(markdown);
    assert!(matches!(parse(markdown).properties, Properties::Raw(_)));
}

#[test]
fn unsupported_content_is_kept_raw() {
    assert_round_trip("<div>\nhello\n</div>\n");
    assert_round_trip("- item\n\n  second paragraph in item\n");
    let document = parse("- item\n\n  second paragraph in item\n");
    assert!(matches!(document.blocks[0], Block::Raw(_)));
}

#[test]
fn escaping() {
    let document = Document {
        properties: Properties::None,
        blocks: vec![
            Block::Paragraph(Paragraph::text(
                ParagraphKind::Normal,
                "# not a heading, 5 * 3 = $15, snake_case, a_b, [x] ~tilde~ ==no== 1. two",
            )),
            Block::Paragraph(Paragraph::text(ParagraphKind::Normal, "1. not a list")),
            Block::Paragraph(Paragraph::text(ParagraphKind::Normal, "- not a bullet")),
        ],
    };
    assert_model_round_trip(&document);
}

#[test]
fn edge_whitespace_in_formatting() {
    let document = Document {
        properties: Properties::None,
        blocks: vec![Block::Paragraph(Paragraph {
            kind: ParagraphKind::Normal,
            list: None,
            inlines: vec![
                Inline::plain("Say"),
                Inline::styled(
                    " hello ",
                    Style {
                        bold: true,
                        ..Style::default()
                    },
                ),
                Inline::plain("world"),
            ],
        })],
    };
    assert_eq!(serialize(&document), "Say **hello** world\n");
}

#[test]
fn line_breaks() {
    let document = Document {
        properties: Properties::None,
        blocks: vec![Block::Paragraph(Paragraph {
            kind: ParagraphKind::Normal,
            list: Some(ListItem {
                kind: ListKind::Bullet,
                level: 0,
            }),
            inlines: vec![Inline::plain("line one\nline two")],
        })],
    };
    assert_eq!(serialize(&document), "- line one\\\n  line two\n");
    assert_model_round_trip(&document);
}

#[test]
fn sharepoint_reference_page() {
    // Every element from the SharePoint "create and edit a knowledge base manually" article.
    let markdown = "---
title: Reference
---

# Heading

## Formatting

**Important** *Emphasis* ~~Old wording~~ ==Review this==

- First item
- [ ] To do
- [x] Completed

1. First step

> Quoted text

Use `inline` code.

```
code block
```

[Microsoft Support](https://support.microsoft.com) and [[Page name]]

:::image type=\"content\" source=\"https://contoso.com/image.png\" alt-text=\"Description.\":::

:::image type=\"content\" source=\"https://contoso.com/video.mp4\" alt-text=\"Description.\":::

| Header | Header |
| --- | --- |
| Data | Data |

$x^2$

$$
x^2
$$

```mermaid
graph TD
  A --> B
```

---
";
    assert_round_trip(markdown);
}

#[test]
fn source_headings_for_the_markdown_view() {
    let source = "---\ntitle: Page\n---\n\n# Intro *here*\n\nText\n\n```\n# not a heading\n```\n\nSetext `code`\n------\n\n### Third\n";
    let found = super::headings(source);
    let summary: Vec<(u8, &str, usize)> = found
        .iter()
        .map(|heading| (heading.level, heading.text.as_str(), heading.line))
        .collect();
    assert_eq!(
        summary,
        [(1, "Intro here", 4), (2, "Setext code", 12), (3, "Third", 15)]
    );
}
