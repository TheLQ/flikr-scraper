use std::ops::Range;

pub fn flikr_photostream_pages_as_ids(user_id: &str, total_pages: usize) -> Vec<String> {
    (1..=total_pages)
        .into_iter()
        .map(|i| format!("{user_id}/page{i}"))
        .collect()
}

pub fn extract_image_id_from_livestatic(url: &str) -> &str {
    let url_bytes = url.as_bytes();

    let start = index_of_at_count(url_bytes, b'/', 4) + 1;
    let end = index_of_at_count(url_bytes, b'_', 1);
    &url[start..end]
}

fn index_of_at_count<T: PartialEq>(slice: &[T], needle: T, count: usize) -> usize {
    let mut final_needle = 0;

    let mut i = 0;
    while final_needle != count {
        if slice[i] == needle {
            final_needle += 1;
        }
        i += 1;
    }

    assert_eq!(final_needle, count);

    i - 1
}

#[cfg(test)]
mod test {
    use crate::flikr_url::extract_image_id_from_livestatic;

    #[test]
    fn text_extract() {
        let url = "https://live.staticflickr.com/1646/23800577019_ce3007b2e6.jpg";
        assert_eq!(extract_image_id_from_livestatic(url), "23800577019")
    }
}
