use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Subreddit {
    pub data: SubredditData,
}

#[derive(Debug, Deserialize)]
pub struct SubredditData {
    pub children: Vec<Child>,
}

#[derive(Debug, Deserialize)]
pub struct Child {
    pub data: ChildData,
}

#[derive(Debug, Deserialize)]
pub struct ChildData {
    pub title: String,
    pub selftext: String,
}
