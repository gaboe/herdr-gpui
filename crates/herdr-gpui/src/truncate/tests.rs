use super::*;

/// `e` and a combining acute accent; a family emoji joined by zero-width
/// joiners; a flag from two regional indicators; and a Hangul syllable
/// spelled as its three conjoining jamo.
const CLUSTERS: [&str; 4] = [
    "e\u{301}",
    "\u{1f468}\u{200d}\u{1f469}\u{200d}\u{1f467}",
    "\u{1f1ef}\u{1f1f5}",
    "\u{1112}\u{1161}\u{11ab}",
];

#[test]
fn cuts_keep_every_cluster_whole() {
    let text = CLUSTERS.concat();
    for limit in 0..CLUSTERS.len() {
        let kept = cut(&text, limit).unwrap_or_default();
        assert_eq!(kept, CLUSTERS[..limit].concat(), "limit {limit}");
        assert_eq!(
            ellipsize(&text, limit),
            format!("{}{ELLIPSIS}", CLUSTERS[..limit].concat())
        );
    }
    assert_eq!(cut(&text, CLUSTERS.len()), None);
    assert!(matches!(
        ellipsize(&text, CLUSTERS.len()),
        Cow::Borrowed(same) if same == text
    ));
}

#[test]
fn cjk_text_counts_one_cluster_per_character() {
    assert_eq!(ellipsize("你好世界", 2), "你好\u{2026}");
    assert_eq!(ellipsize("こんにちは", 5), "こんにちは");
}

#[test]
fn width_cuts_count_wide_characters_twice() {
    assert_eq!(cut_width("abcdef", 4), Some("abcd"));
    assert_eq!(cut_width("你好世界", 5), Some("你好"));
    assert_eq!(cut_width("你好世界", 8), None);
    // A combining accent adds no width and stays with its letter.
    assert_eq!(
        cut_width("e\u{301}e\u{301}e\u{301}", 2),
        Some("e\u{301}e\u{301}")
    );
    // A wide character that would overrun is left out whole.
    assert_eq!(cut_width("ab你", 3), Some("ab"));
}
