use crate::core::indexer::AppEntry;
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

fn starts_with_ignore_ascii_case(s: &str, prefix: &str) -> bool {
    if s.len() < prefix.len() { return false; }
    s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}

fn contains_ignore_ascii_case(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() { return true; }
    if haystack.len() < needle.len() { return false; }
    haystack.as_bytes().windows(needle.len()).any(|w| w.eq_ignore_ascii_case(needle.as_bytes()))
}

pub fn search_apps(query: &str, index: &[AppEntry]) -> Vec<AppEntry> {
    if query.is_empty() {
        return Vec::new();
    }

    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);

    // Index already filters out blacklisted apps during build_index,
    // so we can directly match on the index to save huge amounts of CPU.
    let matches = pattern.match_list(index, &mut matcher);

    let mut results: Vec<(i64, &AppEntry)> = matches
        .into_iter()
        .map(|(app, score)| {
            let mut final_score = (score as i64) * (app.priority as i64);

            if starts_with_ignore_ascii_case(&app.name, query) {
                final_score += 10000;
            }

            let docs = ["documentation", "help", "readme", "manual"];
            if docs.iter().any(|term| contains_ignore_ascii_case(&app.name, term)) {
                final_score /= 10;
            }

            (final_score, app)
        })
        .collect();

    results.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    results
        .into_iter()
        .take(10)
        .map(|(_, app)| app.clone())
        .collect()
}

