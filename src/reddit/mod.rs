mod json;

use self::json::{post, subreddit};
use crate::ToTextFrames;

const MAX_TEXT_LENGTH: usize = 300;
const MAX_ITEM_COUNT: usize = 1000;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/113.0.5666.197 Safari/537.36";

#[derive(Clone)]
pub struct Post {
    pub title: String,
    pub body: String,
    pub permalink: String,
}

impl ToTextFrames for Post {
    fn to_text_frames(self) -> Vec<String> {
        vec![self.title, self.body]
    }
}

#[derive(Clone)]
pub struct Comment {
    pub body: String,
}

impl ToTextFrames for Comment {
    fn to_text_frames(self) -> Vec<String> {
        vec![self.body]
    }
}

pub fn get_posts(subreddit: &str, sort: &str, time: &str) -> Result<Vec<Post>, reqwest::Error> {
    let count = 100;

    let url = format!("https://reddit.com/r/{subreddit}/{sort}.json?t={time}&count={count}");

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to build request client");

    let response = client.get(&url).send()?;

    let json = response.text()?;

    let subreddit: subreddit::Response = serde_json::from_str(&json).expect("Failed to parse json");

    let mut posts = Vec::new();

    for child in subreddit.data.children {
        let subreddit::ChildData {
            title,
            selftext,
            permalink,
        } = child.data;

        if title.len().max(selftext.len()) > MAX_TEXT_LENGTH {
            continue;
        }

        posts.push(Post {
            title,
            body: selftext,
            permalink,
        });

        if posts.len() >= MAX_ITEM_COUNT {
            break;
        }
    }

    Ok(posts)
}

pub fn get_comments(post_permalink: &str) -> Result<Vec<Comment>, reqwest::Error> {
    let url = format!("https://reddit.com/{}.json", post_permalink);

    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .expect("Failed to build request client");

    let response = client.get(&url).send()?;

    let json = response.text()?;

    let post: post::Response = serde_json::from_str(&json).expect("Failed to parse json");

    let mut comments = Vec::new();

    for child in post.1.data.children {
        let body = child.data.body;

        let Some(body) = body else {
            println!("[info] comment has no body");
            continue;
        };

        if body.len() > MAX_TEXT_LENGTH {
            continue;
        }

        comments.push(Comment { body });

        if comments.len() >= MAX_ITEM_COUNT {
            break;
        }
    }

    Ok(comments)
}
