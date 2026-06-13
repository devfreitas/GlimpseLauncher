use crate::indexer::{AppEntry, BLACKLIST};
use nucleo_matcher::{Matcher, Config, pattern::{Pattern, CaseMatching, Normalization}};

struct MatchItem<'a> {
    app: &'a AppEntry,
}

impl<'a> AsRef<str> for MatchItem<'a> {
    fn as_ref(&self) -> &str {
        &self.app.name
    }
}

pub fn search_apps(query: &str, index: &[AppEntry]) -> Vec<AppEntry> {
    if query.is_empty() {
        return Vec::new();
    }

    let mut matcher = Matcher::new(Config::DEFAULT.match_paths());
    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);

    // Filter out blacklisted apps before matching
    let items: Vec<MatchItem> = index.iter()
        .filter(|app| {
            let name_lc = app.name.to_lowercase();
            !BLACKLIST.iter().any(|b| name_lc.contains(&b.to_lowercase()))
        })
        .map(|app| MatchItem { app })
        .collect();

    let matches = pattern.match_list(items, &mut matcher);

    let mut results: Vec<(i64, AppEntry)> = matches.into_iter()
        .map(|(item, score)| {
            let app = item.app;
            let name_lower = app.name.to_lowercase();
            let query_lower = query.to_lowercase();

            let mut final_score = (score as i64) * (app.priority as i64);

            if name_lower.starts_with(&query_lower) {
                final_score += 10000;
            }

            let docs = ["documentation", "help", "readme", "manual"];
            if docs.iter().any(|term| name_lower.contains(term)) {
                final_score /= 10;
            }

            (final_score, app.clone())
        })
        .collect();

    results.sort_by(|a, b| b.0.cmp(&a.0));

    results.into_iter().map(|(_, app)| app).take(10).collect()
}