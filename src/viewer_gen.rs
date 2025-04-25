use crate::downloader::{IMAGE_DB_ROOT, path};
use crate::err::{SError, SResult};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::write;
use tracing::info;

#[derive(Serialize)]
pub struct BookRoot {
    photos: HashMap<String, Vec<BookPhoto>>,
}

#[derive(Serialize)]
pub struct BookPhoto {
    url: String,
    description: String,
}

impl BookRoot {
    pub fn new() -> Self {
        Self {
            photos: HashMap::new(),
        }
    }

    pub fn push_image(&mut self, for_user: &str, url: &str, description: String) {
        let for_user = for_user.to_string();
        let user_photos = self.photos.entry(for_user).or_default();
        user_photos.push(BookPhoto {
            url: url.to_string(),
            description,
        })
    }

    pub fn write(&self) -> SResult<()> {
        let json = simd_json::to_string(self).unwrap();

        let output_path = path([IMAGE_DB_ROOT, "scrape.json"]);
        write(&output_path, &json).map_err(SError::io(&output_path))?;
        info!("Wrote book state to {}", output_path.display());

        let output_path = path([IMAGE_DB_ROOT, "scrape-viewer.js"]);
        let js = format!("var book_viewer = {json}");
        write(&output_path, &js).map_err(SError::io(&output_path))?;
        info!("Wrote book state to {}", output_path.display());

        const RAW_VIEWER_HTML: &str = include_str!("html/viewer.html");
        const RAW_VIEWER_JS: &str = include_str!("html/viewer.js");

        let output_path = path([IMAGE_DB_ROOT, "viewer.html"]);
        write(&output_path, RAW_VIEWER_HTML).map_err(SError::io(&output_path))?;
        info!("wrote viewer client to {}", output_path.display());

        let output_path = path([IMAGE_DB_ROOT, "viewer.js"]);
        write(&output_path, RAW_VIEWER_JS).map_err(SError::io(&output_path))?;
        info!("wrote viewer client to {}", output_path.display());

        Ok(())
    }
}
