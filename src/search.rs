use crate::indexer::{AppEntry, BLACKLIST};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use rayon::prelude::*;

pub fn search_apps(query: &str, index: &[AppEntry]) -> Vec<AppEntry> {
    // If the query is empty, show no results initially.
    if query.is_empty() {
        return Vec::new();
    }

    let matcher = SkimMatcherV2::default();
    let query_lower = query.to_lowercase();

    let mut results: Vec<(i64, AppEntry)> = index
        .par_iter()
        .filter(|app| {
            let name_lc = app.name.to_lowercase();
            // Exclude entries whose name contains any black‑list term (case‑insensitive).
            !BLACKLIST.iter().any(|b| name_lc.contains(&b.to_lowercase()))
        })
        .filter_map(|app| {
            let name_lower = app.name.to_lowercase();

            // SCORE BASE FUZZY
            matcher.fuzzy_match(&app.name, query).map(|score| {
                let mut final_score = score * (app.priority as i64);

                // BÔNUS DE INÍCIO DE PALAVRA
                if name_lower.starts_with(&query_lower) {
                    final_score += 10000;
                }

                // PENALIDADE PARA DOCUMENTAÇÃO
                let docs = ["documentation", "help", "readme", "manual"];
                if docs.iter().any(|term| name_lower.contains(term)) {
                    final_score /= 10;
                }

                (final_score, app.clone())
            })
        })
        .collect();

    results.sort_by(|a, b| b.0.cmp(&a.0));

    results.into_iter().map(|(_, app)| app).take(10).collect()
}
