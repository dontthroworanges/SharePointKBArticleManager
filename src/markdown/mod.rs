//! The Markdown dialect of SharePoint Knowledge Base pages.

mod model;
mod parse;
mod serialize;
#[cfg(test)]
mod tests;

pub use model::*;
pub use parse::{merge_inlines as merge, parse};
pub use serialize::{inlines_to_markdown, serialize};

/// Parses inline Markdown such as the contents of a table cell.
pub fn parse_inlines(markdown: &str) -> Vec<Inline> {
    let document = parse(markdown);
    match document.blocks.first() {
        Some(Block::Paragraph(paragraph)) if document.blocks.len() == 1 => {
            paragraph.inlines.clone()
        }
        _ if markdown.trim().is_empty() => Vec::new(),
        _ => vec![Inline::styled(
            markdown.trim(),
            Style {
                raw: true,
                ..Style::default()
            },
        )],
    }
}
