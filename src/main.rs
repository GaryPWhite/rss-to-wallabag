use crate::config::Feed;
use crate::config::RSSConfig;
use crate::config::WallabagConfig;
use crate::wallabag::make_wallabag_client;

mod config;
mod rss;
mod wallabag;

#[tokio::main]
async fn main() {
    let config_from_file: RSSConfig = config::read_from_file().unwrap_or_else(|error| {
        panic!("Could not read file {error:?}");
    });
    let wallabag_config: WallabagConfig = config_from_file.wallabag;
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
    // make client for shared requests
    let w_client = make_wallabag_client(wallabag_config.clone())
        .await
        .expect("Wallabag Client was not instantiated.");
    for feed in feeds.iter() {
        // fetch the contents of the rss feed (https://en.wikipedia.org/wiki/RSS)
        let items = rss::get_feed_items(&feed.url).await.unwrap_or_else(|f| {
            println!("could not fetch feeds: {:?}", f);
            Vec::default()
        });
        // read max per feed or
        for item in items.into_iter().take(max_per_feed as usize) {
            let link = item.link.clone();
            println!("writing {link} to wallabag");
            println!("{:?}", item);
            // match wallabag::add_item(wallabag_config.clone(), &w_client, item)
            //     .await
            //     .expect("failed to write to wallabag")
            //     .error_for_status()
            // {
            //     Ok(res) => {
            //         let status = res.status();
            //         println!("wrote {link} to wallabag with return status {status}");
            //     }
            //     Err(e) => println!("failed to write {link} to wallabag, error: {e}"),
            // }
        }
    }
}
