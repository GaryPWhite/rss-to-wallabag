use std::ops::Range;

use crate::config::Feed;
use crate::config::RSSConfig;
use crate::config::WallabagConfig;

mod config;
mod rss;

#[tokio::main]
async fn main() {
    let config_from_file: RSSConfig = config::read_from_file().unwrap_or_else(|error| {
        panic!("Could not read file {error:?}");
    });
    let wallabag: WallabagConfig = config_from_file.wallabag;
    let feeds: Vec<Feed> = config_from_file.feeds;
    let max_per_feed: i32 = std::env::var("RSS_WALLABAG_MAX_PER_FEED")
        .unwrap_or("10".to_string())
        .parse::<i32>()
        .unwrap_or_else(|f| {
            println!(
                "could not parse RSS_WALLABAG_MAX_PER_FEED; {:?} \nusing default (10)",
                f
            );
            10
        });
    for feed in feeds.iter() {
        // fetch the contents of the rss feed (https://en.wikipedia.org/wiki/RSS)
        let items = rss::get_feed_items(&feed.url)
            .await
            .unwrap_or_else(|f| {
                println!("could not fetch feeds: {:?}", f);
                Vec::default()
            });
        // read max per feed or
        let max_to_upload = std::cmp::min(max_per_feed, items.len() as i32);
        for _idx in (Range {
            start: 0,
            end: max_to_upload,
        }) {
            println!("TODO: write to wallabag and test it works up to here.")
        }
    }
    println!("Hello, feeds \n{feeds:?}\nand wallabag {wallabag:?}");
    println!("Hello, world!");
}
