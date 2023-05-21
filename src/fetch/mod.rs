mod json;

use json::Subreddit;

use self::json::ChildData;

const MAX_TEXT_LENGTH: usize = 100;

pub fn fetch_texts() -> Result<Vec<String>, reqwest::Error> {
    let subreddit = "dadjokes";
    let sort = "top";
    let time = "week";
    let count = 100;

    let user_agent = "Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.5666.197 Safari/537.36";

    let url = format!("https://reddit.com/r/{subreddit}/{sort}.json?t={time}&count={count}");

    let client = reqwest::blocking::Client::builder()
        .user_agent(user_agent)
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
