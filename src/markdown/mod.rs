//! The Markdown dialect of SharePoint Knowledge Base pages.

mod model;
mod parse;
mod serialize;
#[cfg(test)]
mod tests;

pub use model::*;
pub use parse::{headings, merge_inlines as merge, parse};
pub use serialize::serialize;

