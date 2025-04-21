use crate::downloader::{IMAGE_DB_ROOT, path};
use crate::err::{SError, SResult};
use std::fs::{read, read_dir, read_to_string};

/// lol, if you wget the page it shows 25 images.
/// But in the browser there's 100 images per page once you scroll down
///
/// ```javascript
///(() => {
/// let imgs = [];
/// document.querySelectorAll(".photo-list-photo-container > img").forEach((e) => {
/// 	console.log("img " + e.src);
/// 	imgs.push(e.src)
/// });
/// console.log("v", imgs)
/// })()
/// ```
pub fn read_js_extractor(for_user: &str) -> SResult<Vec<String>> {
    let for_user = for_user.to_ascii_lowercase();
    let mut extracted_imgs = Vec::new();

    let js_root = path([IMAGE_DB_ROOT, "photostream-js"]);
    for dir_entry in read_dir(js_root).unwrap() {
        let dir_entry = dir_entry.unwrap();

        let path = dir_entry.path();
        if !path.as_os_str().to_string_lossy().contains(&for_user) {
            continue;
        }

        let mut raw_json = read(dir_entry.path()).map_err(SError::io(dir_entry.path()))?;
        let imgs: Vec<String> = unsafe { simd_json::from_slice(&mut raw_json).unwrap() };
        extracted_imgs.extend(imgs);
    }

    assert!(!extracted_imgs.is_empty(), "extracted 0 images");

    let len_before = extracted_imgs.len();
    extracted_imgs.sort();
    extracted_imgs.dedup();
    if extracted_imgs.len() != len_before {
        panic!("non-unique images?")
    }

    Ok(extracted_imgs)
}
