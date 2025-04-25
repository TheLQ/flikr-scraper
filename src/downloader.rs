use crate::err::{SError, SResult};
use crate::utils::last_position_of;
use reqwest::{Proxy, StatusCode};
use std::fs::{create_dir, read, write};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};
use strum::{AsRefStr, VariantArray};
use tracing::{debug, info, warn};

pub struct Downloader {
    // cache: HashMap<DownType, Vec<PathBuf>>,
    client: reqwest::blocking::Client,
    last_request: Instant,
}

#[derive(Clone, PartialEq, Eq, Hash, VariantArray, AsRefStr)]
pub enum DownType {
    Photostream,
    ImageViewer,
    ImageSizes,
    ImageOrig,
}

pub const IMAGE_DB_ROOT: &str = "image-db";
const REQUEST_THROTTLE: Duration = Duration::from_secs(5); // Please be a nice scraper

impl Downloader {
    pub fn init() -> Self {
        let proxy_addr = std::env::var("WARC_PROXY")
            .expect("Please be nice, export WARC_PROXY=127.0.0.1:8000 pointing to warcprox");

        Self {
            client: reqwest::blocking::Client::builder()
                // proxy to MITM warcprox
                .proxy(Proxy::all(format!("http://{proxy_addr}")).unwrap())
                // which uses self-signed CA
                .danger_accept_invalid_certs(true)
                // increase timeout. I think the proxy buffers the whole response first
                .timeout(Duration::from_mins(3))
                .build()
                .unwrap(),
            // arbitrary old date
            last_request: Instant::now() - Duration::from_days(1),
        }
    }

    pub fn fetch(&mut self, downtype: DownType, for_user: &str, extra: &str) -> SResult<Vec<u8>> {
        let safe_name: String;
        let url = match downtype {
            DownType::Photostream => {
                safe_name = format!("{for_user}_page{extra}");
                format!("https://www.flickr.com/photos/{for_user}/page{extra}")
            }
            DownType::ImageViewer => {
                safe_name = format!("{for_user}_{extra}");
                format!("https://www.flickr.com/photos/{for_user}/{extra}/")
            }
            DownType::ImageSizes => {
                safe_name = format!("{for_user}_{extra}");
                format!("https://www.flickr.com/photos/{for_user}/{extra}/sizes/o/")
            }
            DownType::ImageOrig => {
                let filename_start = last_position_of(extra, b'/');
                safe_name = format!("{for_user}_{}", &extra[(filename_start + 1)..]);
                extra.to_string()
            }
        };

        let cache_path = path([IMAGE_DB_ROOT, &downtype.safe_name(), &safe_name]);
        if cache_path.exists() {
            debug!("cached url {url} at {}", cache_path.display());
            Ok(read(&cache_path).map_err(SError::io(&cache_path))?)
        } else {
            debug!("writing url {url} to {}", cache_path.display());

            let mut body = None;
            for i in 0..2 {
                if i != 0 {
                    warn!("retry {i}");
                }
                let throttle_safe: Instant = self.last_request + REQUEST_THROTTLE;
                let throttle_cur = Instant::now();
                let sleep_dur = throttle_safe - throttle_cur;
                if sleep_dur.as_secs() > 0 {
                    debug!("Throttle for {} secs", sleep_dur.as_secs());
                    thread::sleep(sleep_dur);
                }

                let response = self.client.get(&url).send()?;
                if response.status() != StatusCode::OK {
                    panic!("bad response {}", response.status());
                }
                body = Some(response.bytes()?);
                break;
            }
            let Some(body) = body else {
                panic!("failed to download {url}")
            };
            write(&cache_path, &body).map_err(SError::io(cache_path))?;

            self.last_request = Instant::now();
            Ok(body.to_vec())
            // Ok("".into())
        }
    }
}

impl DownType {
    pub fn mkdirs() {
        let mut output_dirs: Vec<PathBuf> = Self::VARIANTS
            .iter()
            .map(|downtype| path([IMAGE_DB_ROOT, &downtype.safe_name()]))
            .collect();
        output_dirs.insert(0, path([IMAGE_DB_ROOT]));

        for dir in output_dirs {
            if !dir.exists() {
                info!("Creating directory {}", dir.display());
                create_dir(&dir).unwrap();
            }
        }
    }

    fn safe_name(&self) -> String {
        self.as_ref().to_ascii_lowercase()
    }
}

pub fn path<const N: usize>(input: [&str; N]) -> PathBuf {
    input.into_iter().collect()
}
