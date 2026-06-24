//! Shared string utility functions for case-insensitive ASCII operations.
//!
//! These are used by both the indexer and search modules to avoid code duplication.

/// Case-insensitive substring search (ASCII only).
///
/// Returns `true` if `haystack` contains `needle`, ignoring ASCII case.
pub fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return true;
    }
    if haystack.len() < needle.len() {
        return false;
    }
    haystack
        .as_bytes()
        .windows(needle.len())
        .any(|w| w.eq_ignore_ascii_case(needle.as_bytes()))
}

/// Case-insensitive prefix check (ASCII only).
///
/// Returns `true` if `s` starts with `prefix`, ignoring ASCII case.
pub fn starts_with_ignore_ascii_case(s: &str, prefix: &str) -> bool {
    if s.len() < prefix.len() {
        return false;
    }
    s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}

/// Case-insensitive reverse search for a substring (ASCII only).
///
/// Returns the byte offset of the last occurrence of `needle` in `haystack`,
/// ignoring ASCII case.
pub fn rfind_ignore_ascii_case(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(haystack.len());
    }
    if haystack.len() < needle.len() {
        return None;
    }
    haystack
        .as_bytes()
        .windows(needle.len())
        .enumerate()
        .rev()
        .find_map(|(i, w)| {
            if w.eq_ignore_ascii_case(needle.as_bytes()) {
                Some(i)
            } else {
                None
            }
        })
}
