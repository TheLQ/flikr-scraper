#![feature(error_generic_member_access)]
#![feature(duration_constructors)]

use crate::downloader::{DownType, Downloader};
use crate::err::{SResult, pretty_panic};
use crate::flikr_extractor::extract_photostream_image_ids;
use crate::flikr_js_extractor::read_js_extractor;
use crate::flikr_url::flikr_photostream_pages_as_ids;
use std::env;
use std::process::ExitCode;
use tracing::info;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

mod downloader;
mod err;
mod flikr_extractor;
mod flikr_js_extractor;
mod flikr_url;

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

    let image_paths = spider_image_paths_js()?;
    info!("loaded {} images", image_paths.len());

    Ok(())
}

fn spider_image_paths(downloader: &mut Downloader) -> SResult<()> {
    let scrape_users = [flikr_photostream_pages_as_ids("98762402@N06", 5)];
    for scrape_user in scrape_users {
        for page in scrape_user {
            let content = downloader.fetch(DownType::Photostream, page)?;
            info!("loaded content of {}", content.len());

            extract_photostream_image_ids(content)?;
            todo!()
        }
    }
    Ok(())
}

fn spider_image_paths_js() -> SResult<Vec<String>> {
    read_js_extractor("98762402@N06")
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
