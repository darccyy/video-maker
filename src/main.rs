use std::fs;

use video_maker::clean_assets_output;
use video_maker::config;
use video_maker::create_video;
use video_maker::reddit;
use video_maker::ToTextFrames;

fn main() {
    println!("====== VIDEO-MAKER ======");

    // Clean and check assets directory
    clean_assets_output().expect("Failed to clean assets output");

    let config_filename = "./config.toml";

    let config = fs::read_to_string(config_filename).expect("Failed to read config file");
    let config = config::parse(&config);

    println!("{:#?}", config);

    println!("Fetching posts...");
    let posts = reddit::get_posts(&config.reddit).expect("Failed to fetch posts");

    let texts = if !config.reddit.comments {
        posts.to_text_frames()
    } else {
        choose_post_for_comments(&posts)
    };

    let texts = texts.into_iter().filter(|text| !text.is_empty());

    let texts = match config.reddit.limit {
        Some(limit) => texts.take(limit).collect(),
        None => texts.collect(),
    };

    println!("{:#?}", texts);

    create_video(texts, &config);
}

fn choose_post_for_comments(posts: &[reddit::Post]) -> Vec<String> {
    let post_title_options = posts.clone().into_iter().map(|post| &post.title).collect();

    let post_title = inquire::Select::new(
        "Which post to take comments from? (scroll for more)",
        post_title_options,
    )
    .with_page_size(12)
    .prompt()
    .expect("Error reading input");

    let mut permalink = None;
    for post in posts {
        if &post.title == post_title {
            permalink = Some(&post.permalink);
            break;
        }
    }
    let permalink = permalink.expect("Post not found with that title???");

    println!("Fetching comments...");
    let comments = reddit::get_comments(&permalink).expect("Failed to fetch comments");

    let mut texts = vec![post_title.clone()];
    texts.append(&mut comments.to_text_frames());
    texts
}
