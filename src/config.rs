use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{default::Default, fs};

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct RSSConfig {
    pub wallabag: WallabagConfig,
    pub feeds: Vec<Feed>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct WallabagConfig {
    pub url: String,
    pub clientid: String,
    pub clientsecret: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub(crate) struct Feed {
    pub url: String,
    pub tags: Vec<String>,
    // can be "ghost-fulltext" or defaults to "rss"
    pub flavor: Option<String>,
    // a cookie string to send with any GET requests for RSS feeds
    // see https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Cookies#creating_removing_and_updating_cookies
    pub cookies: Option<String>
}

pub(crate) fn read_from_file() -> Result<RSSConfig> {
    let path =
        std::env::var("RSS_WALLABAG_PATH").unwrap_or("./.rss-config.json".to_string());
    serde_json::from_str(
        fs::read_to_string(&path)
            .expect(format!("Failed to read Config file from RSS_WALLABAG_PATH <{}>", path).as_str())
            .as_str(),
    )
}
