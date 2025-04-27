use crate::downloader::{IMAGE_DB_ROOT, path};
use crate::err::{SError, SResult};
use crate::viewer_gen::BookPhoto;
use scraper::{ElementRef, Html, Selector};
use std::fs::read;
use tracing::error;

/// JSON output of JS extractor
/// Direct HTML extraction isn't possible
pub fn read_js_extractor(for_user: &str) -> SResult<Vec<String>> {
    let json_path = path([IMAGE_DB_ROOT, "photostream-js", &format!("{for_user}.json")]);
    let mut raw_json = read(&json_path).map_err(SError::io(json_path))?;
    let mut extracted_imgs: Vec<String> = simd_json::from_slice(&mut raw_json).unwrap();
    assert!(!extracted_imgs.is_empty(), "extracted 0 images");

    let len_before = extracted_imgs.len();
    extracted_imgs.sort();
    extracted_imgs.dedup();
    if extracted_imgs.len() != len_before {
        panic!("non-unique images?")
    }

    Ok(extracted_imgs)
}

pub fn extract_original_size_url(content: Vec<u8>) -> SResult<String> {
    let document = Html::parse_document(str::from_utf8(&content).unwrap());

    let selector = Selector::parse("#all-sizes-header > dl > dd > a").unwrap();
    let mut founds: Vec<ElementRef> = document.select(&selector).collect();
    if founds.len() != 2 && founds.len() != 3 {
        panic!("uhhh {}", founds.len());
        // return Err(SError::bad_age("...".into()));
    }
    let found = founds.remove(founds.len() - 1);

    Ok(found.attr("href").unwrap().to_string())
}

pub fn extract_image_meta(content: Vec<u8>, book_photo: &mut BookPhoto) -> SResult<()> {
    let document = Html::parse_document(str::from_utf8(&content).unwrap());

    book_photo.title = {
        let selector = Selector::parse(".scrollable-container .photo-title").unwrap();
        let mut founds: Vec<ElementRef> = document.select(&selector).collect();
        if founds.len() == 1 {
            founds.remove(0).inner_html().trim().into()
        } else {
            "no-title".into()
        }
    };

    book_photo.description = {
        let selector = Selector::parse(".photo-desc").unwrap();
        let mut founds: Vec<ElementRef> = document.select(&selector).collect();
        if founds.len() == 1 {
            founds.remove(0).inner_html().trim().into()
        } else {
            "no-desc".into()
        }
    };

    Ok(())
}
