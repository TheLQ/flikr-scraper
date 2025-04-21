use crate::err::{SError, SResult};
use std::fs::{read, write};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};
use strum::{AsRefStr, VariantArray};
use tracing::debug;

pub struct Downloader {
    // cache: HashMap<DownType, Vec<PathBuf>>,
    client: reqwest::blocking::Client,
    last_request: Instant,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, VariantArray, AsRefStr)]
pub enum DownType {
    Photostream,
    ImageSizes,
}

pub const IMAGE_DB_ROOT: &str = "image-db";
const REQUEST_THROTTLE: Duration = Duration::from_secs(5);

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

        Self {
            client: reqwest::blocking::Client::new(),
            // arbitrary old date
            last_request: Instant::now() - Duration::from_days(1),
        }
    }

    pub fn fetch(&mut self, downtype: DownType, url_id: String) -> SResult<Vec<u8>> {
        let url = match downtype {
            DownType::Photostream => format!("https://www.flickr.com/photos/{url_id}"),
            DownType::ImageSizes => format!("https://www.flickr.com/photos/{url_id}/sizes/o/"),
        };

        let safe_name = url_id
            .replace("/", "_")
            .replace("\\", "_")
            .to_ascii_lowercase();
        let cache_path = path([
            IMAGE_DB_ROOT,
            &downtype.as_ref().to_ascii_lowercase(),
            &safe_name,
        ]);
        if cache_path.exists() {
            debug!("cached id {url_id} url {url} at {}", cache_path.display());
            Ok(read(&cache_path).map_err(SError::io(&cache_path))?)
        } else {
            debug!("writing id {url_id} url {url} to {}", cache_path.display());

            let throttle_safe: Instant = self.last_request + REQUEST_THROTTLE;
            let throttle_cur = Instant::now();
            let sleep_dur = throttle_safe - throttle_cur;
            if sleep_dur.as_secs() > 0 {
                debug!("Throttle for {} secs", sleep_dur.as_secs());
                thread::sleep(sleep_dur);
            }

            let body = self.client.get(url).send()?.bytes()?;
            write(&cache_path, &body).map_err(SError::io(cache_path))?;

            self.last_request = Instant::now();
            Ok(body.to_vec())
            // Ok("".into())
        }
    }
}

pub fn path<const N: usize>(input: [&str; N]) -> PathBuf {
    input.into_iter().collect()
}
