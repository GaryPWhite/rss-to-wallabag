use reqwest::Error;
use serde;
use serde::{Deserialize, Serialize};

// TODO: what does the RSS / Atom feed look like?
// How can we pull data from either/both, and accomodate for fulltext versions?
// can we detect fulltext versions ???

// content, title & url are required for full-text.
// We should also populate preview_picture, tags, published_at, authors, and origin_url

#[derive(Serialize, Deserialize)]
pub(crate) struct RSSRoot {
    pub channel: RSSFeed
}

#[derive(Serialize, Deserialize)]
pub(crate) struct RSSFeed {
    #[serde(rename = "item")]
    pub items: Vec<RSSItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct RSSItem {
    // required in ATOM and RSS Feeds
    pub title: Option<String>,
    pub link: String,
    // sometimes-provided fields
    pub description: Option<String>,
    #[serde(rename = "pubDate")]
    pub pub_date: Option<String>,
    pub author: Option<String>,
    #[serde(rename = "dc:creator")]
    pub creator: Option<String>,
    #[serde(rename = "content:encoded")]
    pub content: Option<String>,
}

pub(crate) async fn get_feed_items(url: &String) -> Result<Vec<RSSItem>, Error> {
    // fetch from URL, should generate an RSS file we parse then return `item`s from.
    match (
        match reqwest::get(url).await {
            Ok(response) => response,
            Err(e) => return Err(e)
        }
    ).text().await {
        Ok(body) => {
            let _doc: RSSRoot = quick_xml::de::from_str(&body).expect(format!("failed to parse xml from url {:?}", url).as_str());
            Ok(_doc.channel.items)
        }
        Err(e) => return Err(e)
    }
}
