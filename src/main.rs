use video_maker::{
    config::{Config, RedditConfig, VideoConfig, VoiceConfig},
    create_video,
};

fn main() {
    let config = Config {
        reddit: RedditConfig {
            subreddit: "askreddit".to_string(),
            sort: "top".to_string(),
            time: "week".to_string(),
        },
        voice: VoiceConfig {
            language: "en-GB".to_string(),
            gender: "male".to_string(),
            pitch: 0.5,
            rate: 0.5,
        },
        video: VideoConfig {},
    };

    create_video(config);
}
