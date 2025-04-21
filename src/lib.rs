#![feature(error_generic_member_access)]

use crate::downloader::{DownType, Downloader};
use crate::err::{SResult, pretty_panic};
use std::env;
use std::process::ExitCode;
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

mod downloader;
mod err;

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
    downloader.fetch(DownType::Photostream, "asdasd".into())?;

    Ok(())
}

fn init_logging() {
    let default_env = "trace,\
    reqwest::blocking::wait=DEBUG,\
    reqwest::blocking::client=DEBUG,\
    hyper_util::client::legacy::pool=DEBUG";

    let subscriber = Registry::default();

    let env_var = env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_| default_env.into());
    let env_layer = EnvFilter::builder().parse(env_var).expect("bad env");
    let subscriber = subscriber.with(env_layer);

    let filter_layer = Layer::default().compact();
    let subscriber = subscriber.with(filter_layer);

    subscriber.init()
}
