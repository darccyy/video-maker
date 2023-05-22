use video_maker::reddit;
use video_maker::ToTextFrames;
use video_maker::{
    config::{Config, VideoConfig, VoiceConfig},
    create_video,
};

fn main() {
    let texts = get_reddit_texts();

    #[allow(unused_variables)]
    let config = Config {
        voice: VoiceConfig {
            language: "en-GB".to_string(),
            gender: "male".to_string(),
            pitch: 0.5,
            rate: 0.5,
        },
        video: VideoConfig {},
    };

    println!("Creating video...");

    create_video(texts);
}

fn get_reddit_texts() -> Vec<String> {
    let subreddit = inquire::Text::new("Subreddit:")
        .prompt()
        .expect("Error reading input");

    let sort_options = vec!["top", "hot", "new"];

    let sort = inquire::Select::new("Sort by:", sort_options)
        .prompt()
        .expect("Error reading input")
        .to_string();

    let time = if sort == "top" {
        let time_options = vec!["month", "day", "week", "month", "year", "all"];

        inquire::Select::new("Top of:", time_options)
            .prompt()
            .expect("Error reading input")
    } else {
        "all"
    }
    .to_string();

    println!("Fetching posts...");
    let posts = reddit::get_posts(&subreddit, &sort, &time).expect("Failed to fetch posts");

    let content_type = inquire::Select::new("Posts or comments?", vec!["posts", "comments"])
        .prompt()
        .expect("Error reading input");

    let texts: Vec<String> = if content_type == "posts" {
        posts.to_text_frames()
    } else {
        let post_title_options = posts.clone().into_iter().map(|post| post.title).collect();

        let post_title = inquire::Select::new("Comments of which post?", post_title_options)
            .prompt()
            .expect("Error reading input");

        let mut permalink = None;
        for post in posts {
            if post.title == post_title {
                permalink = Some(post.permalink);
                break;
            }
        }
        let permalink = permalink.expect("Post not found with that title???");

        println!("Fetching comments...");
        let comments = reddit::get_comments(&permalink).expect("Failed to fetch comments");

        comments.to_text_frames()
    };

    texts
}
