#![feature(error_generic_member_access)]
#![feature(duration_constructors)]

use crate::downloader::{DownType, Downloader};
use crate::err::{SError, SResult, pretty_panic};
use crate::flikr_extractor::extract_original_size_url;
use crate::flikr_url::extract_image_id_from_livestatic;
use flikr_extractor::read_js_extractor;
use std::env;
use std::process::ExitCode;
use tracing::{error, info};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

mod downloader;
mod err;
mod flikr_extractor;
mod flikr_url;
mod utils;

pub fn start_scraper() -> ExitCode {
    init_logging();
    if let Err(e) = _start_scraper() {
        pretty_panic(e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn _start_scraper() -> SResult<()> {
    let mut downloader = Downloader::init();

    for (user, max_pages) in [(USER_OLEG_KASHIRIN, 5)] {
        let image_paths = match 1 {
            1 => spider_image_paths(&mut downloader, user, max_pages)?,
            2 => spider_image_paths_js(user)?,
            _ => unimplemented!(),
        };
        info!("loaded {} images", image_paths.len());

        spider_image_sizes(&mut downloader, user, &image_paths)?;
    }

    Ok(())
}

const USER_OLEG_KASHIRIN: &str = "98762402@N06";

fn spider_image_paths(
    downloader: &mut Downloader,
    for_user: &str,
    max_pages: usize,
) -> SResult<Vec<String>> {
    for page in 1..=max_pages {
        let content = downloader.fetch(DownType::Photostream, for_user, &page.to_string())?;
        info!("loaded content of {}", content.len());

        todo!() // extract_photostream_image_ids(content)?;
    }
    Ok(Vec::new())
}

fn spider_image_paths_js(for_user: &str) -> SResult<Vec<String>> {
    read_js_extractor(for_user)
}

fn spider_image_sizes(
    downloader: &mut Downloader,
    for_user: &str,
    image_paths: &[String],
) -> SResult<()> {
    for image_path in image_paths {
        let image_id = extract_image_id_from_livestatic(&image_path);

        let image_page = downloader.fetch(DownType::ImageSizes, for_user, image_id)?;
        let original_image_url = match extract_original_size_url(image_page) {
            Err(SError::BadPage(page, _)) => {
                error!("no page found");
                continue;
            }
            r => r,
        }?;

        downloader.fetch(DownType::ImageOrig, for_user, &original_image_url)?;
    }

    Ok(())
}

fn init_logging() {
    let default_env = "trace,\
    reqwest::blocking::wait=DEBUG,\
    reqwest::blocking::client=DEBUG,\
    hyper_util::client::legacy::pool=DEBUG,\
    selectors::matching=INFO,\
    html5ever=INFO";

    let subscriber = Registry::default();

    let env_var = env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_| default_env.into());
    let env_layer = EnvFilter::builder().parse(env_var).expect("bad env");
    let subscriber = subscriber.with(env_layer);

    let filter_layer = Layer::default().compact();
    let subscriber = subscriber.with(filter_layer);

    subscriber.init()
}
