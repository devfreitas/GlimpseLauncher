use crate::core::indexer::AppEntry;
use crate::utils::{contains_ignore_ascii_case, starts_with_ignore_ascii_case};
use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

pub fn search_apps(query: &str, index: &[AppEntry]) -> Vec<AppEntry> {
    if query.is_empty() {
        return Vec::new();
    }

    thread_local! {
        static MATCHER: std::cell::RefCell<Matcher> = std::cell::RefCell::new(Matcher::new(Config::DEFAULT.match_paths()));
    }

    let pattern = Pattern::parse(query, CaseMatching::Ignore, Normalization::Smart);

    let matches = MATCHER.with(|m| {
        let mut matcher = m.borrow_mut();
        pattern.match_list(index, &mut *matcher)
    });

    let mut results: Vec<(i64, &AppEntry)> = matches
        .into_iter()
        .map(|(app, score)| {
            let mut final_score = (score as i64) * (app.priority as i64);

            if starts_with_ignore_ascii_case(&app.name, query) {
                final_score += 10000;
            }

            let docs = ["documentation", "help", "readme", "manual"];
            if docs
                .iter()
                .any(|term| contains_ignore_ascii_case(&app.name, term))
            {
                final_score /= 10;
            }

            (final_score, app)
        })
        .collect();

    results.sort_unstable_by_key(|b| std::cmp::Reverse(b.0));

    results
        .into_iter()
        .take(4)
        .map(|(_, app)| app.clone())
        .collect()
}
