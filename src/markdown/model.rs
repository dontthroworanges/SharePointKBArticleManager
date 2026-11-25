//! Document model shared by the visual editor, the source view, and the Markdown parser.
//!
//! The model only holds what SharePoint Knowledge Base pages can express. Anything else in a
//! file is kept verbatim as [`Block::Raw`] so opening and saving never loses content.

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Document {
    pub properties: Properties,
    pub blocks: Vec<Block>,
}

/// YAML front matter. Simple `key: value` lines are editable as properties; anything more
/// complex (lists, nesting, comments) is kept as raw YAML.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Properties {
    #[default]
    None,
    Entries(Vec<Property>),
    Raw(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Property {
    pub key: String,
    /// The YAML scalar exactly as written, including any quotes.
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Paragraph(Paragraph),
    /// A fenced code block. Mermaid diagrams are code blocks with the `mermaid` language.
    Code {
        language: String,
        code: String,
    },
    /// A display equation written as `$$ … $$`.
    Math(String),
    Table(Table),
    /// A `:::image …:::` image or video.
    Media(Media),
    Rule,
    /// Markdown the visual editor does not model, kept exactly as written.
    Raw(String),
}

/// A paragraph of the text flow: normal text, a heading, a quote, or a list item.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Paragraph {
    pub kind: ParagraphKind,
    pub list: Option<ListItem>,
    pub inlines: Vec<Inline>,
}

impl Paragraph {
    #[cfg(test)]
    pub fn text(kind: ParagraphKind, text: &str) -> Self {
        Self {
            kind,
            list: None,
            inlines: vec![Inline::plain(text)],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inlines.iter().all(|inline| inline.text.is_empty())
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ParagraphKind {
    #[default]
    Normal,
    /// Heading levels 1 through 6.
    Heading(u8),
    Quote,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListItem {
    pub kind: ListKind,
    /// Nesting depth, starting at 0.
    pub level: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListKind {
    Bullet,
    Numbered,
    Task { checked: bool },
}

/// A run of text that shares one set of formatting.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Inline {
    /// Display text. `\n` is a line break inside the paragraph. For inline math this is the
    /// LaTeX source; for raw inlines it is the Markdown to emit verbatim.
    pub text: String,
    pub style: Style,
    pub link: Option<Link>,
}

impl Inline {
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    pub fn styled(text: impl Into<String>, style: Style) -> Self {
        Self {
            text: text.into(),
            style,
            link: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Style {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub highlight: bool,
    pub code: bool,
    pub math: bool,
    /// Markdown the editor does not model (for example an inline image or HTML), emitted as is.
    pub raw: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Link {
    Url(String),
    /// A `[[Page name]]` link to another Knowledge Base page.
    Page(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    pub alignments: Vec<Alignment>,
    pub header: Vec<Vec<Inline>>,
    pub rows: Vec<Vec<Vec<Inline>>>,
}

impl Table {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self {
            alignments: vec![Alignment::None; columns],
            header: (1..=columns)
                .map(|column| vec![Inline::plain(format!("Column {column}"))])
                .collect(),
            rows: vec![vec![Vec::new(); columns]; rows],
        }
    }

    pub fn columns(&self) -> usize {
        self.header.len()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Alignment {
    #[default]
    None,
    Left,
    Center,
    Right,
}

/// The attributes of a `:::image …:::` directive, in their original order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Media {
    pub attributes: Vec<(String, String)>,
}

impl Media {
    pub fn new(source: &str, alt_text: &str) -> Self {
        Self {
            attributes: vec![
                ("type".into(), "content".into()),
                ("source".into(), source.into()),
                ("alt-text".into(), alt_text.into()),
            ],
        }
    }

    pub fn get(&self, key: &str) -> &str {
        self.attributes
            .iter()
            .find(|(name, _)| name == key)
            .map_or("", |(_, value)| value.as_str())
    }

    pub fn set(&mut self, key: &str, value: &str) {
        match self.attributes.iter_mut().find(|(name, _)| name == key) {
            Some((_, existing)) => *existing = value.to_string(),
            None => self.attributes.push((key.into(), value.into())),
        }
    }

    pub fn source(&self) -> &str {
        self.get("source")
    }

    pub fn alt_text(&self) -> &str {
        self.get("alt-text")
    }

}
