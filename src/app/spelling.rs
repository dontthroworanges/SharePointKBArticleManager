//! Spelling, through the Windows spell checker: the same checker, dictionary and language as
//! the red squiggles RichEditBox draws, used here to list every misspelling in the
//! Document Editor's Spelling pane.
//!
//! The editor cannot report its squiggles, so each paragraph's text goes to the checker
//! (ISpellChecker, from the Windows spell checking API) and each misspelling is then found in
//! the editor by its word, which gives its exact position even where links add hidden text.

use crate::rich;
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

/// A misspelling in the page: which editor, where, and what could replace it.
#[derive(Clone, Debug, PartialEq)]
pub struct Issue {
    pub editor: super::Id,
    pub start: i32,
    pub end: i32,
    pub word: String,
    pub suggestions: Vec<String>,
    /// The text around it, for the pane: (before, word, after).
    pub before: String,
    pub after: String,
}

/// Every misspelling in one editor, in order, skipping inline code and math.
pub fn check_editor(
    checker: &Checker,
    editor: super::Id,
    document: &native::RichEditTextDocument,
) -> Result<Vec<Issue>> {
    let mut issues = Vec::new();
    for (start, end, text) in rich::paragraph_texts(document)? {
        // The checker rejects empty text.
        if text.trim().is_empty() {
            continue;
        }
        for misspelling in checker.check(&text)? {
            // The word is at least this far in: hidden link text only pushes it further.
            let from = start + misspelling.start as i32;
            let Some((word_start, word_end)) =
                rich::find_word(document, from, end, &misspelling.word)?
            else {
                continue;
            };
            if rich::is_code_or_math(document, word_start)? {
                continue;
            }
            let (before, _, after) =
                rich::snippet(document, word_start, word_end).unwrap_or_default();
            issues.push(Issue {
                editor,
                start: word_start,
                end: word_end,
                word: misspelling.word,
                suggestions: misspelling.suggestions,
                before,
                after,
            });
        }
    }
    Ok(issues)
}
