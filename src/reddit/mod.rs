mod json;

use self::json::{ChildData, Subreddit};
use crate::config::RedditConfig;

const MAX_TEXT_LENGTH: usize = 100;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.5666.197 Safari/537.36";

pub fn get_posts(config: RedditConfig) -> Result<Vec<String>, reqwest::Error> {
    let RedditConfig {
        subreddit,
        sort,
        time,
    } = config;

    let count = 100;

    let url = format!("https://reddit.com/r/{subreddit}/{sort}.json?t={time}&count={count}");

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to build request client");

    let response = client.get(&url).send()?;

    let json = response.text()?;

    let subreddit: Subreddit = serde_json::from_str(&json).expect("Failed to parse json");

    let mut texts = Vec::new();
    for child in subreddit.data.children {
        let ChildData { title, selftext } = child.data;

        if title.len().max(selftext.len()) > MAX_TEXT_LENGTH {
            continue;
        }

        texts.push(title);
        texts.push(selftext);

        if texts.len() >= 4 {
            //    break;
        }
    }

    Ok(texts)
}
