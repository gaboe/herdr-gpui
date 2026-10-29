//! Cutting text the app shows to a length. Cuts fall between grapheme
//! clusters, so one never leaves half of what a reader sees as a single
//! character: a letter without its accent, part of an emoji sequence, or a
//! Hangul syllable without its final consonant.

use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub(crate) const ELLIPSIS: char = '\u{2026}';

/// The first `limit` grapheme clusters of `text`, or `None` when it has no
/// more than that.
pub(crate) fn cut(text: &str, limit: usize) -> Option<&str> {
    text.grapheme_indices(true)
        .nth(limit)
        .map(|(end, _)| &text[..end])
}

/// The start of `text` that fits in `columns` terminal columns, or `None`
/// when all of it fits. A CJK character takes two columns, so a budget set
/// in Latin letters also holds about as wide a CJK label, not twice as wide.
pub(crate) fn cut_width(text: &str, columns: usize) -> Option<&str> {
    let mut used = 0;
    for (start, grapheme) in text.grapheme_indices(true) {
        used += grapheme.width();
        if used > columns {
            return Some(&text[..start]);
        }
    }
    None
}

/// `text` cut to `limit` grapheme clusters, followed by an ellipsis when
/// anything was cut.
pub(crate) fn ellipsize(text: &str, limit: usize) -> Cow<'_, str> {
    match cut(text, limit) {
        Some(kept) => Cow::Owned(format!("{kept}{ELLIPSIS}")),
        None => Cow::Borrowed(text),
    }
}

#[cfg(test)]
mod tests;
