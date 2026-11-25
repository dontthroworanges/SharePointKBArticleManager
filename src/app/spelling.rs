//! Spelling, through the Windows spell checker, for the Document Editor's Spelling pane.
//!
//! The page's red squiggles cannot be read back, so the page sends its prose as runs of text
//! at their positions, each paragraph goes to the checker (ISpellChecker, from the Windows
//! spell checking API), and each misspelling is placed back in the page from the positions
//! its text came with.

use crate::winui as native;
use windows_core::{HSTRING, Interface, PWSTR, Result};

/// From spellcheck.h: what the checker suggests doing about a word.
const GET_SUGGESTIONS: native::CORRECTIVE_ACTION = 1;
const REPLACE: native::CORRECTIVE_ACTION = 2;
/// From objbase.h: the spell checker factory runs in the app's process.
const CLSCTX_INPROC_SERVER: u32 = 1;
/// The most suggestions offered for one word, as in Word's Editor pane.
const MAX_SUGGESTIONS: usize = 3;

pub struct Checker(native::ISpellChecker);

/// A misspelled word in a piece of text, at UTF-16 offsets, with what could replace it.
#[derive(Clone, Debug, PartialEq)]
pub struct Misspelling {
    pub start: usize,
    pub word: String,
    pub suggestions: Vec<String>,
}

impl Checker {
    /// A checker for the user's language, or US English if Windows has none for it.
    pub fn new() -> Result<Self> {
        let factory: native::ISpellCheckerFactory = unsafe {
            let mut object = std::ptr::null_mut();
            native::CoCreateInstance(
                &native::SpellCheckerFactory,
                std::ptr::null_mut(),
                CLSCTX_INPROC_SERVER,
                &native::ISpellCheckerFactory::IID,
                &mut object,
            )
            .ok()?;
            native::ISpellCheckerFactory::from_raw(object)
        };
        let language = user_language();
        let tag = match &language {
            Some(tag)
                if unsafe { factory.IsSupported(&HSTRING::from(tag.as_str())) }
                    .is_ok_and(|supported| supported.as_bool()) =>
            {
                tag.as_str()
            }
            _ => "en-US",
        };
        Ok(Self(unsafe {
            factory.CreateSpellChecker(&HSTRING::from(tag))?
        }))
    }

    /// The misspellings in `text`. Repeated words ("the the") and grammar are left to the
    /// editor; only words to correct are listed.
    pub fn check(&self, text: &str) -> Result<Vec<Misspelling>> {
        let units: Vec<u16> = text.encode_utf16().collect();
        let errors = unsafe { self.0.ComprehensiveCheck(&HSTRING::from(text))? };
        let mut found = Vec::new();
        // Next fails, or returns nothing, once every error has been read.
        while let Ok(error) = unsafe { errors.Next() } {
            let (start, length, action) = unsafe {
                (
                    error.StartIndex()? as usize,
                    error.Length()? as usize,
                    error.CorrectiveAction()?,
                )
            };
            if action != GET_SUGGESTIONS && action != REPLACE {
                continue;
            }
            let Some(word) = units.get(start..start + length) else {
                continue;
            };
            let word = String::from_utf16_lossy(word);
            let mut suggestions = Vec::new();
            if action == REPLACE
                && let Ok(replacement) = unsafe { error.Replacement() }
            {
                suggestions.push(take_string(replacement));
            }
            for suggestion in self.suggest(&word)? {
                if suggestions.len() == MAX_SUGGESTIONS {
                    break;
                }
                if !suggestions.contains(&suggestion) {
                    suggestions.push(suggestion);
                }
            }
            found.push(Misspelling {
                start,
                word,
                suggestions,
            });
        }
        Ok(found)
    }

    fn suggest(&self, word: &str) -> Result<Vec<String>> {
        let list = unsafe { self.0.Suggest(&HSTRING::from(word))? };
        let mut suggestions = Vec::new();
        while suggestions.len() < MAX_SUGGESTIONS {
            let mut item = PWSTR::null();
            let mut fetched = 0;
            let result = unsafe { list.Next(1, &mut item, Some(&mut fetched)) };
            if result.is_err() || fetched == 0 || item.is_null() {
                break;
            }
            suggestions.push(take_string(item));
        }
        Ok(suggestions)
    }
}

/// Reads a string the checker allocated, and frees it.
fn take_string(text: PWSTR) -> String {
    let value = unsafe { text.to_string() }.unwrap_or_default();
    unsafe { native::CoTaskMemFree(text.0.cast()) };
    value
}

/// The user's Windows language, such as "en-GB".
fn user_language() -> Option<String> {
    let mut buffer = [0u16; 85];
    let length = unsafe { native::GetUserDefaultLocaleName(PWSTR(buffer.as_mut_ptr()), 85) };
    (length > 1).then(|| String::from_utf16_lossy(&buffer[..length as usize - 1]))
}

/// A misspelling in the page: where it is, and what could replace it.
#[derive(Clone, Debug, PartialEq)]
pub struct Issue {
    /// Page positions of the word.
    pub start: i32,
    pub end: i32,
    pub word: String,
    pub suggestions: Vec<String>,
    /// The text around it, for the pane.
    pub before: String,
    pub after: String,
}

/// Every misspelling in the page's prose, in order. `blocks` is what the page sends: for each
/// paragraph, its runs of text as `{pos, text}` (see `textBlocks` in `web/editor.js`).
pub fn check_blocks(checker: &Checker, blocks: &serde_json::Value) -> Result<Vec<Issue>> {
    let mut issues = Vec::new();
    for block in blocks.as_array().into_iter().flatten() {
        let runs: Vec<(i32, Vec<u16>)> = block
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|run| {
                Some((
                    run["pos"].as_i64()? as i32,
                    run["text"].as_str()?.encode_utf16().collect(),
                ))
            })
            .collect();
        let paragraph = Paragraph::new(&runs);
        let text = String::from_utf16_lossy(&paragraph.units);
        // The checker rejects empty text.
        if text.trim().is_empty() {
            continue;
        }
        for misspelling in checker.check(&text)? {
            let length = misspelling.word.encode_utf16().count();
            let (Some(start), Some(last)) = (
                paragraph.position(misspelling.start),
                paragraph.position(misspelling.start + length.saturating_sub(1)),
            ) else {
                continue;
            };
            let (before, after) = paragraph.around(misspelling.start, misspelling.start + length);
            issues.push(Issue {
                start,
                end: last + 1,
                word: misspelling.word,
                suggestions: misspelling.suggestions,
                before,
                after,
            });
        }
    }
    Ok(issues)
}

/// A paragraph's text, joined from its runs, and where each of its characters is in the page.
struct Paragraph {
    units: Vec<u16>,
    /// The page position of each unit; None for the space put between runs that are not next
    /// to each other (around a line break or an equation), so words do not run together.
    positions: Vec<Option<i32>>,
}

impl Paragraph {
    fn new(runs: &[(i32, Vec<u16>)]) -> Self {
        let mut units = Vec::new();
        let mut positions = Vec::new();
        let mut next = None;
        for (pos, text) in runs {
            if next.is_some_and(|next| next != *pos) {
                units.push(u16::from(b' '));
                positions.push(None);
            }
            for (index, unit) in text.iter().enumerate() {
                units.push(*unit);
                positions.push(Some(pos + index as i32));
            }
            next = Some(pos + text.len() as i32);
        }
        Self { units, positions }
    }

    fn position(&self, offset: usize) -> Option<i32> {
        self.positions.get(offset).copied().flatten()
    }

    /// Up to 30 characters before and 40 after a range, for the pane.
    fn around(&self, start: usize, end: usize) -> (String, String) {
        let before = String::from_utf16_lossy(&self.units[..start.min(self.units.len())]);
        let after = String::from_utf16_lossy(&self.units[end.min(self.units.len())..]);
        let before: String = before.chars().rev().take(30).collect::<Vec<_>>().into_iter().rev().collect();
        let after: String = after.chars().take(40).collect();
        (before.trim_start().to_string(), after.trim_end().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_keep_their_positions_and_gaps_become_spaces() {
        let runs = vec![
            (1, "Hello ".encode_utf16().collect()),
            (7, "wor".encode_utf16().collect()),
            (10, "ld".encode_utf16().collect()),
            // After a line break (one position).
            (13, "again".encode_utf16().collect()),
        ];
        let paragraph = Paragraph::new(&runs);
        assert_eq!(String::from_utf16_lossy(&paragraph.units), "Hello world again");
        assert_eq!(paragraph.position(6), Some(7));
        assert_eq!(paragraph.position(10), Some(11));
        assert_eq!(paragraph.position(11), None);
        assert_eq!(paragraph.position(12), Some(13));
        assert_eq!(paragraph.around(6, 11), ("Hello ".to_string(), " again".to_string()));
    }
}
