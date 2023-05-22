use video_maker::config::{Config, VideoConfig, VoiceConfig};
use video_maker::create_video;
use video_maker::reddit;
use video_maker::ToTextFrames;

fn main() {
    println!("====== VIDEO-MAKER ======");

    let texts = get_reddit_texts();

    println!("Creating video with {} text frames...", texts.len());

    let config = Config {
        voice: VoiceConfig {
            language: "en-GB".to_string(),
            gender: "male".to_string(),
            pitch: 0.5,
            rate: 0.55,
        },
        video: VideoConfig {},
    };

    create_video(texts, &config);
}

fn get_reddit_texts() -> Vec<String> {
    let subreddit = inquire::Text::new("Subreddit:")
        .with_default("AskReddit")
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

    let get_posts = || {
        println!("Fetching posts...");
        reddit::get_posts(&subreddit, &sort, &time).expect("Failed to fetch posts")
    };

    let content_type = inquire::Select::new("Posts or comments?", vec!["comments", "posts"])
        .prompt()
        .expect("Error reading input");

    let texts: Vec<String> = if content_type == "posts" {
        get_posts().to_text_frames()
    } else {
        let posts = get_posts();

        let post_title_options = posts.clone().into_iter().map(|post| post.title).collect();

        let post_title = inquire::Select::new(
            "Which post to take comments from? (scroll for more)",
            post_title_options,
        )
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

        let mut frames = vec![post_title];
        frames.append(&mut comments.to_text_frames());
        frames
    };

    println!("Text frames: {}", texts.len());

    let frames_limit = inquire::Text::new("Limit text frames count?")
        .prompt()
        .expect("Error reading input");

    if let Ok(frames_limit) = frames_limit.parse::<usize>() {
        texts.into_iter().take(frames_limit).collect()
    } else {
        texts
    }
}
