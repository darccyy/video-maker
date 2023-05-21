pub struct Config {
    pub reddit: RedditConfig,
    pub voice: VoiceConfig,
    pub video: VideoConfig,
}

pub struct RedditConfig {
    pub subreddit: String,
    pub sort: String,
    pub time: String,
}

pub struct VoiceConfig {
    pub language: String,
    pub gender: String,
    pub pitch: f32,
    pub rate: f32,
}

pub struct VideoConfig {}
