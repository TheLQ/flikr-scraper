use crate::err::{SError, SResult};
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, write};
use std::path::PathBuf;
use strum::{AsRefStr, VariantArray};
use tracing::debug;

pub struct Downloader {
    // cache: HashMap<DownType, Vec<PathBuf>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, VariantArray, AsRefStr)]
pub enum DownType {
    Photostream,
}

const ROOT: &str = "image-db";

impl Downloader {
    pub fn init() -> Self {
        // let mut cache = HashMap::new();
        //
        // for downtype in DownType::VARIANTS {
        //     let dir_name = downtype.as_ref().to_lowercase();
        //
        //     let mut paths = Vec::new();
        //     for dir_entry in read_dir(path([ROOT, &dir_name])).unwrap() {
        //         let dir_entry = dir_entry.unwrap();
        //         paths.push(dir_entry.path());
        //     }
        //
        //     cache.insert(*downtype, paths);
        // }
        //
        // Self { cache }
        Self {}
    }

    pub fn fetch(&mut self, downtype: DownType, url_id: String) -> SResult<String> {
        let url = match downtype {
            // DownType::Photostream => format!("https://www.flickr.com/photos/{url_id}")
            DownType::Photostream => format!("https://www.rust-lang.org/"),
        };

        let safe_name = url_id.replace("/", "_").replace("\\", "_");
        let cache_path = path([ROOT, downtype.as_ref(), &safe_name]);
        if cache_path.exists() {
            debug!("cached id {url_id} url {url} at {}", cache_path.display());
            Ok(read_to_string(&cache_path).map_err(SError::io(&cache_path))?)
        } else {
            debug!("writing id {url_id} url {url} to {}", cache_path.display());

            let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;
            write(&cache_path, &body).map_err(SError::io(cache_path))?;
            Ok(body)
        }
    }
}

fn path<const N: usize>(input: [&str; N]) -> PathBuf {
    input.into_iter().collect()
}
