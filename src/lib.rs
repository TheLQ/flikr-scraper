#![feature(error_generic_member_access)]
#![feature(duration_constructors)]

use crate::downloader::{DownType, Downloader, IMAGE_DB_ROOT, path};
use crate::err::{SError, SResult, pretty_panic};
use crate::flikr_extractor::extract_original_size_url;
use crate::flikr_url::extract_image_id_from_livestatic;
use crate::viewer_gen::BookRoot;
use flikr_extractor::read_js_extractor;
use serde::de::IntoDeserializer;
use std::env;
use std::fs::read_dir;
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
mod viewer_gen;

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
    DownType::mkdirs();
    let mut downloader = Downloader::init();
    let mut book = BookRoot::new();

    // const USER_OLEG_KASHIRIN: &str = "98762402@N06";
    // const USER_MARTIJN_BOER: &str = "sic66";
    // const USER_FRITZ: &str = "130561288@N04";
    // let mut user_ids = detect_js_user_ids();
    // user_ids.retain(|v| ![USER_OLEG_KASHIRIN, USER_MARTIJN_BOER, USER_FRITZ].contains(&v.as_str()));
    // user_ids.insert(0, USER_OLEG_KASHIRIN.to_string());
    // user_ids.insert(0, USER_MARTIJN_BOER.to_string());
    // user_ids.push(USER_FRITZ.to_string());
    let user_ids = detect_js_user_ids();

    /*
    TODO: This is basic sequential processing. 5k files = 15k reads and 10k html parses.
    To scale needs a parallel processing queue and download queue.
    But meh, seems overkill for this one-shot scraper. I can wait
    */
    for user in user_ids {
        let max_pages = 0;
        let user = &user;
        let image_paths = match 2 {
            1 => spider_image_paths(&mut downloader, user, max_pages)?,
            2 => spider_image_paths_js(user)?,
            _ => unimplemented!(),
        };
        info!("loaded {} images", image_paths.len());

        // TODO: skip reparsing when imageorig already contains the image
        spider_image_sizes(&mut downloader, user, &image_paths, &mut book)?;
    }

    book.write()?;

    Ok(())
}

fn detect_js_user_ids() -> Vec<String> {
    let mut user_ids = Vec::new();
    for dir_entry in read_dir(path([IMAGE_DB_ROOT, "photostream-js"])).unwrap() {
        let dir_entry = dir_entry.unwrap();
        let path = dir_entry.path();
        let filename = path.file_name().unwrap().to_str().unwrap();
        user_ids.push(filename.replace(".json", ""));
    }
    user_ids
}

fn spider_image_paths(
    downloader: &mut Downloader,
    for_user: &str,
    max_pages: usize,
) -> SResult<Vec<String>> {
    for page in 1..=max_pages {
        let content = downloader.fetch(DownType::Photostream, for_user, &page.to_string())?;
        info!("loaded content of {}", content.body.len());

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
    book: &mut BookRoot,
) -> SResult<()> {
    for image_path in image_paths {
        let image_id = extract_image_id_from_livestatic(image_path);

        let image_page = downloader.fetch(DownType::ImageSizes, for_user, image_id)?;
        let original_image_url = match extract_original_size_url(image_page.body) {
            Err(SError::BadPage(_page, _)) => {
                error!("no page found");
                continue;
            }
            r => r,
        }?;
        let image_orig = downloader.fetch(DownType::ImageOrig, for_user, &original_image_url)?;

        // downloader.fetch(DownType::ImageViewer, for_user, image_id)?;

        book.push_image(
            for_user,
            image_orig
                .output_path
                .strip_prefix(IMAGE_DB_ROOT)
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap(),
            "some description".into(),
        );
    }

    Ok(())
}

fn init_logging() {
    let default_env = "trace,\
    reqwest::blocking::wait=DEBUG,\
    reqwest::blocking::client=DEBUG,\
    hyper_util::client::legacy::pool=DEBUG,\
    selectors::matching=INFO,\
    reqwest::connect=DEBUG,\
    hyper_util::client::legacy::client=DEBUG,\
    html5ever=INFO";
    // let default_env = "trace";

    let subscriber = Registry::default();

    let env_var = env::var(EnvFilter::DEFAULT_ENV).unwrap_or_else(|_| default_env.into());
    let env_layer = EnvFilter::builder().parse(env_var).expect("bad env");
    let subscriber = subscriber.with(env_layer);

    let filter_layer = Layer::default().compact();
    let subscriber = subscriber.with(filter_layer);

    subscriber.init()
}
