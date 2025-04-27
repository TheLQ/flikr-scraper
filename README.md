# Flikr Scraper

Highly specific scraper for Flickr original images.
Javascript gallery scanner, Rust download and cache management.

Comments and other data are out of scope

## Guide

1. Setup [warcprox](https://github.com/internetarchive/warcprox) for future upload to your own Internet Archive
2. For each user, open their photostream page in the browser
3. Run `src/extract_v2.js` in developer console
4. On "== EXTRACTED IMAGES ==" message, right click > Copy Object, paste into `image-db/photostream-js/$user_id.json`
5. Run scraper. Be patient, downloading is throttled to be a nice scraper

## Output

* A basic image viewer of all images
* JSON dump with user, url, title, and description
* All scraped HTML pages
