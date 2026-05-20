use crate::{
    config::{self, WallabagConfig},
    rss::RSSItem,
};
use reqwest::{
    Client,
    header::{HeaderMap, HeaderValue},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct AccessToken {
    pub access_token: String,
}

pub(crate) async fn make_wallabag_client(
    wallabag_config: config::WallabagConfig,
) -> Result<Client, reqwest::Error> {
    // fetch api token from wallabag
    let url = wallabag_config.url + "/oauth/v2/token";
    let params = std::collections::HashMap::from([
        ("grant_type", "password"),
        ("client_id", &wallabag_config.clientid),
        ("client_secret", &wallabag_config.clientsecret),
        ("username", &wallabag_config.username),
        ("password", &wallabag_config.password),
    ]);
    let client = reqwest::Client::new();
    let token = match client.post(url).form(&params).send().await?.text().await {
        Ok(body) => {
            let parsed_token: AccessToken = serde_json::de::from_str(&body).expect(
                format!("could not parse access token from wallabag body: <{body}>").as_str(),
            );
            parsed_token.access_token
        }
        Err(e) => return Err(e),
    };
    let mut default_headers: HeaderMap = Default::default();
    let token_as_header_value = HeaderValue::from_str(format!("Bearer {}", token).as_str())
        .expect("Could not get token from wallabag instance, please check your credentials.");
    default_headers.insert(reqwest::header::AUTHORIZATION, token_as_header_value);
    println!("{:?}", default_headers);
    reqwest::Client::builder()
        .default_headers(default_headers)
        .build()
}

pub(crate) async fn add_item(
    w_config: WallabagConfig,
    client: &Client,
    item: RSSItem,
) -> Result<reqwest::Response, reqwest::Error> {
    // we bare-minimum need URL
    let mut params =
        std::collections::HashMap::from([("url", item.link.clone()), ("origin_url", item.link)]);
    // everything else is OPTIONal haha
    if let Some(value) = item.title {
        params.insert("title", value);
    }
    if let Some(value) = item.pub_date {
        params.insert("published_at", value);
    }
    if let Some(value) = item.author {
        params.insert("authors", value);
    }
    if let Some(value) = item.creator {
        params.insert("authors", value);
    }
    if let Some(value) = item.content {
        params.insert("content", value);
    }
    client
        .post(w_config.url + "/api/entries")
        .form(&params)
        .send()
        .await
}
