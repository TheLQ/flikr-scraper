use std::ops::Range;

pub fn flikr_photostream_pages_as_ids(user_id: &str, total_pages: usize) -> Vec<String> {
    (1..=total_pages)
        .into_iter()
        .map(|i| format!("{user_id}/page{i}"))
        .collect()
}
